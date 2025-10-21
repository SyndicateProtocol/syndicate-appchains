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

interface SyndicateStorageTest {
    event EpochRangeProcessed(uint256 indexed startEpochNumber, uint256 indexed endEpochNumber);
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
    function appchainId() external view returns (uint256);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function manager() external view returns (address);
    function nonAuthorized() external view returns (address);
    function setUp() external;
    function syndicateStorage() external view returns (address);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testConstructor() external view;
    function testConstructorZeroAdmin() external;
    function testConstructorZeroAppChainId() external;
    function testConstructorZeroManager() external;
    function testRoleManagement() external;
    function testSave() external;
    function testSaveForMany() external;
    function testSaveForManyArrayLengthMismatch1() external;
    function testSaveForManyArrayLengthMismatch2() external;
    function testSaveForManyEmptyArrays() external;
    function testSaveForManyNonManager() external;
    function testSaveForManyWithSingleEpoch() external;
    function testSaveNonManager() external;
    function testSetIndexFromBlock() external;
    function testSetIndexFromBlockNonManager() external;
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
    "name": "appchainId",
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
    "name": "manager",
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
    "name": "nonAuthorized",
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
    "name": "syndicateStorage",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract SyndicateStorage"
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
    "name": "testConstructor",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testConstructorZeroAdmin",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testConstructorZeroAppChainId",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testConstructorZeroManager",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testRoleManagement",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSave",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSaveForMany",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSaveForManyArrayLengthMismatch1",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSaveForManyArrayLengthMismatch2",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSaveForManyEmptyArrays",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSaveForManyNonManager",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSaveForManyWithSingleEpoch",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSaveNonManager",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSetIndexFromBlock",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSetIndexFromBlockNonManager",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "EpochRangeProcessed",
    "inputs": [
      {
        "name": "startEpochNumber",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "endEpochNumber",
        "type": "uint256",
        "indexed": true,
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
pub mod SyndicateStorageTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234602f57600160ff19600c541617600c55600160ff19601f541617601f55614a7c90816100348239f35b5f80fdfe60806040526004361015610011575f80fd5b5f5f3560e01c806305a56f80146132fd5780630a9254e4146131a05780631ddc2d10146131795780631ed7831c146130fb578063297feeb2146130d15780632ade388014612edd5780632c41ee6714612b665780633b7b325b146128015780633e5e3c23146127835780633f7286f414612705578063481c6a75146126de57806351f5287a146124d257806353882c6e1461211f57806359ebf0491461200457806366d9a9a014611ec757806385226c8114611e3d5780638d3a83b914611d225780638d3d6c991461180757806390c9677014611511578063916a17c614611467578063965140251461130a57806396e181cf146110c4578063ac29af3314610e01578063b0464fdc14610d57578063b5508aa914610ccd578063ba414fa614610ca8578063c0ce94ad146108a0578063c2e9f2e4146104a8578063ca505fe21461027c578063d87813421461025e578063e20c9f71146101d0578063f851a440146101aa5763fa7626d414610185575f80fd5b346101a757806003193601126101a757602060ff601f54166040519015158152f35b80fd5b50346101a757806003193601126101a75760206001600160a01b03815416604051908152f35b50346101a757806003193601126101a75760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b81811061023f5761023b8561022f8187038261380a565b6040519182918261361e565b0390f35b82546001600160a01b0316845260209093019260019283019201610218565b50346101a757806003193601126101a7576020602354604051908152f35b50346101a757806003193601126101a75760209060405161029d838261380a565b8181525f368137604051906102b2848361380a565b8282525f3681376102c28361382d565b6102cf604051918261380a565b838152601f196102de8561382d565b01845b8181106104995750508394506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561045b57604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610479578591610484575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474576040517ff4844814000000000000000000000000000000000000000000000000000000008152848160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561047957859161045f575b50506001600160a01b03601f5460081c16803b1561045b5761042e93858094604051968795869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b03925af180156104505761043f5750f35b816104499161380a565b6101a75780f35b6040513d84823e3d90fd5b8480fd5b816104699161380a565b61047457835f6103d9565b505050fd5b6040513d87823e3d90fd5b8161048e9161380a565b61047457835f61036b565b606083820188015286016102e1565b50346101a757806003193601126101a757806001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c1578391610868575b50602080546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101939093526001600160a01b0316602483015281604481855afa80156107c157610565918491610801575b50613e79565b6040517fec87621c000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c1578391610830575b506021546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b03166024820152602081604481855afa80156107c1576106019184916108015750613e79565b6040517fd8781342000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c15783916107cc575b50602354737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561047457604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156107c15783916107ac575b50506020600491604051928380927fcce5d2f60000000000000000000000000000000000000000000000000000000082525afa908115610450578291610773575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201528160248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156104505761043f5750f35b50fd5b9150506020813d6020116107a4575b8161078f6020938361380a565b810103126107a0578190515f6106f9565b5f80fd5b3d9150610782565b816107b69161380a565b61077057815f6106b8565b6040513d85823e3d90fd5b9250506020823d6020116107f9575b816107e86020938361380a565b810103126107a0578291515f61063e565b3d91506107db565b610823915060203d602011610829575b61081b818361380a565b810190613d88565b5f61055f565b503d610811565b9250506020823d602011610860575b8161084c6020938361380a565b810103126107a057905182919060206105a2565b3d915061083f565b9250506020823d602011610898575b816108846020938361380a565b810103126107a05790518291906020610506565b3d9150610877565b50346101a757806003193601126101a75780604080516108c0828261380a565b60018152601f1982019081366020830137600a6108dc82613845565b528251916108ea848461380a565b600183523660208401377f9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb65861091e83613845565b526109276138d9565b8351602080820152600a60408201527f7465737420626174636800000000000000000000000000000000000000000000606082015261097381608081015b03601f19810183528261380a565b61097c82613845565b5261098681613845565b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c2d578451907f06447d560000000000000000000000000000000000000000000000000000000082526004820152858160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c4657908691610c93575b505060046001600160a01b036022541660206001600160a01b03601f5460081c168751938480927fec87621c0000000000000000000000000000000000000000000000000000000082525afa918215610c89578792610c50575b5085517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b0390911660248201526044810191909152610aac8160648101610965565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c2d5785610b06918651809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190613660565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c4657908691610c31575b50506001600160a01b03601f5460081c16803b15610c2d57610b82938680948751968795869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b03925af18015610c2157908391610c0c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610770578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c03575061043f5750f35b513d84823e3d90fd5b81610c169161380a565b61077057815f610b94565b505051903d90823e3d90fd5b8580fd5b81610c3b9161380a565b61045b57845f610b2e565b85513d88823e3d90fd5b965090506020863d602011610c81575b81610c6d6020938361380a565b810103126107a05794518695610aac610a5e565b3d9150610c60565b86513d89823e3d90fd5b81610c9d9161380a565b61045b57845f610a04565b50346101a757806003193601126101a7576020610cc3613da0565b6040519015158152f35b50346101a757806003193601126101a757601954610cea8161382d565b91610cf8604051938461380a565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310610d3a576040518061023b87826136da565b600160208192610d49856139e1565b815201920192019190610d25565b50346101a757806003193601126101a757601c54610d748161382d565b91610d82604051938461380a565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610dc4576040518061023b8782613757565b60026020600192604051610dd7816137ee565b6001600160a01b038654168152610def858701613ae4565b83820152815201920192019190610daf565b50346101a757806003193601126101a757806040805190610e22818361380a565b60018252601f1981019182366020830137600a610e3e82613845565b52815192610e4c838561380a565b600184523660208501377f9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb658610e8084613845565b52610e896138d9565b8251602080820152600a60408201527f74657374206261746368000000000000000000000000000000000000000000006060820152610ecb8160808101610965565b610ed482613845565b52610ede81613845565b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561045b5782517f491cc7c2000000000000000000000000000000000000000000000000000000008152858180610f4660048201906001606060808401938281528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156110a1579086916110af575b5050610f7982613845565b51610f8383613845565b518451917ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f38880a36001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156110ab577fca669fa70000000000000000000000000000000000000000000000000000000082526004820152858160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156110a15790869161108c575b50506001600160a01b03601f5460081c16803b15610c2d57611079948680948651978895869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b03925af1908115610c03575061043f5750f35b816110969161380a565b61045b57845f611025565b84513d88823e3d90fd5b8680fd5b816110b99161380a565b61045b57845f610f6e565b50346101a757806003193601126101a7578060405161111c816109656020820160609060208152600a60208201527f746573742062617463680000000000000000000000000000000000000000000060408201520190565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610770576040517f491cc7c200000000000000000000000000000000000000000000000000000000815282818061118460048201906001606060808401938281528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156107c15783916112f5575b5050604051600a807ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f38580a36001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474577fca669fa70000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156107c15783916112e0575b50506001600160a01b03601f5460081c16803b156112dc5761042e83929183926040519485809481937fb1d06045000000000000000000000000000000000000000000000000000000008352600a60048401527f9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb6586024840152606060448401526064830190613660565b5050fd5b816112ea9161380a565b61077057815f611252565b816112ff9161380a565b61077057815f6111ac565b50346101a757806003193601126101a757737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f41707020636861696e2049442063616e6e6f74206265203000000000000000006044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561045057611452575b50506001600160a01b03602054166001600160a01b03602154169060405191610b86918284019284841067ffffffffffffffff8511176114255791606093918593613ef685398252602082015284604082015203019082f0156114195780f35b604051903d90823e3d90fd5b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b8161145c9161380a565b6101a757805f6113b9565b50346101a757806003193601126101a757601d546114848161382d565b91611492604051938461380a565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b8383106114d4576040518061023b8782613757565b600260206001926040516114e7816137ee565b6001600160a01b0386541681526114ff858701613ae4565b838201528152019201920191906114bf565b50346101a757806003193601126101a757806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610450576117f2575b5060046001600160a01b036022541660206001600160a01b03601f5460081c16604051938480927fec87621c0000000000000000000000000000000000000000000000000000000082525afa9182156107c15783926117b9575b506040517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b03909116602482015260448101919091526116478160648101610965565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057816116a291604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190613660565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610450576117a4575b506001600160a01b03601f5460081c16803b15610770578180916024604051809481937f0ffd1380000000000000000000000000000000000000000000000000000000008352606460048401525af180156104505761178f575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104505761043f5750f35b816117999161380a565b6101a757805f611721565b816117ae9161380a565b6101a757805f6116c7565b925090506020823d6020116117ea575b816117d66020938361380a565b810103126107a057905182916116476115f8565b3d91506117c9565b816117fc9161380a565b6101a757805f61159e565b50346101a757806003193601126101a757806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561045057611d0d575b506001600160a01b03601f5460081c16604051907fec87621c000000000000000000000000000000000000000000000000000000008252602082600481845afa9182156107c1578392611cd6575b50803b156112dc578291829160405180948180947f2f2ff15d0000000000000000000000000000000000000000000000000000000082526004602081840160408101938152015203925af1801561045057611cc1575b506001600160a01b03601f5460081c166040517fec87621c000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c1578391611c8c575b5060206040518080937f91d148540000000000000000000000000000000000000000000000000000000082526004848184016040810193815201520381855afa80156107c1576119db9184916108015750613e79565b604051907fec87621c000000000000000000000000000000000000000000000000000000008252602082600481845afa9182156107c1578392611c55575b50803b156112dc578291829160405180948180947fd547741f0000000000000000000000000000000000000000000000000000000082526004602081840160408101938152015203925af1801561045057611c40575b506001600160a01b03601f5460081c166040517fec87621c000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c1578391611c09575b5060209060405192838080937f91d1485400000000000000000000000000000000000000000000000000000000825260048681840160408101938152015203915afa908115610450578291611bea575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156104505761178f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104505761043f5750f35b611c03915060203d6020116108295761081b818361380a565b5f611b0c565b9250506020823d602011611c38575b81611c256020938361380a565b810103126107a057602083925190611abc565b3d9150611c18565b81611c4a9161380a565b6101a757805f611a6f565b925090506020823d602011611c84575b81611c726020938361380a565b810103126107a057829151905f611a19565b3d9150611c65565b9250506020823d602011611cb9575b81611ca86020938361380a565b810103126107a0578291515f611985565b3d9150611c9b565b81611ccb9161380a565b6101a757805f611938565b925090506020823d602011611d05575b81611cf36020938361380a565b810103126107a057829151905f6118e2565b3d9150611ce6565b81611d179161380a565b6101a757805f611894565b50346101a757806003193601126101a757737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601960248201527f41646d696e20616464726573732063616e6e6f742062652030000000000000006044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561045057611e28575b50506001600160a01b03602154166023549060405191610b86918284019284841067ffffffffffffffff8511176114255791606093918593613ef685398683526020830152604082015203019082f0156114195780f35b81611e329161380a565b6101a757805f611dd1565b50346101a757806003193601126101a757601a54611e5a8161382d565b91611e68604051938461380a565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310611eaa576040518061023b87826136da565b600160208192611eb9856139e1565b815201920192019190611e95565b50346101a757806003193601126101a757601b54611ee48161382d565b611ef1604051918261380a565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310611fc957868587604051928392602084019060208552518091526040840160408260051b8601019392905b828210611f5e57505050500390f35b91936020611fb9827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083611fa98351604084526040840190613660565b9201519084818403910152613685565b9601920192018594939192611f4f565b60026020600192604051611fdc816137ee565b611fe5866139e1565b8152611ff2858701613ae4565b83820152815201920192019190611f21565b50346101a757806003193601126101a757737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f4d616e6167657220616464726573732063616e6e6f74206265203000000000006044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104505761210a575b50506001600160a01b03602054166023549060405191610b86918284019284841067ffffffffffffffff8511176114255791606093918593613ef685398252856020830152604082015203019082f0156114195780f35b816121149161380a565b6101a757805f6120b3565b50346101a757806003193601126101a7578060405161213f60808261380a565b6003815260609081366020830137600a61215882613845565b52600b6121648261387f565b52600c6121708261388f565b526040519161218060808461380a565b600383523660208401377f6d255fc3390ee6b41191da315958b7d6a1e5b17904cc7683558f98acc57977b46121b483613845565b527f4da432f1ecd4c0ac028ebde3a3f78510a21d54087b161590a63080d33b702b8d6121df8361387f565b527f204558076efb2042ebc9b034aab36d85d672d8ac1fa809288da5b453a4714aae61220a8361388f565b5261221361389f565b604051612259816109656020820160609060208152600c60208201527f746573742062617463682031000000000000000000000000000000000000000060408201520190565b61226282613845565b5261226c81613845565b506040516122b3816109656020820160609060208152600c60208201527f746573742062617463682032000000000000000000000000000000000000000060408201520190565b6122bc8261387f565b526122c68161387f565b5060405161230d816109656020820160609060208152600c60208201527f746573742062617463682033000000000000000000000000000000000000000060408201520190565b6123168261388f565b526123208161388f565b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474576040517f491cc7c200000000000000000000000000000000000000000000000000000000815284818061238960048201906001606060808401938281528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104795785916124bd575b50506123bc82613845565b516123c68361388f565b51604051917ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f38780a36001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c2d577fca669fa70000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561047957859161045f5750506001600160a01b03601f5460081c16803b1561045b5761042e93858094604051968795869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b816124c79161380a565b61047457835f6123b1565b50346101a757806003193601126101a757806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610450576126c9575b506001600160a01b03601f5460081c16803b15610770578180916024604051809481937f0ffd1380000000000000000000000000000000000000000000000000000000008352606460048401525af18015610450576126b4575b50600460206001600160a01b03601f5460081c16604051928380927fcce5d2f60000000000000000000000000000000000000000000000000000000082525afa90811561045057829161267f575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152606460248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156104505761043f5750f35b9150506020813d6020116126ac575b8161269b6020938361380a565b810103126107a0578190515f612607565b3d915061268e565b816126be9161380a565b6101a757805f6125b9565b816126d39161380a565b6101a757805f61255f565b50346101a757806003193601126101a75760206001600160a01b0360215416604051908152f35b50346101a757806003193601126101a75760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b8181106127645761023b8561022f8187038261380a565b82546001600160a01b031684526020909301926001928301920161274d565b50346101a757806003193601126101a75760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b8181106127e25761023b8561022f8187038261380a565b82546001600160a01b03168452602090930192600192830192016127cb565b50346101a757806003193601126101a7578060405161282160808261380a565b600381526060366020830137600a61283882613845565b52600b6128448261387f565b52600c6128508261388f565b526040519061286060608361380a565b6002825260403660208401377f6d255fc3390ee6b41191da315958b7d6a1e5b17904cc7683558f98acc57977b461289683613845565b527f4da432f1ecd4c0ac028ebde3a3f78510a21d54087b161590a63080d33b702b8d6128c18361387f565b526128ca61389f565b604051612910816109656020820160609060208152600c60208201527f746573742062617463682031000000000000000000000000000000000000000060408201520190565b61291982613845565b5261292381613845565b5060405161296a816109656020820160609060208152600c60208201527f746573742062617463682032000000000000000000000000000000000000000060408201520190565b6129738261387f565b5261297d8161387f565b506040516129c4816109656020820160609060208152600c60208201527f746573742062617463682033000000000000000000000000000000000000000060408201520190565b6129cd8261388f565b526129d78161388f565b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561045b57604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610479578591612b51575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f4172726179206c656e67746873206d75737420626520657175616c00000000006044820152848180606481015b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561047957859161045f5750506001600160a01b03601f5460081c16803b1561045b5761042e93858094604051968795869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b81612b5b9161380a565b61047457835f612a56565b50346101a757806003193601126101a75780604051612bbe816109656020820160609060208152600a60208201527f746573742062617463680000000000000000000000000000000000000000000060408201520190565b6001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156112dc57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156107c1578391612ec8575b505060046001600160a01b036022541660206001600160a01b03601f5460081c16604051938480927fec87621c0000000000000000000000000000000000000000000000000000000082525afa918215612ebd578492612e84575b506040517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b0390911660248201526044810191909152612ce68160648101610965565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156112dc5782612d4191604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190613660565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156107c1578391612e6f575b50506001600160a01b03601f5460081c16803b156112dc57612df383929183926040519485809481937fb1d06045000000000000000000000000000000000000000000000000000000008352600a60048401527f9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb6586024840152606060448401526064830190613660565b03925af180156104505761178f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104505761043f5750f35b81612e799161380a565b61077057815f612d69565b935090506020833d602011612eb5575b81612ea16020938361380a565b810103126107a05791518392612ce6612c97565b3d9150612e94565b6040513d86823e3d90fd5b81612ed29161380a565b61077057815f612c3c565b50346101a757806003193601126101a757601e54612efa8161382d565b612f07604051918261380a565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106130485786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310612f735786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110612fff57505050505060208060019297019301930190928695949293612f66565b909192939460208061303b837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951613660565b9701950193929101612fdb565b604051613054816137ee565b6001600160a01b0383541681526001830180546130708161382d565b9161307e604051938461380a565b8183528a526020808b20908b9084015b8382106130b4575050505060019282602092836002950152815201920192019190612f37565b6001602081926130c3866139e1565b81520193019101909161308e565b50346101a757806003193601126101a75760206001600160a01b03601f5460081c16604051908152f35b50346101a757806003193601126101a75760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061315a5761023b8561022f8187038261380a565b82546001600160a01b0316845260209093019260019283019201613143565b50346101a757806003193601126101a75760206001600160a01b0360225416604051908152f35b50346101a757806003193601126101a75760017fffffffffffffffffffffffff0000000000000000000000000000000000000000602054161760205560027fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215560037fffffffffffffffffffffffff00000000000000000000000000000000000000006022541617602255607b602355604051610b868082019082821067ffffffffffffffff8311176132d0576060918391613ef683396001815260026020820152607b604082015203019082f080156132c3577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f5580f35b50604051903d90823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346107a0575f6003193601126107a0576040519061331d60808361380a565b60038252606080366020850137600a61333584613845565b52600b6133418461387f565b52600c61334d8461388f565b526040519061335d60808361380a565b600382523660208301377f6d255fc3390ee6b41191da315958b7d6a1e5b17904cc7683558f98acc57977b461339182613845565b527f4da432f1ecd4c0ac028ebde3a3f78510a21d54087b161590a63080d33b702b8d6133bc8261387f565b527f204558076efb2042ebc9b034aab36d85d672d8ac1fa809288da5b453a4714aae6133e78261388f565b526040519260606133f8818661380a565b60028552601f19015f5b8181106135e0575050604051613451816109656020820160609060208152600c60208201527f746573742062617463682031000000000000000000000000000000000000000060408201520190565b61345a85613845565b5261346484613845565b506040516134ab816109656020820160609060208152600c60208201527f746573742062617463682032000000000000000000000000000000000000000060408201520190565b6134b48561387f565b526134be8461387f565b506001600160a01b036021541693737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107a057604051947fca669fa700000000000000000000000000000000000000000000000000000000865260048601525f8560248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156135d5576135bf575b839450737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f4172726179206c656e67746873206d75737420626520657175616c0000000000604482015284818060648101612ad5565b9250925f6135cc9161380a565b5f91839061353b565b6040513d5f823e3d90fd5b806060602080938901015201613402565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60206040818301928281528451809452019201905f5b8181106136415750505090565b82516001600160a01b0316845260209384019390920191600101613634565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b8181106136a25750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101613695565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061370c57505050505090565b9091929394602080613748837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951613660565b970193019301919392906136fd565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061378957505050505090565b90919293946020806137df837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b03815116845201519181858201520190613685565b9701930193019193929061377a565b6040810190811067ffffffffffffffff8211176135f157604052565b90601f601f19910116810190811067ffffffffffffffff8211176135f157604052565b67ffffffffffffffff81116135f15760051b60200190565b8051156138525760200190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b8051600110156138525760400190565b8051600210156138525760600190565b604051608091906138b0838261380a565b6003815291601f1901825f5b8281106138c857505050565b8060606020809385010152016138bc565b604080519091906138ea838261380a565b6001815291601f1901825f5b82811061390257505050565b8060606020809385010152016138f6565b90606082016060835281518091526020608084019201905f5b8181106139cb575050508181036020830152602080845192838152019301905f5b8181106139b5575050506040818303910152815180825260208201916020808360051b8301019401925f915b83831061398857505050505090565b90919293946020806139a683601f1986600196030187528951613660565b97019301930191939290613979565b825185526020948501949092019160010161394d565b825184526020938401939092019160010161392c565b90604051915f8154908160011c9260018316928315613ada575b602085108414613aad578487528693908115613a6d5750600114613a29575b50613a279250038361380a565b565b90505f9291925260205f20905f915b818310613a51575050906020613a27928201015f613a1a565b6020919350806001915483858901015201910190918492613a38565b60209350613a279592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f613a1a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f16936139fb565b90604051918281549182825260208201905f5260205f20925f905b806007830110613cfb57613a27945491818110613cc5575b818110613c8f575b818110613c59575b818110613c23575b818110613bed575b818110613bb7575b818110613b82575b10613b55575b50038361380a565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f613b4d565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301613b47565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301613b3f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301613b37565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301613b2f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301613b27565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301613b1f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301613b17565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391613aff565b908160209103126107a0575180151581036107a05790565b60085460ff168015613daf5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156135d5575f91613e47575b50151590565b90506020813d602011613e71575b81613e626020938361380a565b810103126107a057515f613e41565b3d9150613e55565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107a057604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156135d557613eeb5750565b5f613a279161380a56fe60a03461017a57601f610b8638819003918201601f19168301916001600160401b0383118484101761017e5780849260609460405283398101031261017a5761004781610192565b604061005560208401610192565b920151916001600160a01b03821615610135576001600160a01b038116156100f05782156100ab5761008961008f926101a6565b5061021c565b5060805260405161087690816102b08239608051816101000152f35b60405162461bcd60e51b815260206004820152601860248201527f41707020636861696e2049442063616e6e6f74206265203000000000000000006044820152606490fd5b60405162461bcd60e51b815260206004820152601b60248201527f4d616e6167657220616464726573732063616e6e6f74206265203000000000006044820152606490fd5b60405162461bcd60e51b815260206004820152601960248201527f41646d696e20616464726573732063616e6e6f742062652030000000000000006044820152606490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b038216820361017a57565b6001600160a01b0381165f9081525f516020610b665f395f51905f52602052604090205460ff16610217576001600160a01b03165f8181525f516020610b665f395f51905f5260205260408120805460ff191660011790553391905f516020610b265f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f516020610b465f395f51905f52602052604090205460ff16610217576001600160a01b03165f8181525f516020610b465f395f51905f5260205260408120805460ff191660011790553391907f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b08905f516020610b265f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f3560e01c90816301ffc9a7146104ec57508063048420a7146103365780630ffd138014610315578063248a9ca3146102e35780632f2ff15d146102b457806336568abe1461025857806391d148541461020f578063a217fddf146101f5578063b1d0604514610176578063cce5d2f614610159578063d547741f14610123578063d8781342146100e95763ec87621c146100ab575f80fd5b346100e5575f6003193601126100e55760206040517f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b088152f35b5f80fd5b346100e5575f6003193601126100e55760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b346100e557610157610134366105bb565b9061015261014d825f525f602052600160405f20015490565b610676565b6107ae565b005b346100e5575f6003193601126100e5576020600154604051908152f35b346100e55760606003193601126100e55760043560443567ffffffffffffffff81116100e557366023820112156100e557806004013567ffffffffffffffff81116100e557369101602401116100e5576101ce6105ee565b807ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f35f80a3005b346100e5575f6003193601126100e55760206040515f8152f35b346100e55761021d366105bb565b905f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b346100e557610266366105bb565b3373ffffffffffffffffffffffffffffffffffffffff82160361028c57610157916107ae565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346100e5576101576102c5366105bb565b906102de61014d825f525f602052600160405f20015490565b6106dc565b346100e55760206003193601126100e557602061030d6004355f525f602052600160405f20015490565b604051908152f35b346100e55760206003193601126100e55761032e6105ee565b600435600155005b346100e55760606003193601126100e55760043567ffffffffffffffff81116100e55761036790369060040161058a565b60243567ffffffffffffffff81116100e55761038790369060040161058a565b905060443567ffffffffffffffff81116100e5576103a990369060040161058a565b90506103b36105ee565b81831491826104e2575b50501561048457801561042a577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908082116104575781101561042a5760051b81013590357ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f35f80a3005b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f4172726179206c656e67746873206d75737420626520657175616c00000000006044820152fd5b14905083806103bd565b346100e55760206003193601126100e557600435907fffffffff0000000000000000000000000000000000000000000000000000000082168092036100e557817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115610560575b5015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483610559565b9181601f840112156100e55782359167ffffffffffffffff83116100e5576020808501948460051b0101116100e557565b60031960409101126100e5576004359060243573ffffffffffffffffffffffffffffffffffffffff811681036100e55790565b335f9081527fe84508f2c7fa9c351146748b3025cb78b45df37d868e48c6a75102fecdeee645602052604090205460ff161561062657565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b0860245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156106ad5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f146107a857805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f146107a857805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0de84508f2c7fa9c351146748b3025cb78b45df37d868e48c6a75102fecdeee645ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`/W`\x01`\xFF\x19`\x0CT\x16\x17`\x0CU`\x01`\xFF\x19`\x1FT\x16\x17`\x1FUaJ|\x90\x81a\x004\x829\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x05\xA5o\x80\x14a2\xFDW\x80c\n\x92T\xE4\x14a1\xA0W\x80c\x1D\xDC-\x10\x14a1yW\x80c\x1E\xD7\x83\x1C\x14a0\xFBW\x80c)\x7F\xEE\xB2\x14a0\xD1W\x80c*\xDE8\x80\x14a.\xDDW\x80c,A\xEEg\x14a+fW\x80c;{2[\x14a(\x01W\x80c>^<#\x14a'\x83W\x80c?r\x86\xF4\x14a'\x05W\x80cH\x1Cju\x14a&\xDEW\x80cQ\xF5(z\x14a$\xD2W\x80cS\x88,n\x14a!\x1FW\x80cY\xEB\xF0I\x14a \x04W\x80cf\xD9\xA9\xA0\x14a\x1E\xC7W\x80c\x85\"l\x81\x14a\x1E=W\x80c\x8D:\x83\xB9\x14a\x1D\"W\x80c\x8D=l\x99\x14a\x18\x07W\x80c\x90\xC9gp\x14a\x15\x11W\x80c\x91j\x17\xC6\x14a\x14gW\x80c\x96Q@%\x14a\x13\nW\x80c\x96\xE1\x81\xCF\x14a\x10\xC4W\x80c\xAC)\xAF3\x14a\x0E\x01W\x80c\xB0FO\xDC\x14a\rWW\x80c\xB5P\x8A\xA9\x14a\x0C\xCDW\x80c\xBAAO\xA6\x14a\x0C\xA8W\x80c\xC0\xCE\x94\xAD\x14a\x08\xA0W\x80c\xC2\xE9\xF2\xE4\x14a\x04\xA8W\x80c\xCAP_\xE2\x14a\x02|W\x80c\xD8x\x13B\x14a\x02^W\x80c\xE2\x0C\x9Fq\x14a\x01\xD0W\x80c\xF8Q\xA4@\x14a\x01\xAAWc\xFAv&\xD4\x14a\x01\x85W_\x80\xFD[4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x02?Wa\x02;\x85a\x02/\x81\x87\x03\x82a8\nV[`@Q\x91\x82\x91\x82a6\x1EV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x02\x18V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `#T`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` \x90`@Qa\x02\x9D\x83\x82a8\nV[\x81\x81R_6\x817`@Q\x90a\x02\xB2\x84\x83a8\nV[\x82\x82R_6\x817a\x02\xC2\x83a8-V[a\x02\xCF`@Q\x91\x82a8\nV[\x83\x81R`\x1F\x19a\x02\xDE\x85a8-V[\x01\x84[\x81\x81\x10a\x04\x99WPP\x83\x94P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04[W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a\x04\x84W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a\x04_W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x04[Wa\x04.\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x03\x92Z\xF1\x80\x15a\x04PWa\x04?WP\xF3[\x81a\x04I\x91a8\nV[a\x01\xA7W\x80\xF3[`@Q=\x84\x82>=\x90\xFD[\x84\x80\xFD[\x81a\x04i\x91a8\nV[a\x04tW\x83_a\x03\xD9V[PPP\xFD[`@Q=\x87\x82>=\x90\xFD[\x81a\x04\x8E\x91a8\nV[a\x04tW\x83_a\x03kV[``\x83\x82\x01\x88\x01R\x86\x01a\x02\xE1V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x08hW[P` \x80T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R\x81`D\x81\x85Z\xFA\x80\x15a\x07\xC1Wa\x05e\x91\x84\x91a\x08\x01W[Pa>yV[`@Q\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x080W[P`!T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R` \x81`D\x81\x85Z\xFA\x80\x15a\x07\xC1Wa\x06\x01\x91\x84\x91a\x08\x01WPa>yV[`@Q\x7F\xD8x\x13B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x07\xCCW[P`#Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x07\xACW[PP` `\x04\x91`@Q\x92\x83\x80\x92\x7F\xCC\xE5\xD2\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04PW\x82\x91a\x07sW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04PWa\x04?WP\xF3[P\xFD[\x91PP` \x81=` \x11a\x07\xA4W[\x81a\x07\x8F` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x81\x90Q_a\x06\xF9V[_\x80\xFD[=\x91Pa\x07\x82V[\x81a\x07\xB6\x91a8\nV[a\x07pW\x81_a\x06\xB8V[`@Q=\x85\x82>=\x90\xFD[\x92PP` \x82=` \x11a\x07\xF9W[\x81a\x07\xE8` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x82\x91Q_a\x06>V[=\x91Pa\x07\xDBV[a\x08#\x91P` =` \x11a\x08)W[a\x08\x1B\x81\x83a8\nV[\x81\x01\x90a=\x88V[_a\x05_V[P=a\x08\x11V[\x92PP` \x82=` \x11a\x08`W[\x81a\x08L` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x90Q\x82\x91\x90` a\x05\xA2V[=\x91Pa\x08?V[\x92PP` \x82=` \x11a\x08\x98W[\x81a\x08\x84` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x90Q\x82\x91\x90` a\x05\x06V[=\x91Pa\x08wV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@\x80Qa\x08\xC0\x82\x82a8\nV[`\x01\x81R`\x1F\x19\x82\x01\x90\x816` \x83\x017`\na\x08\xDC\x82a8EV[R\x82Q\x91a\x08\xEA\x84\x84a8\nV[`\x01\x83R6` \x84\x017\x7F\x9C\"\xFF_!\xF0\xB8\x1B\x11>c\xF7\xDBm\xA9O\xED\xEF\x11\xB2\x11\x9B@\x88\xB8\x96d\xFB\x9A<\xB6Xa\t\x1E\x83a8EV[Ra\t'a8\xD9V[\x83Q` \x80\x82\x01R`\n`@\x82\x01R\x7Ftest batch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01Ra\ts\x81`\x80\x81\x01[\x03`\x1F\x19\x81\x01\x83R\x82a8\nV[a\t|\x82a8EV[Ra\t\x86\x81a8EV[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C-W\x84Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x85\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0CFW\x90\x86\x91a\x0C\x93W[PP`\x04`\x01`\x01`\xA0\x1B\x03`\"T\x16` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x87Q\x93\x84\x80\x92\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x0C\x89W\x87\x92a\x0CPW[P\x85Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Ra\n\xAC\x81`d\x81\x01a\teV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C-W\x85a\x0B\x06\x91\x86Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6`V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0CFW\x90\x86\x91a\x0C1W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C-Wa\x0B\x82\x93\x86\x80\x94\x87Q\x96\x87\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x03\x92Z\xF1\x80\x15a\x0C!W\x90\x83\x91a\x0C\x0CW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x03WPa\x04?WP\xF3[Q=\x84\x82>=\x90\xFD[\x81a\x0C\x16\x91a8\nV[a\x07pW\x81_a\x0B\x94V[PPQ\x90=\x90\x82>=\x90\xFD[\x85\x80\xFD[\x81a\x0C;\x91a8\nV[a\x04[W\x84_a\x0B.V[\x85Q=\x88\x82>=\x90\xFD[\x96P\x90P` \x86=` \x11a\x0C\x81W[\x81a\x0Cm` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x94Q\x86\x95a\n\xACa\n^V[=\x91Pa\x0C`V[\x86Q=\x89\x82>=\x90\xFD[\x81a\x0C\x9D\x91a8\nV[a\x04[W\x84_a\n\x04V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` a\x0C\xC3a=\xA0V[`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x19Ta\x0C\xEA\x81a8-V[\x91a\x0C\xF8`@Q\x93\x84a8\nV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\r:W`@Q\x80a\x02;\x87\x82a6\xDAV[`\x01` \x81\x92a\rI\x85a9\xE1V[\x81R\x01\x92\x01\x92\x01\x91\x90a\r%V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1CTa\rt\x81a8-V[\x91a\r\x82`@Q\x93\x84a8\nV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\r\xC4W`@Q\x80a\x02;\x87\x82a7WV[`\x02` `\x01\x92`@Qa\r\xD7\x81a7\xEEV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\r\xEF\x85\x87\x01a:\xE4V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\r\xAFV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@\x80Q\x90a\x0E\"\x81\x83a8\nV[`\x01\x82R`\x1F\x19\x81\x01\x91\x826` \x83\x017`\na\x0E>\x82a8EV[R\x81Q\x92a\x0EL\x83\x85a8\nV[`\x01\x84R6` \x85\x017\x7F\x9C\"\xFF_!\xF0\xB8\x1B\x11>c\xF7\xDBm\xA9O\xED\xEF\x11\xB2\x11\x9B@\x88\xB8\x96d\xFB\x9A<\xB6Xa\x0E\x80\x84a8EV[Ra\x0E\x89a8\xD9V[\x82Q` \x80\x82\x01R`\n`@\x82\x01R\x7Ftest batch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01Ra\x0E\xCB\x81`\x80\x81\x01a\teV[a\x0E\xD4\x82a8EV[Ra\x0E\xDE\x81a8EV[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04[W\x82Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x85\x81\x80a\x0FF`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x10\xA1W\x90\x86\x91a\x10\xAFW[PPa\x0Fy\x82a8EV[Qa\x0F\x83\x83a8EV[Q\x84Q\x91\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3\x88\x80\xA3`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x10\xABW\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x85\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x10\xA1W\x90\x86\x91a\x10\x8CW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C-Wa\x10y\x94\x86\x80\x94\x86Q\x97\x88\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x03\x92Z\xF1\x90\x81\x15a\x0C\x03WPa\x04?WP\xF3[\x81a\x10\x96\x91a8\nV[a\x04[W\x84_a\x10%V[\x84Q=\x88\x82>=\x90\xFD[\x86\x80\xFD[\x81a\x10\xB9\x91a8\nV[a\x04[W\x84_a\x0FnV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@Qa\x11\x1C\x81a\te` \x82\x01``\x90` \x81R`\n` \x82\x01R\x7Ftest batch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81\x80a\x11\x84`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07\xC1W\x83\x91a\x12\xF5W[PP`@Q`\n\x80\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3\x85\x80\xA3`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07\xC1W\x83\x91a\x12\xE0W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x12\xDCWa\x04.\x83\x92\x91\x83\x92`@Q\x94\x85\x80\x94\x81\x93\x7F\xB1\xD0`E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\n`\x04\x84\x01R\x7F\x9C\"\xFF_!\xF0\xB8\x1B\x11>c\xF7\xDBm\xA9O\xED\xEF\x11\xB2\x11\x9B@\x88\xB8\x96d\xFB\x9A<\xB6X`$\x84\x01R```D\x84\x01R`d\x83\x01\x90a6`V[PP\xFD[\x81a\x12\xEA\x91a8\nV[a\x07pW\x81_a\x12RV[\x81a\x12\xFF\x91a8\nV[a\x07pW\x81_a\x11\xACV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x14RW[PP`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`@Q\x91a\x0B\x86\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14%W\x91``\x93\x91\x85\x93a>\xF6\x859\x82R` \x82\x01R\x84`@\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x19W\x80\xF3[`@Q\x90=\x90\x82>=\x90\xFD[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a\x14\\\x91a8\nV[a\x01\xA7W\x80_a\x13\xB9V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1DTa\x14\x84\x81a8-V[\x91a\x14\x92`@Q\x93\x84a8\nV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x14\xD4W`@Q\x80a\x02;\x87\x82a7WV[`\x02` `\x01\x92`@Qa\x14\xE7\x81a7\xEEV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x14\xFF\x85\x87\x01a:\xE4V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x14\xBFV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x17\xF2W[P`\x04`\x01`\x01`\xA0\x1B\x03`\"T\x16` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x07\xC1W\x83\x92a\x17\xB9W[P`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Ra\x16G\x81`d\x81\x01a\teV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW\x81a\x16\xA2\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6`V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x17\xA4W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07pW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\x0F\xFD\x13\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`d`\x04\x84\x01RZ\xF1\x80\x15a\x04PWa\x17\x8FW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x04?WP\xF3[\x81a\x17\x99\x91a8\nV[a\x01\xA7W\x80_a\x17!V[\x81a\x17\xAE\x91a8\nV[a\x01\xA7W\x80_a\x16\xC7V[\x92P\x90P` \x82=` \x11a\x17\xEAW[\x81a\x17\xD6` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x90Q\x82\x91a\x16Ga\x15\xF8V[=\x91Pa\x17\xC9V[\x81a\x17\xFC\x91a8\nV[a\x01\xA7W\x80_a\x15\x9EV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x1D\rW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x07\xC1W\x83\x92a\x1C\xD6W[P\x80;\x15a\x12\xDCW\x82\x91\x82\x91`@Q\x80\x94\x81\x80\x94\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04` \x81\x84\x01`@\x81\x01\x93\x81R\x01R\x03\x92Z\xF1\x80\x15a\x04PWa\x1C\xC1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x1C\x8CW[P` `@Q\x80\x80\x93\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x84\x81\x84\x01`@\x81\x01\x93\x81R\x01R\x03\x81\x85Z\xFA\x80\x15a\x07\xC1Wa\x19\xDB\x91\x84\x91a\x08\x01WPa>yV[`@Q\x90\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x07\xC1W\x83\x92a\x1CUW[P\x80;\x15a\x12\xDCW\x82\x91\x82\x91`@Q\x80\x94\x81\x80\x94\x7F\xD5Gt\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04` \x81\x84\x01`@\x81\x01\x93\x81R\x01R\x03\x92Z\xF1\x80\x15a\x04PWa\x1C@W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x1C\tW[P` \x90`@Q\x92\x83\x80\x80\x93\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x86\x81\x84\x01`@\x81\x01\x93\x81R\x01R\x03\x91Z\xFA\x90\x81\x15a\x04PW\x82\x91a\x1B\xEAW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04PWa\x17\x8FWPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x04?WP\xF3[a\x1C\x03\x91P` =` \x11a\x08)Wa\x08\x1B\x81\x83a8\nV[_a\x1B\x0CV[\x92PP` \x82=` \x11a\x1C8W[\x81a\x1C%` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W` \x83\x92Q\x90a\x1A\xBCV[=\x91Pa\x1C\x18V[\x81a\x1CJ\x91a8\nV[a\x01\xA7W\x80_a\x1AoV[\x92P\x90P` \x82=` \x11a\x1C\x84W[\x81a\x1Cr` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x82\x91Q\x90_a\x1A\x19V[=\x91Pa\x1CeV[\x92PP` \x82=` \x11a\x1C\xB9W[\x81a\x1C\xA8` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x82\x91Q_a\x19\x85V[=\x91Pa\x1C\x9BV[\x81a\x1C\xCB\x91a8\nV[a\x01\xA7W\x80_a\x198V[\x92P\x90P` \x82=` \x11a\x1D\x05W[\x81a\x1C\xF3` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x82\x91Q\x90_a\x18\xE2V[=\x91Pa\x1C\xE6V[\x81a\x1D\x17\x91a8\nV[a\x01\xA7W\x80_a\x18\x94V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAdmin address cannot be 0\0\0\0\0\0\0\0`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x1E(W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`#T\x90`@Q\x91a\x0B\x86\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14%W\x91``\x93\x91\x85\x93a>\xF6\x859\x86\x83R` \x83\x01R`@\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x19W\x80\xF3[\x81a\x1E2\x91a8\nV[a\x01\xA7W\x80_a\x1D\xD1V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1ATa\x1EZ\x81a8-V[\x91a\x1Eh`@Q\x93\x84a8\nV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x1E\xAAW`@Q\x80a\x02;\x87\x82a6\xDAV[`\x01` \x81\x92a\x1E\xB9\x85a9\xE1V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x1E\x95V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1BTa\x1E\xE4\x81a8-V[a\x1E\xF1`@Q\x91\x82a8\nV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x1F\xC9W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x1F^WPPPP\x03\x90\xF3[\x91\x93` a\x1F\xB9\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x1F\xA9\x83Q`@\x84R`@\x84\x01\x90a6`V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra6\x85V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x1FOV[`\x02` `\x01\x92`@Qa\x1F\xDC\x81a7\xEEV[a\x1F\xE5\x86a9\xE1V[\x81Ra\x1F\xF2\x85\x87\x01a:\xE4V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1F!V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FManager address cannot be 0\0\0\0\0\0`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa!\nW[PP`\x01`\x01`\xA0\x1B\x03` T\x16`#T\x90`@Q\x91a\x0B\x86\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14%W\x91``\x93\x91\x85\x93a>\xF6\x859\x82R\x85` \x83\x01R`@\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x19W\x80\xF3[\x81a!\x14\x91a8\nV[a\x01\xA7W\x80_a \xB3V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@Qa!?`\x80\x82a8\nV[`\x03\x81R``\x90\x816` \x83\x017`\na!X\x82a8EV[R`\x0Ba!d\x82a8\x7FV[R`\x0Ca!p\x82a8\x8FV[R`@Q\x91a!\x80`\x80\x84a8\nV[`\x03\x83R6` \x84\x017\x7Fm%_\xC39\x0E\xE6\xB4\x11\x91\xDA1YX\xB7\xD6\xA1\xE5\xB1y\x04\xCCv\x83U\x8F\x98\xAC\xC5yw\xB4a!\xB4\x83a8EV[R\x7FM\xA42\xF1\xEC\xD4\xC0\xAC\x02\x8E\xBD\xE3\xA3\xF7\x85\x10\xA2\x1DT\x08{\x16\x15\x90\xA60\x80\xD3;p+\x8Da!\xDF\x83a8\x7FV[R\x7F EX\x07n\xFB B\xEB\xC9\xB04\xAA\xB3m\x85\xD6r\xD8\xAC\x1F\xA8\t(\x8D\xA5\xB4S\xA4qJ\xAEa\"\n\x83a8\x8FV[Ra\"\x13a8\x9FV[`@Qa\"Y\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a\"b\x82a8EV[Ra\"l\x81a8EV[P`@Qa\"\xB3\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a\"\xBC\x82a8\x7FV[Ra\"\xC6\x81a8\x7FV[P`@Qa#\r\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a#\x16\x82a8\x8FV[Ra# \x81a8\x8FV[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84\x81\x80a#\x89`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a$\xBDW[PPa#\xBC\x82a8EV[Qa#\xC6\x83a8\x8FV[Q`@Q\x91\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3\x87\x80\xA3`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C-W\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a\x04_WPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x04[Wa\x04.\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x81a$\xC7\x91a8\nV[a\x04tW\x83_a#\xB1V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa&\xC9W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07pW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\x0F\xFD\x13\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`d`\x04\x84\x01RZ\xF1\x80\x15a\x04PWa&\xB4W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xCC\xE5\xD2\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04PW\x82\x91a&\x7FW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`d`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04PWa\x04?WP\xF3[\x91PP` \x81=` \x11a&\xACW[\x81a&\x9B` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x81\x90Q_a&\x07V[=\x91Pa&\x8EV[\x81a&\xBE\x91a8\nV[a\x01\xA7W\x80_a%\xB9V[\x81a&\xD3\x91a8\nV[a\x01\xA7W\x80_a%_V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a'dWa\x02;\x85a\x02/\x81\x87\x03\x82a8\nV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a'MV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a'\xE2Wa\x02;\x85a\x02/\x81\x87\x03\x82a8\nV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a'\xCBV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@Qa(!`\x80\x82a8\nV[`\x03\x81R``6` \x83\x017`\na(8\x82a8EV[R`\x0Ba(D\x82a8\x7FV[R`\x0Ca(P\x82a8\x8FV[R`@Q\x90a(```\x83a8\nV[`\x02\x82R`@6` \x84\x017\x7Fm%_\xC39\x0E\xE6\xB4\x11\x91\xDA1YX\xB7\xD6\xA1\xE5\xB1y\x04\xCCv\x83U\x8F\x98\xAC\xC5yw\xB4a(\x96\x83a8EV[R\x7FM\xA42\xF1\xEC\xD4\xC0\xAC\x02\x8E\xBD\xE3\xA3\xF7\x85\x10\xA2\x1DT\x08{\x16\x15\x90\xA60\x80\xD3;p+\x8Da(\xC1\x83a8\x7FV[Ra(\xCAa8\x9FV[`@Qa)\x10\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a)\x19\x82a8EV[Ra)#\x81a8EV[P`@Qa)j\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a)s\x82a8\x7FV[Ra)}\x81a8\x7FV[P`@Qa)\xC4\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a)\xCD\x82a8\x8FV[Ra)\xD7\x81a8\x8FV[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04[W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a+QW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FArray lengths must be equal\0\0\0\0\0`D\x82\x01R\x84\x81\x80`d\x81\x01[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a\x04_WPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x04[Wa\x04.\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x81a+[\x91a8\nV[a\x04tW\x83_a*VV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@Qa+\xBE\x81a\te` \x82\x01``\x90` \x81R`\n` \x82\x01R\x7Ftest batch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12\xDCW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07\xC1W\x83\x91a.\xC8W[PP`\x04`\x01`\x01`\xA0\x1B\x03`\"T\x16` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a.\xBDW\x84\x92a.\x84W[P`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Ra,\xE6\x81`d\x81\x01a\teV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12\xDCW\x82a-A\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6`V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07\xC1W\x83\x91a.oW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x12\xDCWa-\xF3\x83\x92\x91\x83\x92`@Q\x94\x85\x80\x94\x81\x93\x7F\xB1\xD0`E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\n`\x04\x84\x01R\x7F\x9C\"\xFF_!\xF0\xB8\x1B\x11>c\xF7\xDBm\xA9O\xED\xEF\x11\xB2\x11\x9B@\x88\xB8\x96d\xFB\x9A<\xB6X`$\x84\x01R```D\x84\x01R`d\x83\x01\x90a6`V[\x03\x92Z\xF1\x80\x15a\x04PWa\x17\x8FWPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x04?WP\xF3[\x81a.y\x91a8\nV[a\x07pW\x81_a-iV[\x93P\x90P` \x83=` \x11a.\xB5W[\x81a.\xA1` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x91Q\x83\x92a,\xE6a,\x97V[=\x91Pa.\x94V[`@Q=\x86\x82>=\x90\xFD[\x81a.\xD2\x91a8\nV[a\x07pW\x81_a,<V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1ETa.\xFA\x81a8-V[a/\x07`@Q\x91\x82a8\nV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a0HW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a/sW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a/\xFFWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a/fV[\x90\x91\x92\x93\x94` \x80a0;\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa6`V[\x97\x01\x95\x01\x93\x92\x91\x01a/\xDBV[`@Qa0T\x81a7\xEEV[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta0p\x81a8-V[\x91a0~`@Q\x93\x84a8\nV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a0\xB4WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a/7V[`\x01` \x81\x92a0\xC3\x86a9\xE1V[\x81R\x01\x93\x01\x91\x01\x90\x91a0\x8EV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a1ZWa\x02;\x85a\x02/\x81\x87\x03\x82a8\nV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a1CV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"U`{`#U`@Qa\x0B\x86\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a2\xD0W``\x91\x83\x91a>\xF6\x839`\x01\x81R`\x02` \x82\x01R`{`@\x82\x01R\x03\x01\x90\x82\xF0\x80\x15a2\xC3W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80\xF3[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x07\xA0W_`\x03\x196\x01\x12a\x07\xA0W`@Q\x90a3\x1D`\x80\x83a8\nV[`\x03\x82R``\x806` \x85\x017`\na35\x84a8EV[R`\x0Ba3A\x84a8\x7FV[R`\x0Ca3M\x84a8\x8FV[R`@Q\x90a3]`\x80\x83a8\nV[`\x03\x82R6` \x83\x017\x7Fm%_\xC39\x0E\xE6\xB4\x11\x91\xDA1YX\xB7\xD6\xA1\xE5\xB1y\x04\xCCv\x83U\x8F\x98\xAC\xC5yw\xB4a3\x91\x82a8EV[R\x7FM\xA42\xF1\xEC\xD4\xC0\xAC\x02\x8E\xBD\xE3\xA3\xF7\x85\x10\xA2\x1DT\x08{\x16\x15\x90\xA60\x80\xD3;p+\x8Da3\xBC\x82a8\x7FV[R\x7F EX\x07n\xFB B\xEB\xC9\xB04\xAA\xB3m\x85\xD6r\xD8\xAC\x1F\xA8\t(\x8D\xA5\xB4S\xA4qJ\xAEa3\xE7\x82a8\x8FV[R`@Q\x92``a3\xF8\x81\x86a8\nV[`\x02\x85R`\x1F\x19\x01_[\x81\x81\x10a5\xE0WPP`@Qa4Q\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a4Z\x85a8EV[Ra4d\x84a8EV[P`@Qa4\xAB\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a4\xB4\x85a8\x7FV[Ra4\xBE\x84a8\x7FV[P`\x01`\x01`\xA0\x1B\x03`!T\x16\x93sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xA0W`@Q\x94\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01R_\x85`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a5\xD5Wa5\xBFW[\x83\x94Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FArray lengths must be equal\0\0\0\0\0`D\x82\x01R\x84\x81\x80`d\x81\x01a*\xD5V[\x92P\x92_a5\xCC\x91a8\nV[_\x91\x83\x90a5;V[`@Q=_\x82>=\x90\xFD[\x80``` \x80\x93\x89\x01\x01R\x01a4\x02V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a6AWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a64V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a6\xA2WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6\x95V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7\x0CWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a7H\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa6`V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a6\xFDV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7\x89WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a7\xDF\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a6\x85V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a7zV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5\xF1W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5\xF1W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a5\xF1W`\x05\x1B` \x01\x90V[\x80Q\x15a8RW` \x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80Q`\x01\x10\x15a8RW`@\x01\x90V[\x80Q`\x02\x10\x15a8RW``\x01\x90V[`@Q`\x80\x91\x90a8\xB0\x83\x82a8\nV[`\x03\x81R\x91`\x1F\x19\x01\x82_[\x82\x81\x10a8\xC8WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a8\xBCV[`@\x80Q\x90\x91\x90a8\xEA\x83\x82a8\nV[`\x01\x81R\x91`\x1F\x19\x01\x82_[\x82\x81\x10a9\x02WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a8\xF6V[\x90``\x82\x01``\x83R\x81Q\x80\x91R` `\x80\x84\x01\x92\x01\x90_[\x81\x81\x10a9\xCBWPPP\x81\x81\x03` \x83\x01R` \x80\x84Q\x92\x83\x81R\x01\x93\x01\x90_[\x81\x81\x10a9\xB5WPPP`@\x81\x83\x03\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a9\x88WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a9\xA6\x83`\x1F\x19\x86`\x01\x96\x03\x01\x87R\x89Qa6`V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a9yV[\x82Q\x85R` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a9MV[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a9,V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a:\xDAW[` \x85\x10\x84\x14a:\xADW\x84\x87R\x86\x93\x90\x81\x15a:mWP`\x01\x14a:)W[Pa:'\x92P\x03\x83a8\nV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a:QWPP\x90` a:'\x92\x82\x01\x01_a:\x1AV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a:8V[` \x93Pa:'\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a:\x1AV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a9\xFBV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a<\xFBWa:'\x94T\x91\x81\x81\x10a<\xC5W[\x81\x81\x10a<\x8FW[\x81\x81\x10a<YW[\x81\x81\x10a<#W[\x81\x81\x10a;\xEDW[\x81\x81\x10a;\xB7W[\x81\x81\x10a;\x82W[\x10a;UW[P\x03\x83a8\nV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a;MV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a;GV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a;?V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a;7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a;/V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a;'V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a;\x1FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a;\x17V[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a:\xFFV[\x90\x81` \x91\x03\x12a\x07\xA0WQ\x80\x15\x15\x81\x03a\x07\xA0W\x90V[`\x08T`\xFF\x16\x80\x15a=\xAFW\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a5\xD5W_\x91a>GW[P\x15\x15\x90V[\x90P` \x81=` \x11a>qW[\x81a>b` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0WQ_a>AV[=\x91Pa>UV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xA0W`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5\xD5Wa>\xEBWPV[_a:'\x91a8\nV\xFE`\xA04a\x01zW`\x1Fa\x0B\x868\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01~W\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\x01zWa\0G\x81a\x01\x92V[`@a\0U` \x84\x01a\x01\x92V[\x92\x01Q\x91`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x015W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\0\xF0W\x82\x15a\0\xABWa\0\x89a\0\x8F\x92a\x01\xA6V[Pa\x02\x1CV[P`\x80R`@Qa\x08v\x90\x81a\x02\xB0\x829`\x80Q\x81a\x01\0\x01R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FManager address cannot be 0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAdmin address cannot be 0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01zWV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x0Bf_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02\x17W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x0Bf_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x0B&_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x0BF_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02\x17W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x0BF_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08\x90_Q` a\x0B&_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\x04\xECWP\x80c\x04\x84 \xA7\x14a\x036W\x80c\x0F\xFD\x13\x80\x14a\x03\x15W\x80c$\x8A\x9C\xA3\x14a\x02\xE3W\x80c//\xF1]\x14a\x02\xB4W\x80c6V\x8A\xBE\x14a\x02XW\x80c\x91\xD1HT\x14a\x02\x0FW\x80c\xA2\x17\xFD\xDF\x14a\x01\xF5W\x80c\xB1\xD0`E\x14a\x01vW\x80c\xCC\xE5\xD2\xF6\x14a\x01YW\x80c\xD5Gt\x1F\x14a\x01#W\x80c\xD8x\x13B\x14a\0\xE9Wc\xEC\x87b\x1C\x14a\0\xABW_\x80\xFD[4a\0\xE5W_`\x03\x196\x01\x12a\0\xE5W` `@Q\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08\x81R\xF3[_\x80\xFD[4a\0\xE5W_`\x03\x196\x01\x12a\0\xE5W` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[4a\0\xE5Wa\x01Wa\x0146a\x05\xBBV[\x90a\x01Ra\x01M\x82_R_` R`\x01`@_ \x01T\x90V[a\x06vV[a\x07\xAEV[\0[4a\0\xE5W_`\x03\x196\x01\x12a\0\xE5W` `\x01T`@Q\x90\x81R\xF3[4a\0\xE5W```\x03\x196\x01\x12a\0\xE5W`\x045`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5W6`#\x82\x01\x12\x15a\0\xE5W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5W6\x91\x01`$\x01\x11a\0\xE5Wa\x01\xCEa\x05\xEEV[\x80\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3_\x80\xA3\0[4a\0\xE5W_`\x03\x196\x01\x12a\0\xE5W` `@Q_\x81R\xF3[4a\0\xE5Wa\x02\x1D6a\x05\xBBV[\x90_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\0\xE5Wa\x02f6a\x05\xBBV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x02\x8CWa\x01W\x91a\x07\xAEV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\0\xE5Wa\x01Wa\x02\xC56a\x05\xBBV[\x90a\x02\xDEa\x01M\x82_R_` R`\x01`@_ \x01T\x90V[a\x06\xDCV[4a\0\xE5W` `\x03\x196\x01\x12a\0\xE5W` a\x03\r`\x045_R_` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\0\xE5W` `\x03\x196\x01\x12a\0\xE5Wa\x03.a\x05\xEEV[`\x045`\x01U\0[4a\0\xE5W```\x03\x196\x01\x12a\0\xE5W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5Wa\x03g\x906\x90`\x04\x01a\x05\x8AV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5Wa\x03\x87\x906\x90`\x04\x01a\x05\x8AV[\x90P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5Wa\x03\xA9\x906\x90`\x04\x01a\x05\x8AV[\x90Pa\x03\xB3a\x05\xEEV[\x81\x83\x14\x91\x82a\x04\xE2W[PP\x15a\x04\x84W\x80\x15a\x04*W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x80\x82\x11a\x04WW\x81\x10\x15a\x04*W`\x05\x1B\x81\x015\x905\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3_\x80\xA3\0[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FArray lengths must be equal\0\0\0\0\0`D\x82\x01R\xFD[\x14\x90P\x83\x80a\x03\xBDV[4a\0\xE5W` `\x03\x196\x01\x12a\0\xE5W`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\0\xE5W\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\x05`W[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\x05YV[\x91\x81`\x1F\x84\x01\x12\x15a\0\xE5W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xE5W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\0\xE5WV[`\x03\x19`@\x91\x01\x12a\0\xE5W`\x045\x90`$5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\0\xE5W\x90V[3_\x90\x81R\x7F\xE8E\x08\xF2\xC7\xFA\x9C5\x11Ft\x8B0%\xCBx\xB4]\xF3}\x86\x8EH\xC6\xA7Q\x02\xFE\xCD\xEE\xE6E` R`@\x90 T`\xFF\x16\x15a\x06&WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x06\xADWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x07\xA8W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x07\xA8W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xE8E\x08\xF2\xC7\xFA\x9C5\x11Ft\x8B0%\xCBx\xB4]\xF3}\x86\x8EH\xC6\xA7Q\x02\xFE\xCD\xEE\xE6E\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f5f3560e01c806305a56f80146132fd5780630a9254e4146131a05780631ddc2d10146131795780631ed7831c146130fb578063297feeb2146130d15780632ade388014612edd5780632c41ee6714612b665780633b7b325b146128015780633e5e3c23146127835780633f7286f414612705578063481c6a75146126de57806351f5287a146124d257806353882c6e1461211f57806359ebf0491461200457806366d9a9a014611ec757806385226c8114611e3d5780638d3a83b914611d225780638d3d6c991461180757806390c9677014611511578063916a17c614611467578063965140251461130a57806396e181cf146110c4578063ac29af3314610e01578063b0464fdc14610d57578063b5508aa914610ccd578063ba414fa614610ca8578063c0ce94ad146108a0578063c2e9f2e4146104a8578063ca505fe21461027c578063d87813421461025e578063e20c9f71146101d0578063f851a440146101aa5763fa7626d414610185575f80fd5b346101a757806003193601126101a757602060ff601f54166040519015158152f35b80fd5b50346101a757806003193601126101a75760206001600160a01b03815416604051908152f35b50346101a757806003193601126101a75760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b81811061023f5761023b8561022f8187038261380a565b6040519182918261361e565b0390f35b82546001600160a01b0316845260209093019260019283019201610218565b50346101a757806003193601126101a7576020602354604051908152f35b50346101a757806003193601126101a75760209060405161029d838261380a565b8181525f368137604051906102b2848361380a565b8282525f3681376102c28361382d565b6102cf604051918261380a565b838152601f196102de8561382d565b01845b8181106104995750508394506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561045b57604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610479578591610484575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474576040517ff4844814000000000000000000000000000000000000000000000000000000008152848160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561047957859161045f575b50506001600160a01b03601f5460081c16803b1561045b5761042e93858094604051968795869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b03925af180156104505761043f5750f35b816104499161380a565b6101a75780f35b6040513d84823e3d90fd5b8480fd5b816104699161380a565b61047457835f6103d9565b505050fd5b6040513d87823e3d90fd5b8161048e9161380a565b61047457835f61036b565b606083820188015286016102e1565b50346101a757806003193601126101a757806001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c1578391610868575b50602080546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101939093526001600160a01b0316602483015281604481855afa80156107c157610565918491610801575b50613e79565b6040517fec87621c000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c1578391610830575b506021546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b03166024820152602081604481855afa80156107c1576106019184916108015750613e79565b6040517fd8781342000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c15783916107cc575b50602354737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561047457604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156107c15783916107ac575b50506020600491604051928380927fcce5d2f60000000000000000000000000000000000000000000000000000000082525afa908115610450578291610773575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201528160248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156104505761043f5750f35b50fd5b9150506020813d6020116107a4575b8161078f6020938361380a565b810103126107a0578190515f6106f9565b5f80fd5b3d9150610782565b816107b69161380a565b61077057815f6106b8565b6040513d85823e3d90fd5b9250506020823d6020116107f9575b816107e86020938361380a565b810103126107a0578291515f61063e565b3d91506107db565b610823915060203d602011610829575b61081b818361380a565b810190613d88565b5f61055f565b503d610811565b9250506020823d602011610860575b8161084c6020938361380a565b810103126107a057905182919060206105a2565b3d915061083f565b9250506020823d602011610898575b816108846020938361380a565b810103126107a05790518291906020610506565b3d9150610877565b50346101a757806003193601126101a75780604080516108c0828261380a565b60018152601f1982019081366020830137600a6108dc82613845565b528251916108ea848461380a565b600183523660208401377f9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb65861091e83613845565b526109276138d9565b8351602080820152600a60408201527f7465737420626174636800000000000000000000000000000000000000000000606082015261097381608081015b03601f19810183528261380a565b61097c82613845565b5261098681613845565b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c2d578451907f06447d560000000000000000000000000000000000000000000000000000000082526004820152858160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c4657908691610c93575b505060046001600160a01b036022541660206001600160a01b03601f5460081c168751938480927fec87621c0000000000000000000000000000000000000000000000000000000082525afa918215610c89578792610c50575b5085517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b0390911660248201526044810191909152610aac8160648101610965565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c2d5785610b06918651809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190613660565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c4657908691610c31575b50506001600160a01b03601f5460081c16803b15610c2d57610b82938680948751968795869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b03925af18015610c2157908391610c0c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610770578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c03575061043f5750f35b513d84823e3d90fd5b81610c169161380a565b61077057815f610b94565b505051903d90823e3d90fd5b8580fd5b81610c3b9161380a565b61045b57845f610b2e565b85513d88823e3d90fd5b965090506020863d602011610c81575b81610c6d6020938361380a565b810103126107a05794518695610aac610a5e565b3d9150610c60565b86513d89823e3d90fd5b81610c9d9161380a565b61045b57845f610a04565b50346101a757806003193601126101a7576020610cc3613da0565b6040519015158152f35b50346101a757806003193601126101a757601954610cea8161382d565b91610cf8604051938461380a565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310610d3a576040518061023b87826136da565b600160208192610d49856139e1565b815201920192019190610d25565b50346101a757806003193601126101a757601c54610d748161382d565b91610d82604051938461380a565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610dc4576040518061023b8782613757565b60026020600192604051610dd7816137ee565b6001600160a01b038654168152610def858701613ae4565b83820152815201920192019190610daf565b50346101a757806003193601126101a757806040805190610e22818361380a565b60018252601f1981019182366020830137600a610e3e82613845565b52815192610e4c838561380a565b600184523660208501377f9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb658610e8084613845565b52610e896138d9565b8251602080820152600a60408201527f74657374206261746368000000000000000000000000000000000000000000006060820152610ecb8160808101610965565b610ed482613845565b52610ede81613845565b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561045b5782517f491cc7c2000000000000000000000000000000000000000000000000000000008152858180610f4660048201906001606060808401938281528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156110a1579086916110af575b5050610f7982613845565b51610f8383613845565b518451917ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f38880a36001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156110ab577fca669fa70000000000000000000000000000000000000000000000000000000082526004820152858160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156110a15790869161108c575b50506001600160a01b03601f5460081c16803b15610c2d57611079948680948651978895869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b03925af1908115610c03575061043f5750f35b816110969161380a565b61045b57845f611025565b84513d88823e3d90fd5b8680fd5b816110b99161380a565b61045b57845f610f6e565b50346101a757806003193601126101a7578060405161111c816109656020820160609060208152600a60208201527f746573742062617463680000000000000000000000000000000000000000000060408201520190565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610770576040517f491cc7c200000000000000000000000000000000000000000000000000000000815282818061118460048201906001606060808401938281528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156107c15783916112f5575b5050604051600a807ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f38580a36001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474577fca669fa70000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156107c15783916112e0575b50506001600160a01b03601f5460081c16803b156112dc5761042e83929183926040519485809481937fb1d06045000000000000000000000000000000000000000000000000000000008352600a60048401527f9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb6586024840152606060448401526064830190613660565b5050fd5b816112ea9161380a565b61077057815f611252565b816112ff9161380a565b61077057815f6111ac565b50346101a757806003193601126101a757737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f41707020636861696e2049442063616e6e6f74206265203000000000000000006044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561045057611452575b50506001600160a01b03602054166001600160a01b03602154169060405191610b86918284019284841067ffffffffffffffff8511176114255791606093918593613ef685398252602082015284604082015203019082f0156114195780f35b604051903d90823e3d90fd5b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b8161145c9161380a565b6101a757805f6113b9565b50346101a757806003193601126101a757601d546114848161382d565b91611492604051938461380a565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b8383106114d4576040518061023b8782613757565b600260206001926040516114e7816137ee565b6001600160a01b0386541681526114ff858701613ae4565b838201528152019201920191906114bf565b50346101a757806003193601126101a757806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610450576117f2575b5060046001600160a01b036022541660206001600160a01b03601f5460081c16604051938480927fec87621c0000000000000000000000000000000000000000000000000000000082525afa9182156107c15783926117b9575b506040517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b03909116602482015260448101919091526116478160648101610965565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057816116a291604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190613660565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610450576117a4575b506001600160a01b03601f5460081c16803b15610770578180916024604051809481937f0ffd1380000000000000000000000000000000000000000000000000000000008352606460048401525af180156104505761178f575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104505761043f5750f35b816117999161380a565b6101a757805f611721565b816117ae9161380a565b6101a757805f6116c7565b925090506020823d6020116117ea575b816117d66020938361380a565b810103126107a057905182916116476115f8565b3d91506117c9565b816117fc9161380a565b6101a757805f61159e565b50346101a757806003193601126101a757806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561045057611d0d575b506001600160a01b03601f5460081c16604051907fec87621c000000000000000000000000000000000000000000000000000000008252602082600481845afa9182156107c1578392611cd6575b50803b156112dc578291829160405180948180947f2f2ff15d0000000000000000000000000000000000000000000000000000000082526004602081840160408101938152015203925af1801561045057611cc1575b506001600160a01b03601f5460081c166040517fec87621c000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c1578391611c8c575b5060206040518080937f91d148540000000000000000000000000000000000000000000000000000000082526004848184016040810193815201520381855afa80156107c1576119db9184916108015750613e79565b604051907fec87621c000000000000000000000000000000000000000000000000000000008252602082600481845afa9182156107c1578392611c55575b50803b156112dc578291829160405180948180947fd547741f0000000000000000000000000000000000000000000000000000000082526004602081840160408101938152015203925af1801561045057611c40575b506001600160a01b03601f5460081c166040517fec87621c000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156107c1578391611c09575b5060209060405192838080937f91d1485400000000000000000000000000000000000000000000000000000000825260048681840160408101938152015203915afa908115610450578291611bea575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156104505761178f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104505761043f5750f35b611c03915060203d6020116108295761081b818361380a565b5f611b0c565b9250506020823d602011611c38575b81611c256020938361380a565b810103126107a057602083925190611abc565b3d9150611c18565b81611c4a9161380a565b6101a757805f611a6f565b925090506020823d602011611c84575b81611c726020938361380a565b810103126107a057829151905f611a19565b3d9150611c65565b9250506020823d602011611cb9575b81611ca86020938361380a565b810103126107a0578291515f611985565b3d9150611c9b565b81611ccb9161380a565b6101a757805f611938565b925090506020823d602011611d05575b81611cf36020938361380a565b810103126107a057829151905f6118e2565b3d9150611ce6565b81611d179161380a565b6101a757805f611894565b50346101a757806003193601126101a757737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601960248201527f41646d696e20616464726573732063616e6e6f742062652030000000000000006044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561045057611e28575b50506001600160a01b03602154166023549060405191610b86918284019284841067ffffffffffffffff8511176114255791606093918593613ef685398683526020830152604082015203019082f0156114195780f35b81611e329161380a565b6101a757805f611dd1565b50346101a757806003193601126101a757601a54611e5a8161382d565b91611e68604051938461380a565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310611eaa576040518061023b87826136da565b600160208192611eb9856139e1565b815201920192019190611e95565b50346101a757806003193601126101a757601b54611ee48161382d565b611ef1604051918261380a565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310611fc957868587604051928392602084019060208552518091526040840160408260051b8601019392905b828210611f5e57505050500390f35b91936020611fb9827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083611fa98351604084526040840190613660565b9201519084818403910152613685565b9601920192018594939192611f4f565b60026020600192604051611fdc816137ee565b611fe5866139e1565b8152611ff2858701613ae4565b83820152815201920192019190611f21565b50346101a757806003193601126101a757737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f4d616e6167657220616464726573732063616e6e6f74206265203000000000006044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104505761210a575b50506001600160a01b03602054166023549060405191610b86918284019284841067ffffffffffffffff8511176114255791606093918593613ef685398252856020830152604082015203019082f0156114195780f35b816121149161380a565b6101a757805f6120b3565b50346101a757806003193601126101a7578060405161213f60808261380a565b6003815260609081366020830137600a61215882613845565b52600b6121648261387f565b52600c6121708261388f565b526040519161218060808461380a565b600383523660208401377f6d255fc3390ee6b41191da315958b7d6a1e5b17904cc7683558f98acc57977b46121b483613845565b527f4da432f1ecd4c0ac028ebde3a3f78510a21d54087b161590a63080d33b702b8d6121df8361387f565b527f204558076efb2042ebc9b034aab36d85d672d8ac1fa809288da5b453a4714aae61220a8361388f565b5261221361389f565b604051612259816109656020820160609060208152600c60208201527f746573742062617463682031000000000000000000000000000000000000000060408201520190565b61226282613845565b5261226c81613845565b506040516122b3816109656020820160609060208152600c60208201527f746573742062617463682032000000000000000000000000000000000000000060408201520190565b6122bc8261387f565b526122c68161387f565b5060405161230d816109656020820160609060208152600c60208201527f746573742062617463682033000000000000000000000000000000000000000060408201520190565b6123168261388f565b526123208161388f565b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474576040517f491cc7c200000000000000000000000000000000000000000000000000000000815284818061238960048201906001606060808401938281528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104795785916124bd575b50506123bc82613845565b516123c68361388f565b51604051917ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f38780a36001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c2d577fca669fa70000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561047957859161045f5750506001600160a01b03601f5460081c16803b1561045b5761042e93858094604051968795869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b816124c79161380a565b61047457835f6123b1565b50346101a757806003193601126101a757806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610450576126c9575b506001600160a01b03601f5460081c16803b15610770578180916024604051809481937f0ffd1380000000000000000000000000000000000000000000000000000000008352606460048401525af18015610450576126b4575b50600460206001600160a01b03601f5460081c16604051928380927fcce5d2f60000000000000000000000000000000000000000000000000000000082525afa90811561045057829161267f575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077057604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152606460248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156104505761043f5750f35b9150506020813d6020116126ac575b8161269b6020938361380a565b810103126107a0578190515f612607565b3d915061268e565b816126be9161380a565b6101a757805f6125b9565b816126d39161380a565b6101a757805f61255f565b50346101a757806003193601126101a75760206001600160a01b0360215416604051908152f35b50346101a757806003193601126101a75760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b8181106127645761023b8561022f8187038261380a565b82546001600160a01b031684526020909301926001928301920161274d565b50346101a757806003193601126101a75760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b8181106127e25761023b8561022f8187038261380a565b82546001600160a01b03168452602090930192600192830192016127cb565b50346101a757806003193601126101a7578060405161282160808261380a565b600381526060366020830137600a61283882613845565b52600b6128448261387f565b52600c6128508261388f565b526040519061286060608361380a565b6002825260403660208401377f6d255fc3390ee6b41191da315958b7d6a1e5b17904cc7683558f98acc57977b461289683613845565b527f4da432f1ecd4c0ac028ebde3a3f78510a21d54087b161590a63080d33b702b8d6128c18361387f565b526128ca61389f565b604051612910816109656020820160609060208152600c60208201527f746573742062617463682031000000000000000000000000000000000000000060408201520190565b61291982613845565b5261292381613845565b5060405161296a816109656020820160609060208152600c60208201527f746573742062617463682032000000000000000000000000000000000000000060408201520190565b6129738261387f565b5261297d8161387f565b506040516129c4816109656020820160609060208152600c60208201527f746573742062617463682033000000000000000000000000000000000000000060408201520190565b6129cd8261388f565b526129d78161388f565b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561045b57604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610479578591612b51575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f4172726179206c656e67746873206d75737420626520657175616c00000000006044820152848180606481015b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561047957859161045f5750506001600160a01b03601f5460081c16803b1561045b5761042e93858094604051968795869485937f048420a700000000000000000000000000000000000000000000000000000000855260048501613913565b81612b5b9161380a565b61047457835f612a56565b50346101a757806003193601126101a75780604051612bbe816109656020820160609060208152600a60208201527f746573742062617463680000000000000000000000000000000000000000000060408201520190565b6001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156112dc57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156107c1578391612ec8575b505060046001600160a01b036022541660206001600160a01b03601f5460081c16604051938480927fec87621c0000000000000000000000000000000000000000000000000000000082525afa918215612ebd578492612e84575b506040517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b0390911660248201526044810191909152612ce68160648101610965565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156112dc5782612d4191604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190613660565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156107c1578391612e6f575b50506001600160a01b03601f5460081c16803b156112dc57612df383929183926040519485809481937fb1d06045000000000000000000000000000000000000000000000000000000008352600a60048401527f9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb6586024840152606060448401526064830190613660565b03925af180156104505761178f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101a757806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104505761043f5750f35b81612e799161380a565b61077057815f612d69565b935090506020833d602011612eb5575b81612ea16020938361380a565b810103126107a05791518392612ce6612c97565b3d9150612e94565b6040513d86823e3d90fd5b81612ed29161380a565b61077057815f612c3c565b50346101a757806003193601126101a757601e54612efa8161382d565b612f07604051918261380a565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106130485786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310612f735786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110612fff57505050505060208060019297019301930190928695949293612f66565b909192939460208061303b837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951613660565b9701950193929101612fdb565b604051613054816137ee565b6001600160a01b0383541681526001830180546130708161382d565b9161307e604051938461380a565b8183528a526020808b20908b9084015b8382106130b4575050505060019282602092836002950152815201920192019190612f37565b6001602081926130c3866139e1565b81520193019101909161308e565b50346101a757806003193601126101a75760206001600160a01b03601f5460081c16604051908152f35b50346101a757806003193601126101a75760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061315a5761023b8561022f8187038261380a565b82546001600160a01b0316845260209093019260019283019201613143565b50346101a757806003193601126101a75760206001600160a01b0360225416604051908152f35b50346101a757806003193601126101a75760017fffffffffffffffffffffffff0000000000000000000000000000000000000000602054161760205560027fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215560037fffffffffffffffffffffffff00000000000000000000000000000000000000006022541617602255607b602355604051610b868082019082821067ffffffffffffffff8311176132d0576060918391613ef683396001815260026020820152607b604082015203019082f080156132c3577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f5580f35b50604051903d90823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346107a0575f6003193601126107a0576040519061331d60808361380a565b60038252606080366020850137600a61333584613845565b52600b6133418461387f565b52600c61334d8461388f565b526040519061335d60808361380a565b600382523660208301377f6d255fc3390ee6b41191da315958b7d6a1e5b17904cc7683558f98acc57977b461339182613845565b527f4da432f1ecd4c0ac028ebde3a3f78510a21d54087b161590a63080d33b702b8d6133bc8261387f565b527f204558076efb2042ebc9b034aab36d85d672d8ac1fa809288da5b453a4714aae6133e78261388f565b526040519260606133f8818661380a565b60028552601f19015f5b8181106135e0575050604051613451816109656020820160609060208152600c60208201527f746573742062617463682031000000000000000000000000000000000000000060408201520190565b61345a85613845565b5261346484613845565b506040516134ab816109656020820160609060208152600c60208201527f746573742062617463682032000000000000000000000000000000000000000060408201520190565b6134b48561387f565b526134be8461387f565b506001600160a01b036021541693737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107a057604051947fca669fa700000000000000000000000000000000000000000000000000000000865260048601525f8560248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156135d5576135bf575b839450737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610474576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f4172726179206c656e67746873206d75737420626520657175616c0000000000604482015284818060648101612ad5565b9250925f6135cc9161380a565b5f91839061353b565b6040513d5f823e3d90fd5b806060602080938901015201613402565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60206040818301928281528451809452019201905f5b8181106136415750505090565b82516001600160a01b0316845260209384019390920191600101613634565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b8181106136a25750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101613695565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061370c57505050505090565b9091929394602080613748837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951613660565b970193019301919392906136fd565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061378957505050505090565b90919293946020806137df837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b03815116845201519181858201520190613685565b9701930193019193929061377a565b6040810190811067ffffffffffffffff8211176135f157604052565b90601f601f19910116810190811067ffffffffffffffff8211176135f157604052565b67ffffffffffffffff81116135f15760051b60200190565b8051156138525760200190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b8051600110156138525760400190565b8051600210156138525760600190565b604051608091906138b0838261380a565b6003815291601f1901825f5b8281106138c857505050565b8060606020809385010152016138bc565b604080519091906138ea838261380a565b6001815291601f1901825f5b82811061390257505050565b8060606020809385010152016138f6565b90606082016060835281518091526020608084019201905f5b8181106139cb575050508181036020830152602080845192838152019301905f5b8181106139b5575050506040818303910152815180825260208201916020808360051b8301019401925f915b83831061398857505050505090565b90919293946020806139a683601f1986600196030187528951613660565b97019301930191939290613979565b825185526020948501949092019160010161394d565b825184526020938401939092019160010161392c565b90604051915f8154908160011c9260018316928315613ada575b602085108414613aad578487528693908115613a6d5750600114613a29575b50613a279250038361380a565b565b90505f9291925260205f20905f915b818310613a51575050906020613a27928201015f613a1a565b6020919350806001915483858901015201910190918492613a38565b60209350613a279592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f613a1a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f16936139fb565b90604051918281549182825260208201905f5260205f20925f905b806007830110613cfb57613a27945491818110613cc5575b818110613c8f575b818110613c59575b818110613c23575b818110613bed575b818110613bb7575b818110613b82575b10613b55575b50038361380a565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f613b4d565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301613b47565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301613b3f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301613b37565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301613b2f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301613b27565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301613b1f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301613b17565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391613aff565b908160209103126107a0575180151581036107a05790565b60085460ff168015613daf5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156135d5575f91613e47575b50151590565b90506020813d602011613e71575b81613e626020938361380a565b810103126107a057515f613e41565b3d9150613e55565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107a057604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156135d557613eeb5750565b5f613a279161380a56fe60a03461017a57601f610b8638819003918201601f19168301916001600160401b0383118484101761017e5780849260609460405283398101031261017a5761004781610192565b604061005560208401610192565b920151916001600160a01b03821615610135576001600160a01b038116156100f05782156100ab5761008961008f926101a6565b5061021c565b5060805260405161087690816102b08239608051816101000152f35b60405162461bcd60e51b815260206004820152601860248201527f41707020636861696e2049442063616e6e6f74206265203000000000000000006044820152606490fd5b60405162461bcd60e51b815260206004820152601b60248201527f4d616e6167657220616464726573732063616e6e6f74206265203000000000006044820152606490fd5b60405162461bcd60e51b815260206004820152601960248201527f41646d696e20616464726573732063616e6e6f742062652030000000000000006044820152606490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b038216820361017a57565b6001600160a01b0381165f9081525f516020610b665f395f51905f52602052604090205460ff16610217576001600160a01b03165f8181525f516020610b665f395f51905f5260205260408120805460ff191660011790553391905f516020610b265f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f516020610b465f395f51905f52602052604090205460ff16610217576001600160a01b03165f8181525f516020610b465f395f51905f5260205260408120805460ff191660011790553391907f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b08905f516020610b265f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f3560e01c90816301ffc9a7146104ec57508063048420a7146103365780630ffd138014610315578063248a9ca3146102e35780632f2ff15d146102b457806336568abe1461025857806391d148541461020f578063a217fddf146101f5578063b1d0604514610176578063cce5d2f614610159578063d547741f14610123578063d8781342146100e95763ec87621c146100ab575f80fd5b346100e5575f6003193601126100e55760206040517f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b088152f35b5f80fd5b346100e5575f6003193601126100e55760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b346100e557610157610134366105bb565b9061015261014d825f525f602052600160405f20015490565b610676565b6107ae565b005b346100e5575f6003193601126100e5576020600154604051908152f35b346100e55760606003193601126100e55760043560443567ffffffffffffffff81116100e557366023820112156100e557806004013567ffffffffffffffff81116100e557369101602401116100e5576101ce6105ee565b807ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f35f80a3005b346100e5575f6003193601126100e55760206040515f8152f35b346100e55761021d366105bb565b905f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b346100e557610266366105bb565b3373ffffffffffffffffffffffffffffffffffffffff82160361028c57610157916107ae565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346100e5576101576102c5366105bb565b906102de61014d825f525f602052600160405f20015490565b6106dc565b346100e55760206003193601126100e557602061030d6004355f525f602052600160405f20015490565b604051908152f35b346100e55760206003193601126100e55761032e6105ee565b600435600155005b346100e55760606003193601126100e55760043567ffffffffffffffff81116100e55761036790369060040161058a565b60243567ffffffffffffffff81116100e55761038790369060040161058a565b905060443567ffffffffffffffff81116100e5576103a990369060040161058a565b90506103b36105ee565b81831491826104e2575b50501561048457801561042a577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908082116104575781101561042a5760051b81013590357ffb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f35f80a3005b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f4172726179206c656e67746873206d75737420626520657175616c00000000006044820152fd5b14905083806103bd565b346100e55760206003193601126100e557600435907fffffffff0000000000000000000000000000000000000000000000000000000082168092036100e557817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115610560575b5015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483610559565b9181601f840112156100e55782359167ffffffffffffffff83116100e5576020808501948460051b0101116100e557565b60031960409101126100e5576004359060243573ffffffffffffffffffffffffffffffffffffffff811681036100e55790565b335f9081527fe84508f2c7fa9c351146748b3025cb78b45df37d868e48c6a75102fecdeee645602052604090205460ff161561062657565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b0860245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156106ad5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f146107a857805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f146107a857805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0de84508f2c7fa9c351146748b3025cb78b45df37d868e48c6a75102fecdeee645ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x05\xA5o\x80\x14a2\xFDW\x80c\n\x92T\xE4\x14a1\xA0W\x80c\x1D\xDC-\x10\x14a1yW\x80c\x1E\xD7\x83\x1C\x14a0\xFBW\x80c)\x7F\xEE\xB2\x14a0\xD1W\x80c*\xDE8\x80\x14a.\xDDW\x80c,A\xEEg\x14a+fW\x80c;{2[\x14a(\x01W\x80c>^<#\x14a'\x83W\x80c?r\x86\xF4\x14a'\x05W\x80cH\x1Cju\x14a&\xDEW\x80cQ\xF5(z\x14a$\xD2W\x80cS\x88,n\x14a!\x1FW\x80cY\xEB\xF0I\x14a \x04W\x80cf\xD9\xA9\xA0\x14a\x1E\xC7W\x80c\x85\"l\x81\x14a\x1E=W\x80c\x8D:\x83\xB9\x14a\x1D\"W\x80c\x8D=l\x99\x14a\x18\x07W\x80c\x90\xC9gp\x14a\x15\x11W\x80c\x91j\x17\xC6\x14a\x14gW\x80c\x96Q@%\x14a\x13\nW\x80c\x96\xE1\x81\xCF\x14a\x10\xC4W\x80c\xAC)\xAF3\x14a\x0E\x01W\x80c\xB0FO\xDC\x14a\rWW\x80c\xB5P\x8A\xA9\x14a\x0C\xCDW\x80c\xBAAO\xA6\x14a\x0C\xA8W\x80c\xC0\xCE\x94\xAD\x14a\x08\xA0W\x80c\xC2\xE9\xF2\xE4\x14a\x04\xA8W\x80c\xCAP_\xE2\x14a\x02|W\x80c\xD8x\x13B\x14a\x02^W\x80c\xE2\x0C\x9Fq\x14a\x01\xD0W\x80c\xF8Q\xA4@\x14a\x01\xAAWc\xFAv&\xD4\x14a\x01\x85W_\x80\xFD[4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x02?Wa\x02;\x85a\x02/\x81\x87\x03\x82a8\nV[`@Q\x91\x82\x91\x82a6\x1EV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x02\x18V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `#T`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` \x90`@Qa\x02\x9D\x83\x82a8\nV[\x81\x81R_6\x817`@Q\x90a\x02\xB2\x84\x83a8\nV[\x82\x82R_6\x817a\x02\xC2\x83a8-V[a\x02\xCF`@Q\x91\x82a8\nV[\x83\x81R`\x1F\x19a\x02\xDE\x85a8-V[\x01\x84[\x81\x81\x10a\x04\x99WPP\x83\x94P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04[W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a\x04\x84W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a\x04_W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x04[Wa\x04.\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x03\x92Z\xF1\x80\x15a\x04PWa\x04?WP\xF3[\x81a\x04I\x91a8\nV[a\x01\xA7W\x80\xF3[`@Q=\x84\x82>=\x90\xFD[\x84\x80\xFD[\x81a\x04i\x91a8\nV[a\x04tW\x83_a\x03\xD9V[PPP\xFD[`@Q=\x87\x82>=\x90\xFD[\x81a\x04\x8E\x91a8\nV[a\x04tW\x83_a\x03kV[``\x83\x82\x01\x88\x01R\x86\x01a\x02\xE1V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x08hW[P` \x80T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R\x81`D\x81\x85Z\xFA\x80\x15a\x07\xC1Wa\x05e\x91\x84\x91a\x08\x01W[Pa>yV[`@Q\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x080W[P`!T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R` \x81`D\x81\x85Z\xFA\x80\x15a\x07\xC1Wa\x06\x01\x91\x84\x91a\x08\x01WPa>yV[`@Q\x7F\xD8x\x13B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x07\xCCW[P`#Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x07\xACW[PP` `\x04\x91`@Q\x92\x83\x80\x92\x7F\xCC\xE5\xD2\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04PW\x82\x91a\x07sW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04PWa\x04?WP\xF3[P\xFD[\x91PP` \x81=` \x11a\x07\xA4W[\x81a\x07\x8F` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x81\x90Q_a\x06\xF9V[_\x80\xFD[=\x91Pa\x07\x82V[\x81a\x07\xB6\x91a8\nV[a\x07pW\x81_a\x06\xB8V[`@Q=\x85\x82>=\x90\xFD[\x92PP` \x82=` \x11a\x07\xF9W[\x81a\x07\xE8` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x82\x91Q_a\x06>V[=\x91Pa\x07\xDBV[a\x08#\x91P` =` \x11a\x08)W[a\x08\x1B\x81\x83a8\nV[\x81\x01\x90a=\x88V[_a\x05_V[P=a\x08\x11V[\x92PP` \x82=` \x11a\x08`W[\x81a\x08L` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x90Q\x82\x91\x90` a\x05\xA2V[=\x91Pa\x08?V[\x92PP` \x82=` \x11a\x08\x98W[\x81a\x08\x84` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x90Q\x82\x91\x90` a\x05\x06V[=\x91Pa\x08wV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@\x80Qa\x08\xC0\x82\x82a8\nV[`\x01\x81R`\x1F\x19\x82\x01\x90\x816` \x83\x017`\na\x08\xDC\x82a8EV[R\x82Q\x91a\x08\xEA\x84\x84a8\nV[`\x01\x83R6` \x84\x017\x7F\x9C\"\xFF_!\xF0\xB8\x1B\x11>c\xF7\xDBm\xA9O\xED\xEF\x11\xB2\x11\x9B@\x88\xB8\x96d\xFB\x9A<\xB6Xa\t\x1E\x83a8EV[Ra\t'a8\xD9V[\x83Q` \x80\x82\x01R`\n`@\x82\x01R\x7Ftest batch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01Ra\ts\x81`\x80\x81\x01[\x03`\x1F\x19\x81\x01\x83R\x82a8\nV[a\t|\x82a8EV[Ra\t\x86\x81a8EV[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C-W\x84Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x85\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0CFW\x90\x86\x91a\x0C\x93W[PP`\x04`\x01`\x01`\xA0\x1B\x03`\"T\x16` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x87Q\x93\x84\x80\x92\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x0C\x89W\x87\x92a\x0CPW[P\x85Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Ra\n\xAC\x81`d\x81\x01a\teV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C-W\x85a\x0B\x06\x91\x86Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6`V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0CFW\x90\x86\x91a\x0C1W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C-Wa\x0B\x82\x93\x86\x80\x94\x87Q\x96\x87\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x03\x92Z\xF1\x80\x15a\x0C!W\x90\x83\x91a\x0C\x0CW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x03WPa\x04?WP\xF3[Q=\x84\x82>=\x90\xFD[\x81a\x0C\x16\x91a8\nV[a\x07pW\x81_a\x0B\x94V[PPQ\x90=\x90\x82>=\x90\xFD[\x85\x80\xFD[\x81a\x0C;\x91a8\nV[a\x04[W\x84_a\x0B.V[\x85Q=\x88\x82>=\x90\xFD[\x96P\x90P` \x86=` \x11a\x0C\x81W[\x81a\x0Cm` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x94Q\x86\x95a\n\xACa\n^V[=\x91Pa\x0C`V[\x86Q=\x89\x82>=\x90\xFD[\x81a\x0C\x9D\x91a8\nV[a\x04[W\x84_a\n\x04V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` a\x0C\xC3a=\xA0V[`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x19Ta\x0C\xEA\x81a8-V[\x91a\x0C\xF8`@Q\x93\x84a8\nV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\r:W`@Q\x80a\x02;\x87\x82a6\xDAV[`\x01` \x81\x92a\rI\x85a9\xE1V[\x81R\x01\x92\x01\x92\x01\x91\x90a\r%V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1CTa\rt\x81a8-V[\x91a\r\x82`@Q\x93\x84a8\nV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\r\xC4W`@Q\x80a\x02;\x87\x82a7WV[`\x02` `\x01\x92`@Qa\r\xD7\x81a7\xEEV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\r\xEF\x85\x87\x01a:\xE4V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\r\xAFV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@\x80Q\x90a\x0E\"\x81\x83a8\nV[`\x01\x82R`\x1F\x19\x81\x01\x91\x826` \x83\x017`\na\x0E>\x82a8EV[R\x81Q\x92a\x0EL\x83\x85a8\nV[`\x01\x84R6` \x85\x017\x7F\x9C\"\xFF_!\xF0\xB8\x1B\x11>c\xF7\xDBm\xA9O\xED\xEF\x11\xB2\x11\x9B@\x88\xB8\x96d\xFB\x9A<\xB6Xa\x0E\x80\x84a8EV[Ra\x0E\x89a8\xD9V[\x82Q` \x80\x82\x01R`\n`@\x82\x01R\x7Ftest batch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01Ra\x0E\xCB\x81`\x80\x81\x01a\teV[a\x0E\xD4\x82a8EV[Ra\x0E\xDE\x81a8EV[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04[W\x82Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x85\x81\x80a\x0FF`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x10\xA1W\x90\x86\x91a\x10\xAFW[PPa\x0Fy\x82a8EV[Qa\x0F\x83\x83a8EV[Q\x84Q\x91\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3\x88\x80\xA3`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x10\xABW\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x85\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x10\xA1W\x90\x86\x91a\x10\x8CW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C-Wa\x10y\x94\x86\x80\x94\x86Q\x97\x88\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x03\x92Z\xF1\x90\x81\x15a\x0C\x03WPa\x04?WP\xF3[\x81a\x10\x96\x91a8\nV[a\x04[W\x84_a\x10%V[\x84Q=\x88\x82>=\x90\xFD[\x86\x80\xFD[\x81a\x10\xB9\x91a8\nV[a\x04[W\x84_a\x0FnV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@Qa\x11\x1C\x81a\te` \x82\x01``\x90` \x81R`\n` \x82\x01R\x7Ftest batch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81\x80a\x11\x84`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07\xC1W\x83\x91a\x12\xF5W[PP`@Q`\n\x80\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3\x85\x80\xA3`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07\xC1W\x83\x91a\x12\xE0W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x12\xDCWa\x04.\x83\x92\x91\x83\x92`@Q\x94\x85\x80\x94\x81\x93\x7F\xB1\xD0`E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\n`\x04\x84\x01R\x7F\x9C\"\xFF_!\xF0\xB8\x1B\x11>c\xF7\xDBm\xA9O\xED\xEF\x11\xB2\x11\x9B@\x88\xB8\x96d\xFB\x9A<\xB6X`$\x84\x01R```D\x84\x01R`d\x83\x01\x90a6`V[PP\xFD[\x81a\x12\xEA\x91a8\nV[a\x07pW\x81_a\x12RV[\x81a\x12\xFF\x91a8\nV[a\x07pW\x81_a\x11\xACV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x14RW[PP`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`@Q\x91a\x0B\x86\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14%W\x91``\x93\x91\x85\x93a>\xF6\x859\x82R` \x82\x01R\x84`@\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x19W\x80\xF3[`@Q\x90=\x90\x82>=\x90\xFD[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a\x14\\\x91a8\nV[a\x01\xA7W\x80_a\x13\xB9V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1DTa\x14\x84\x81a8-V[\x91a\x14\x92`@Q\x93\x84a8\nV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x14\xD4W`@Q\x80a\x02;\x87\x82a7WV[`\x02` `\x01\x92`@Qa\x14\xE7\x81a7\xEEV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x14\xFF\x85\x87\x01a:\xE4V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x14\xBFV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x17\xF2W[P`\x04`\x01`\x01`\xA0\x1B\x03`\"T\x16` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x07\xC1W\x83\x92a\x17\xB9W[P`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Ra\x16G\x81`d\x81\x01a\teV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW\x81a\x16\xA2\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6`V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x17\xA4W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07pW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\x0F\xFD\x13\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`d`\x04\x84\x01RZ\xF1\x80\x15a\x04PWa\x17\x8FW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x04?WP\xF3[\x81a\x17\x99\x91a8\nV[a\x01\xA7W\x80_a\x17!V[\x81a\x17\xAE\x91a8\nV[a\x01\xA7W\x80_a\x16\xC7V[\x92P\x90P` \x82=` \x11a\x17\xEAW[\x81a\x17\xD6` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x90Q\x82\x91a\x16Ga\x15\xF8V[=\x91Pa\x17\xC9V[\x81a\x17\xFC\x91a8\nV[a\x01\xA7W\x80_a\x15\x9EV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x1D\rW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x07\xC1W\x83\x92a\x1C\xD6W[P\x80;\x15a\x12\xDCW\x82\x91\x82\x91`@Q\x80\x94\x81\x80\x94\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04` \x81\x84\x01`@\x81\x01\x93\x81R\x01R\x03\x92Z\xF1\x80\x15a\x04PWa\x1C\xC1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x1C\x8CW[P` `@Q\x80\x80\x93\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x84\x81\x84\x01`@\x81\x01\x93\x81R\x01R\x03\x81\x85Z\xFA\x80\x15a\x07\xC1Wa\x19\xDB\x91\x84\x91a\x08\x01WPa>yV[`@Q\x90\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x07\xC1W\x83\x92a\x1CUW[P\x80;\x15a\x12\xDCW\x82\x91\x82\x91`@Q\x80\x94\x81\x80\x94\x7F\xD5Gt\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04` \x81\x84\x01`@\x81\x01\x93\x81R\x01R\x03\x92Z\xF1\x80\x15a\x04PWa\x1C@W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x07\xC1W\x83\x91a\x1C\tW[P` \x90`@Q\x92\x83\x80\x80\x93\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x86\x81\x84\x01`@\x81\x01\x93\x81R\x01R\x03\x91Z\xFA\x90\x81\x15a\x04PW\x82\x91a\x1B\xEAW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04PWa\x17\x8FWPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x04?WP\xF3[a\x1C\x03\x91P` =` \x11a\x08)Wa\x08\x1B\x81\x83a8\nV[_a\x1B\x0CV[\x92PP` \x82=` \x11a\x1C8W[\x81a\x1C%` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W` \x83\x92Q\x90a\x1A\xBCV[=\x91Pa\x1C\x18V[\x81a\x1CJ\x91a8\nV[a\x01\xA7W\x80_a\x1AoV[\x92P\x90P` \x82=` \x11a\x1C\x84W[\x81a\x1Cr` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x82\x91Q\x90_a\x1A\x19V[=\x91Pa\x1CeV[\x92PP` \x82=` \x11a\x1C\xB9W[\x81a\x1C\xA8` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x82\x91Q_a\x19\x85V[=\x91Pa\x1C\x9BV[\x81a\x1C\xCB\x91a8\nV[a\x01\xA7W\x80_a\x198V[\x92P\x90P` \x82=` \x11a\x1D\x05W[\x81a\x1C\xF3` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x82\x91Q\x90_a\x18\xE2V[=\x91Pa\x1C\xE6V[\x81a\x1D\x17\x91a8\nV[a\x01\xA7W\x80_a\x18\x94V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAdmin address cannot be 0\0\0\0\0\0\0\0`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x1E(W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`#T\x90`@Q\x91a\x0B\x86\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14%W\x91``\x93\x91\x85\x93a>\xF6\x859\x86\x83R` \x83\x01R`@\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x19W\x80\xF3[\x81a\x1E2\x91a8\nV[a\x01\xA7W\x80_a\x1D\xD1V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1ATa\x1EZ\x81a8-V[\x91a\x1Eh`@Q\x93\x84a8\nV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x1E\xAAW`@Q\x80a\x02;\x87\x82a6\xDAV[`\x01` \x81\x92a\x1E\xB9\x85a9\xE1V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x1E\x95V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1BTa\x1E\xE4\x81a8-V[a\x1E\xF1`@Q\x91\x82a8\nV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x1F\xC9W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x1F^WPPPP\x03\x90\xF3[\x91\x93` a\x1F\xB9\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x1F\xA9\x83Q`@\x84R`@\x84\x01\x90a6`V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra6\x85V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x1FOV[`\x02` `\x01\x92`@Qa\x1F\xDC\x81a7\xEEV[a\x1F\xE5\x86a9\xE1V[\x81Ra\x1F\xF2\x85\x87\x01a:\xE4V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1F!V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FManager address cannot be 0\0\0\0\0\0`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa!\nW[PP`\x01`\x01`\xA0\x1B\x03` T\x16`#T\x90`@Q\x91a\x0B\x86\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14%W\x91``\x93\x91\x85\x93a>\xF6\x859\x82R\x85` \x83\x01R`@\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x19W\x80\xF3[\x81a!\x14\x91a8\nV[a\x01\xA7W\x80_a \xB3V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@Qa!?`\x80\x82a8\nV[`\x03\x81R``\x90\x816` \x83\x017`\na!X\x82a8EV[R`\x0Ba!d\x82a8\x7FV[R`\x0Ca!p\x82a8\x8FV[R`@Q\x91a!\x80`\x80\x84a8\nV[`\x03\x83R6` \x84\x017\x7Fm%_\xC39\x0E\xE6\xB4\x11\x91\xDA1YX\xB7\xD6\xA1\xE5\xB1y\x04\xCCv\x83U\x8F\x98\xAC\xC5yw\xB4a!\xB4\x83a8EV[R\x7FM\xA42\xF1\xEC\xD4\xC0\xAC\x02\x8E\xBD\xE3\xA3\xF7\x85\x10\xA2\x1DT\x08{\x16\x15\x90\xA60\x80\xD3;p+\x8Da!\xDF\x83a8\x7FV[R\x7F EX\x07n\xFB B\xEB\xC9\xB04\xAA\xB3m\x85\xD6r\xD8\xAC\x1F\xA8\t(\x8D\xA5\xB4S\xA4qJ\xAEa\"\n\x83a8\x8FV[Ra\"\x13a8\x9FV[`@Qa\"Y\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a\"b\x82a8EV[Ra\"l\x81a8EV[P`@Qa\"\xB3\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a\"\xBC\x82a8\x7FV[Ra\"\xC6\x81a8\x7FV[P`@Qa#\r\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a#\x16\x82a8\x8FV[Ra# \x81a8\x8FV[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84\x81\x80a#\x89`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a$\xBDW[PPa#\xBC\x82a8EV[Qa#\xC6\x83a8\x8FV[Q`@Q\x91\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3\x87\x80\xA3`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C-W\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a\x04_WPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x04[Wa\x04.\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x81a$\xC7\x91a8\nV[a\x04tW\x83_a#\xB1V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa&\xC9W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07pW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\x0F\xFD\x13\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`d`\x04\x84\x01RZ\xF1\x80\x15a\x04PWa&\xB4W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xCC\xE5\xD2\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04PW\x82\x91a&\x7FW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07pW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`d`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04PWa\x04?WP\xF3[\x91PP` \x81=` \x11a&\xACW[\x81a&\x9B` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x81\x90Q_a&\x07V[=\x91Pa&\x8EV[\x81a&\xBE\x91a8\nV[a\x01\xA7W\x80_a%\xB9V[\x81a&\xD3\x91a8\nV[a\x01\xA7W\x80_a%_V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a'dWa\x02;\x85a\x02/\x81\x87\x03\x82a8\nV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a'MV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a'\xE2Wa\x02;\x85a\x02/\x81\x87\x03\x82a8\nV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a'\xCBV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@Qa(!`\x80\x82a8\nV[`\x03\x81R``6` \x83\x017`\na(8\x82a8EV[R`\x0Ba(D\x82a8\x7FV[R`\x0Ca(P\x82a8\x8FV[R`@Q\x90a(```\x83a8\nV[`\x02\x82R`@6` \x84\x017\x7Fm%_\xC39\x0E\xE6\xB4\x11\x91\xDA1YX\xB7\xD6\xA1\xE5\xB1y\x04\xCCv\x83U\x8F\x98\xAC\xC5yw\xB4a(\x96\x83a8EV[R\x7FM\xA42\xF1\xEC\xD4\xC0\xAC\x02\x8E\xBD\xE3\xA3\xF7\x85\x10\xA2\x1DT\x08{\x16\x15\x90\xA60\x80\xD3;p+\x8Da(\xC1\x83a8\x7FV[Ra(\xCAa8\x9FV[`@Qa)\x10\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a)\x19\x82a8EV[Ra)#\x81a8EV[P`@Qa)j\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a)s\x82a8\x7FV[Ra)}\x81a8\x7FV[P`@Qa)\xC4\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a)\xCD\x82a8\x8FV[Ra)\xD7\x81a8\x8FV[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04[W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a+QW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FArray lengths must be equal\0\0\0\0\0`D\x82\x01R\x84\x81\x80`d\x81\x01[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04yW\x85\x91a\x04_WPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x04[Wa\x04.\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\x04\x84 \xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a9\x13V[\x81a+[\x91a8\nV[a\x04tW\x83_a*VV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W\x80`@Qa+\xBE\x81a\te` \x82\x01``\x90` \x81R`\n` \x82\x01R\x7Ftest batch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12\xDCW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07\xC1W\x83\x91a.\xC8W[PP`\x04`\x01`\x01`\xA0\x1B\x03`\"T\x16` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xEC\x87b\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a.\xBDW\x84\x92a.\x84W[P`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Ra,\xE6\x81`d\x81\x01a\teV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12\xDCW\x82a-A\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6`V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07\xC1W\x83\x91a.oW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x12\xDCWa-\xF3\x83\x92\x91\x83\x92`@Q\x94\x85\x80\x94\x81\x93\x7F\xB1\xD0`E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\n`\x04\x84\x01R\x7F\x9C\"\xFF_!\xF0\xB8\x1B\x11>c\xF7\xDBm\xA9O\xED\xEF\x11\xB2\x11\x9B@\x88\xB8\x96d\xFB\x9A<\xB6X`$\x84\x01R```D\x84\x01R`d\x83\x01\x90a6`V[\x03\x92Z\xF1\x80\x15a\x04PWa\x17\x8FWPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xA7W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04PWa\x04?WP\xF3[\x81a.y\x91a8\nV[a\x07pW\x81_a-iV[\x93P\x90P` \x83=` \x11a.\xB5W[\x81a.\xA1` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0W\x91Q\x83\x92a,\xE6a,\x97V[=\x91Pa.\x94V[`@Q=\x86\x82>=\x90\xFD[\x81a.\xD2\x91a8\nV[a\x07pW\x81_a,<V[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x1ETa.\xFA\x81a8-V[a/\x07`@Q\x91\x82a8\nV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a0HW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a/sW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a/\xFFWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a/fV[\x90\x91\x92\x93\x94` \x80a0;\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa6`V[\x97\x01\x95\x01\x93\x92\x91\x01a/\xDBV[`@Qa0T\x81a7\xEEV[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta0p\x81a8-V[\x91a0~`@Q\x93\x84a8\nV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a0\xB4WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a/7V[`\x01` \x81\x92a0\xC3\x86a9\xE1V[\x81R\x01\x93\x01\x91\x01\x90\x91a0\x8EV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a1ZWa\x02;\x85a\x02/\x81\x87\x03\x82a8\nV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a1CV[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x01\xA7W\x80`\x03\x196\x01\x12a\x01\xA7W`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"U`{`#U`@Qa\x0B\x86\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a2\xD0W``\x91\x83\x91a>\xF6\x839`\x01\x81R`\x02` \x82\x01R`{`@\x82\x01R\x03\x01\x90\x82\xF0\x80\x15a2\xC3W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80\xF3[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x07\xA0W_`\x03\x196\x01\x12a\x07\xA0W`@Q\x90a3\x1D`\x80\x83a8\nV[`\x03\x82R``\x806` \x85\x017`\na35\x84a8EV[R`\x0Ba3A\x84a8\x7FV[R`\x0Ca3M\x84a8\x8FV[R`@Q\x90a3]`\x80\x83a8\nV[`\x03\x82R6` \x83\x017\x7Fm%_\xC39\x0E\xE6\xB4\x11\x91\xDA1YX\xB7\xD6\xA1\xE5\xB1y\x04\xCCv\x83U\x8F\x98\xAC\xC5yw\xB4a3\x91\x82a8EV[R\x7FM\xA42\xF1\xEC\xD4\xC0\xAC\x02\x8E\xBD\xE3\xA3\xF7\x85\x10\xA2\x1DT\x08{\x16\x15\x90\xA60\x80\xD3;p+\x8Da3\xBC\x82a8\x7FV[R\x7F EX\x07n\xFB B\xEB\xC9\xB04\xAA\xB3m\x85\xD6r\xD8\xAC\x1F\xA8\t(\x8D\xA5\xB4S\xA4qJ\xAEa3\xE7\x82a8\x8FV[R`@Q\x92``a3\xF8\x81\x86a8\nV[`\x02\x85R`\x1F\x19\x01_[\x81\x81\x10a5\xE0WPP`@Qa4Q\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a4Z\x85a8EV[Ra4d\x84a8EV[P`@Qa4\xAB\x81a\te` \x82\x01``\x90` \x81R`\x0C` \x82\x01R\x7Ftest batch 2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a4\xB4\x85a8\x7FV[Ra4\xBE\x84a8\x7FV[P`\x01`\x01`\xA0\x1B\x03`!T\x16\x93sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xA0W`@Q\x94\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01R_\x85`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a5\xD5Wa5\xBFW[\x83\x94Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04tW`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FArray lengths must be equal\0\0\0\0\0`D\x82\x01R\x84\x81\x80`d\x81\x01a*\xD5V[\x92P\x92_a5\xCC\x91a8\nV[_\x91\x83\x90a5;V[`@Q=_\x82>=\x90\xFD[\x80``` \x80\x93\x89\x01\x01R\x01a4\x02V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a6AWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a64V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a6\xA2WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6\x95V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7\x0CWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a7H\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa6`V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a6\xFDV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7\x89WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a7\xDF\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a6\x85V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a7zV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5\xF1W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5\xF1W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a5\xF1W`\x05\x1B` \x01\x90V[\x80Q\x15a8RW` \x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80Q`\x01\x10\x15a8RW`@\x01\x90V[\x80Q`\x02\x10\x15a8RW``\x01\x90V[`@Q`\x80\x91\x90a8\xB0\x83\x82a8\nV[`\x03\x81R\x91`\x1F\x19\x01\x82_[\x82\x81\x10a8\xC8WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a8\xBCV[`@\x80Q\x90\x91\x90a8\xEA\x83\x82a8\nV[`\x01\x81R\x91`\x1F\x19\x01\x82_[\x82\x81\x10a9\x02WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a8\xF6V[\x90``\x82\x01``\x83R\x81Q\x80\x91R` `\x80\x84\x01\x92\x01\x90_[\x81\x81\x10a9\xCBWPPP\x81\x81\x03` \x83\x01R` \x80\x84Q\x92\x83\x81R\x01\x93\x01\x90_[\x81\x81\x10a9\xB5WPPP`@\x81\x83\x03\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a9\x88WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a9\xA6\x83`\x1F\x19\x86`\x01\x96\x03\x01\x87R\x89Qa6`V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a9yV[\x82Q\x85R` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a9MV[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a9,V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a:\xDAW[` \x85\x10\x84\x14a:\xADW\x84\x87R\x86\x93\x90\x81\x15a:mWP`\x01\x14a:)W[Pa:'\x92P\x03\x83a8\nV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a:QWPP\x90` a:'\x92\x82\x01\x01_a:\x1AV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a:8V[` \x93Pa:'\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a:\x1AV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a9\xFBV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a<\xFBWa:'\x94T\x91\x81\x81\x10a<\xC5W[\x81\x81\x10a<\x8FW[\x81\x81\x10a<YW[\x81\x81\x10a<#W[\x81\x81\x10a;\xEDW[\x81\x81\x10a;\xB7W[\x81\x81\x10a;\x82W[\x10a;UW[P\x03\x83a8\nV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a;MV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a;GV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a;?V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a;7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a;/V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a;'V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a;\x1FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a;\x17V[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a:\xFFV[\x90\x81` \x91\x03\x12a\x07\xA0WQ\x80\x15\x15\x81\x03a\x07\xA0W\x90V[`\x08T`\xFF\x16\x80\x15a=\xAFW\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a5\xD5W_\x91a>GW[P\x15\x15\x90V[\x90P` \x81=` \x11a>qW[\x81a>b` \x93\x83a8\nV[\x81\x01\x03\x12a\x07\xA0WQ_a>AV[=\x91Pa>UV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xA0W`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5\xD5Wa>\xEBWPV[_a:'\x91a8\nV\xFE`\xA04a\x01zW`\x1Fa\x0B\x868\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01~W\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\x01zWa\0G\x81a\x01\x92V[`@a\0U` \x84\x01a\x01\x92V[\x92\x01Q\x91`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x015W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\0\xF0W\x82\x15a\0\xABWa\0\x89a\0\x8F\x92a\x01\xA6V[Pa\x02\x1CV[P`\x80R`@Qa\x08v\x90\x81a\x02\xB0\x829`\x80Q\x81a\x01\0\x01R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FManager address cannot be 0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAdmin address cannot be 0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01zWV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x0Bf_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02\x17W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x0Bf_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x0B&_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x0BF_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02\x17W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x0BF_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08\x90_Q` a\x0B&_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\x04\xECWP\x80c\x04\x84 \xA7\x14a\x036W\x80c\x0F\xFD\x13\x80\x14a\x03\x15W\x80c$\x8A\x9C\xA3\x14a\x02\xE3W\x80c//\xF1]\x14a\x02\xB4W\x80c6V\x8A\xBE\x14a\x02XW\x80c\x91\xD1HT\x14a\x02\x0FW\x80c\xA2\x17\xFD\xDF\x14a\x01\xF5W\x80c\xB1\xD0`E\x14a\x01vW\x80c\xCC\xE5\xD2\xF6\x14a\x01YW\x80c\xD5Gt\x1F\x14a\x01#W\x80c\xD8x\x13B\x14a\0\xE9Wc\xEC\x87b\x1C\x14a\0\xABW_\x80\xFD[4a\0\xE5W_`\x03\x196\x01\x12a\0\xE5W` `@Q\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08\x81R\xF3[_\x80\xFD[4a\0\xE5W_`\x03\x196\x01\x12a\0\xE5W` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[4a\0\xE5Wa\x01Wa\x0146a\x05\xBBV[\x90a\x01Ra\x01M\x82_R_` R`\x01`@_ \x01T\x90V[a\x06vV[a\x07\xAEV[\0[4a\0\xE5W_`\x03\x196\x01\x12a\0\xE5W` `\x01T`@Q\x90\x81R\xF3[4a\0\xE5W```\x03\x196\x01\x12a\0\xE5W`\x045`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5W6`#\x82\x01\x12\x15a\0\xE5W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5W6\x91\x01`$\x01\x11a\0\xE5Wa\x01\xCEa\x05\xEEV[\x80\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3_\x80\xA3\0[4a\0\xE5W_`\x03\x196\x01\x12a\0\xE5W` `@Q_\x81R\xF3[4a\0\xE5Wa\x02\x1D6a\x05\xBBV[\x90_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\0\xE5Wa\x02f6a\x05\xBBV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x02\x8CWa\x01W\x91a\x07\xAEV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\0\xE5Wa\x01Wa\x02\xC56a\x05\xBBV[\x90a\x02\xDEa\x01M\x82_R_` R`\x01`@_ \x01T\x90V[a\x06\xDCV[4a\0\xE5W` `\x03\x196\x01\x12a\0\xE5W` a\x03\r`\x045_R_` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\0\xE5W` `\x03\x196\x01\x12a\0\xE5Wa\x03.a\x05\xEEV[`\x045`\x01U\0[4a\0\xE5W```\x03\x196\x01\x12a\0\xE5W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5Wa\x03g\x906\x90`\x04\x01a\x05\x8AV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5Wa\x03\x87\x906\x90`\x04\x01a\x05\x8AV[\x90P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE5Wa\x03\xA9\x906\x90`\x04\x01a\x05\x8AV[\x90Pa\x03\xB3a\x05\xEEV[\x81\x83\x14\x91\x82a\x04\xE2W[PP\x15a\x04\x84W\x80\x15a\x04*W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x80\x82\x11a\x04WW\x81\x10\x15a\x04*W`\x05\x1B\x81\x015\x905\x7F\xFB?\x94,M\x87q\xACZ6/X\xBD8\x04\xE0u\x9E\xBDR\x81\x07#\x88J|\xDBS\x1D\x83\xB6\xF3_\x80\xA3\0[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FArray lengths must be equal\0\0\0\0\0`D\x82\x01R\xFD[\x14\x90P\x83\x80a\x03\xBDV[4a\0\xE5W` `\x03\x196\x01\x12a\0\xE5W`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\0\xE5W\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\x05`W[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\x05YV[\x91\x81`\x1F\x84\x01\x12\x15a\0\xE5W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xE5W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\0\xE5WV[`\x03\x19`@\x91\x01\x12a\0\xE5W`\x045\x90`$5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\0\xE5W\x90V[3_\x90\x81R\x7F\xE8E\x08\xF2\xC7\xFA\x9C5\x11Ft\x8B0%\xCBx\xB4]\xF3}\x86\x8EH\xC6\xA7Q\x02\xFE\xCD\xEE\xE6E` R`@\x90 T`\xFF\x16\x15a\x06&WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x06\xADWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x07\xA8W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x07\xA8W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xE8E\x08\xF2\xC7\xFA\x9C5\x11Ft\x8B0%\xCBx\xB4]\xF3}\x86\x8EH\xC6\xA7Q\x02\xFE\xCD\xEE\xE6E\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EpochRangeProcessed(uint256,uint256)` and selector `0xfb3f942c4d8771ac5a362f58bd3804e0759ebd52810723884a7cdb531d83b6f3`.
```solidity
event EpochRangeProcessed(uint256 indexed startEpochNumber, uint256 indexed endEpochNumber);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EpochRangeProcessed {
        #[allow(missing_docs)]
        pub startEpochNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub endEpochNumber: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EpochRangeProcessed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "EpochRangeProcessed(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8, 63u8, 148u8, 44u8, 77u8, 135u8, 113u8, 172u8, 90u8, 54u8, 47u8,
                88u8, 189u8, 56u8, 4u8, 224u8, 117u8, 158u8, 189u8, 82u8, 129u8, 7u8,
                35u8, 136u8, 74u8, 124u8, 219u8, 83u8, 29u8, 131u8, 182u8, 243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    startEpochNumber: topics.1,
                    endEpochNumber: topics.2,
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
                    self.startEpochNumber.clone(),
                    self.endEpochNumber.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.startEpochNumber);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.endEpochNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EpochRangeProcessed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EpochRangeProcessed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EpochRangeProcessed) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `appchainId()` and selector `0xd8781342`.
```solidity
function appchainId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`appchainId()`](appchainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainIdReturn {
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
            impl ::core::convert::From<appchainIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: appchainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainIdCall {
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
            impl ::core::convert::From<appchainIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: appchainIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for appchainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "appchainId()";
            const SELECTOR: [u8; 4] = [216u8, 120u8, 19u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: appchainIdReturn = r.into();
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
                        let r: appchainIdReturn = r.into();
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
    /**Function with signature `manager()` and selector `0x481c6a75`.
```solidity
function manager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct managerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`manager()`](managerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct managerReturn {
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
            impl ::core::convert::From<managerCall> for UnderlyingRustTuple<'_> {
                fn from(value: managerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for managerCall {
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
            impl ::core::convert::From<managerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: managerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for managerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for managerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "manager()";
            const SELECTOR: [u8; 4] = [72u8, 28u8, 106u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: managerReturn = r.into();
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
                        let r: managerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nonAuthorized()` and selector `0x1ddc2d10`.
```solidity
function nonAuthorized() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonAuthorizedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nonAuthorized()`](nonAuthorizedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonAuthorizedReturn {
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
            impl ::core::convert::From<nonAuthorizedCall> for UnderlyingRustTuple<'_> {
                fn from(value: nonAuthorizedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nonAuthorizedCall {
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
            impl ::core::convert::From<nonAuthorizedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nonAuthorizedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nonAuthorizedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nonAuthorizedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonAuthorized()";
            const SELECTOR: [u8; 4] = [29u8, 220u8, 45u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: nonAuthorizedReturn = r.into();
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
                        let r: nonAuthorizedReturn = r.into();
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
    /**Function with signature `syndicateStorage()` and selector `0x297feeb2`.
```solidity
function syndicateStorage() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct syndicateStorageCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`syndicateStorage()`](syndicateStorageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct syndicateStorageReturn {
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
            impl ::core::convert::From<syndicateStorageCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: syndicateStorageCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for syndicateStorageCall {
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
            impl ::core::convert::From<syndicateStorageReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: syndicateStorageReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for syndicateStorageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for syndicateStorageCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "syndicateStorage()";
            const SELECTOR: [u8; 4] = [41u8, 127u8, 238u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: syndicateStorageReturn = r.into();
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
                        let r: syndicateStorageReturn = r.into();
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Function with signature `testConstructorZeroAdmin()` and selector `0x8d3a83b9`.
```solidity
function testConstructorZeroAdmin() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorZeroAdminCall;
    ///Container type for the return parameters of the [`testConstructorZeroAdmin()`](testConstructorZeroAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorZeroAdminReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testConstructorZeroAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorZeroAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testConstructorZeroAdminCall {
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
            impl ::core::convert::From<testConstructorZeroAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorZeroAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testConstructorZeroAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testConstructorZeroAdminReturn {
            fn _tokenize(
                &self,
            ) -> <testConstructorZeroAdminCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testConstructorZeroAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testConstructorZeroAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testConstructorZeroAdmin()";
            const SELECTOR: [u8; 4] = [141u8, 58u8, 131u8, 185u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testConstructorZeroAdminReturn::_tokenize(ret)
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
    /**Function with signature `testConstructorZeroAppChainId()` and selector `0x96514025`.
```solidity
function testConstructorZeroAppChainId() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorZeroAppChainIdCall;
    ///Container type for the return parameters of the [`testConstructorZeroAppChainId()`](testConstructorZeroAppChainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorZeroAppChainIdReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testConstructorZeroAppChainIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorZeroAppChainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testConstructorZeroAppChainIdCall {
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
            impl ::core::convert::From<testConstructorZeroAppChainIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorZeroAppChainIdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testConstructorZeroAppChainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testConstructorZeroAppChainIdReturn {
            fn _tokenize(
                &self,
            ) -> <testConstructorZeroAppChainIdCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testConstructorZeroAppChainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testConstructorZeroAppChainIdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testConstructorZeroAppChainId()";
            const SELECTOR: [u8; 4] = [150u8, 81u8, 64u8, 37u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testConstructorZeroAppChainIdReturn::_tokenize(ret)
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
    /**Function with signature `testConstructorZeroManager()` and selector `0x59ebf049`.
```solidity
function testConstructorZeroManager() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorZeroManagerCall;
    ///Container type for the return parameters of the [`testConstructorZeroManager()`](testConstructorZeroManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorZeroManagerReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testConstructorZeroManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorZeroManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testConstructorZeroManagerCall {
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
            impl ::core::convert::From<testConstructorZeroManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorZeroManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testConstructorZeroManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testConstructorZeroManagerReturn {
            fn _tokenize(
                &self,
            ) -> <testConstructorZeroManagerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testConstructorZeroManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testConstructorZeroManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testConstructorZeroManager()";
            const SELECTOR: [u8; 4] = [89u8, 235u8, 240u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testConstructorZeroManagerReturn::_tokenize(ret)
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
    /**Function with signature `testRoleManagement()` and selector `0x8d3d6c99`.
```solidity
function testRoleManagement() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRoleManagementCall;
    ///Container type for the return parameters of the [`testRoleManagement()`](testRoleManagementCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRoleManagementReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testRoleManagementCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRoleManagementCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRoleManagementCall {
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
            impl ::core::convert::From<testRoleManagementReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRoleManagementReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRoleManagementReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testRoleManagementReturn {
            fn _tokenize(
                &self,
            ) -> <testRoleManagementCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testRoleManagementCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testRoleManagementReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testRoleManagement()";
            const SELECTOR: [u8; 4] = [141u8, 61u8, 108u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testRoleManagementReturn::_tokenize(ret)
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
    /**Function with signature `testSave()` and selector `0x96e181cf`.
```solidity
function testSave() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveCall;
    ///Container type for the return parameters of the [`testSave()`](testSaveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSaveCall> for UnderlyingRustTuple<'_> {
                fn from(value: testSaveCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for testSaveCall {
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
            impl ::core::convert::From<testSaveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: testSaveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for testSaveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSaveReturn {
            fn _tokenize(
                &self,
            ) -> <testSaveCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSaveCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSaveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSave()";
            const SELECTOR: [u8; 4] = [150u8, 225u8, 129u8, 207u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSaveReturn::_tokenize(ret)
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
    /**Function with signature `testSaveForMany()` and selector `0x53882c6e`.
```solidity
function testSaveForMany() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyCall;
    ///Container type for the return parameters of the [`testSaveForMany()`](testSaveForManyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSaveForManyCall> for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for testSaveForManyCall {
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
            impl ::core::convert::From<testSaveForManyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSaveForManyReturn {
            fn _tokenize(
                &self,
            ) -> <testSaveForManyCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSaveForManyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSaveForManyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSaveForMany()";
            const SELECTOR: [u8; 4] = [83u8, 136u8, 44u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSaveForManyReturn::_tokenize(ret)
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
    /**Function with signature `testSaveForManyArrayLengthMismatch1()` and selector `0x3b7b325b`.
```solidity
function testSaveForManyArrayLengthMismatch1() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyArrayLengthMismatch1Call;
    ///Container type for the return parameters of the [`testSaveForManyArrayLengthMismatch1()`](testSaveForManyArrayLengthMismatch1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyArrayLengthMismatch1Return {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSaveForManyArrayLengthMismatch1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyArrayLengthMismatch1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyArrayLengthMismatch1Call {
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
            impl ::core::convert::From<testSaveForManyArrayLengthMismatch1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyArrayLengthMismatch1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyArrayLengthMismatch1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSaveForManyArrayLengthMismatch1Return {
            fn _tokenize(
                &self,
            ) -> <testSaveForManyArrayLengthMismatch1Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSaveForManyArrayLengthMismatch1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSaveForManyArrayLengthMismatch1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSaveForManyArrayLengthMismatch1()";
            const SELECTOR: [u8; 4] = [59u8, 123u8, 50u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSaveForManyArrayLengthMismatch1Return::_tokenize(ret)
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
    /**Function with signature `testSaveForManyArrayLengthMismatch2()` and selector `0x05a56f80`.
```solidity
function testSaveForManyArrayLengthMismatch2() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyArrayLengthMismatch2Call;
    ///Container type for the return parameters of the [`testSaveForManyArrayLengthMismatch2()`](testSaveForManyArrayLengthMismatch2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyArrayLengthMismatch2Return {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSaveForManyArrayLengthMismatch2Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyArrayLengthMismatch2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyArrayLengthMismatch2Call {
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
            impl ::core::convert::From<testSaveForManyArrayLengthMismatch2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyArrayLengthMismatch2Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyArrayLengthMismatch2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSaveForManyArrayLengthMismatch2Return {
            fn _tokenize(
                &self,
            ) -> <testSaveForManyArrayLengthMismatch2Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSaveForManyArrayLengthMismatch2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSaveForManyArrayLengthMismatch2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSaveForManyArrayLengthMismatch2()";
            const SELECTOR: [u8; 4] = [5u8, 165u8, 111u8, 128u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSaveForManyArrayLengthMismatch2Return::_tokenize(ret)
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
    /**Function with signature `testSaveForManyEmptyArrays()` and selector `0xca505fe2`.
```solidity
function testSaveForManyEmptyArrays() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyEmptyArraysCall;
    ///Container type for the return parameters of the [`testSaveForManyEmptyArrays()`](testSaveForManyEmptyArraysCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyEmptyArraysReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSaveForManyEmptyArraysCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyEmptyArraysCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyEmptyArraysCall {
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
            impl ::core::convert::From<testSaveForManyEmptyArraysReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyEmptyArraysReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyEmptyArraysReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSaveForManyEmptyArraysReturn {
            fn _tokenize(
                &self,
            ) -> <testSaveForManyEmptyArraysCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSaveForManyEmptyArraysCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSaveForManyEmptyArraysReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSaveForManyEmptyArrays()";
            const SELECTOR: [u8; 4] = [202u8, 80u8, 95u8, 226u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSaveForManyEmptyArraysReturn::_tokenize(ret)
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
    /**Function with signature `testSaveForManyNonManager()` and selector `0xc0ce94ad`.
```solidity
function testSaveForManyNonManager() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyNonManagerCall;
    ///Container type for the return parameters of the [`testSaveForManyNonManager()`](testSaveForManyNonManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyNonManagerReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSaveForManyNonManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyNonManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyNonManagerCall {
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
            impl ::core::convert::From<testSaveForManyNonManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyNonManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyNonManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSaveForManyNonManagerReturn {
            fn _tokenize(
                &self,
            ) -> <testSaveForManyNonManagerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSaveForManyNonManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSaveForManyNonManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSaveForManyNonManager()";
            const SELECTOR: [u8; 4] = [192u8, 206u8, 148u8, 173u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSaveForManyNonManagerReturn::_tokenize(ret)
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
    /**Function with signature `testSaveForManyWithSingleEpoch()` and selector `0xac29af33`.
```solidity
function testSaveForManyWithSingleEpoch() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyWithSingleEpochCall;
    ///Container type for the return parameters of the [`testSaveForManyWithSingleEpoch()`](testSaveForManyWithSingleEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveForManyWithSingleEpochReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSaveForManyWithSingleEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyWithSingleEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyWithSingleEpochCall {
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
            impl ::core::convert::From<testSaveForManyWithSingleEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveForManyWithSingleEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveForManyWithSingleEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSaveForManyWithSingleEpochReturn {
            fn _tokenize(
                &self,
            ) -> <testSaveForManyWithSingleEpochCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSaveForManyWithSingleEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSaveForManyWithSingleEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSaveForManyWithSingleEpoch()";
            const SELECTOR: [u8; 4] = [172u8, 41u8, 175u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSaveForManyWithSingleEpochReturn::_tokenize(ret)
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
    /**Function with signature `testSaveNonManager()` and selector `0x2c41ee67`.
```solidity
function testSaveNonManager() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveNonManagerCall;
    ///Container type for the return parameters of the [`testSaveNonManager()`](testSaveNonManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSaveNonManagerReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSaveNonManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveNonManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveNonManagerCall {
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
            impl ::core::convert::From<testSaveNonManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSaveNonManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSaveNonManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSaveNonManagerReturn {
            fn _tokenize(
                &self,
            ) -> <testSaveNonManagerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSaveNonManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSaveNonManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSaveNonManager()";
            const SELECTOR: [u8; 4] = [44u8, 65u8, 238u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSaveNonManagerReturn::_tokenize(ret)
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
    /**Function with signature `testSetIndexFromBlock()` and selector `0x51f5287a`.
```solidity
function testSetIndexFromBlock() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSetIndexFromBlockCall;
    ///Container type for the return parameters of the [`testSetIndexFromBlock()`](testSetIndexFromBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSetIndexFromBlockReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSetIndexFromBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSetIndexFromBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSetIndexFromBlockCall {
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
            impl ::core::convert::From<testSetIndexFromBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSetIndexFromBlockReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSetIndexFromBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSetIndexFromBlockReturn {
            fn _tokenize(
                &self,
            ) -> <testSetIndexFromBlockCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSetIndexFromBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSetIndexFromBlockReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSetIndexFromBlock()";
            const SELECTOR: [u8; 4] = [81u8, 245u8, 40u8, 122u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSetIndexFromBlockReturn::_tokenize(ret)
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
    /**Function with signature `testSetIndexFromBlockNonManager()` and selector `0x90c96770`.
```solidity
function testSetIndexFromBlockNonManager() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSetIndexFromBlockNonManagerCall;
    ///Container type for the return parameters of the [`testSetIndexFromBlockNonManager()`](testSetIndexFromBlockNonManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSetIndexFromBlockNonManagerReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testSetIndexFromBlockNonManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSetIndexFromBlockNonManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSetIndexFromBlockNonManagerCall {
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
            impl ::core::convert::From<testSetIndexFromBlockNonManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSetIndexFromBlockNonManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSetIndexFromBlockNonManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testSetIndexFromBlockNonManagerReturn {
            fn _tokenize(
                &self,
            ) -> <testSetIndexFromBlockNonManagerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSetIndexFromBlockNonManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSetIndexFromBlockNonManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSetIndexFromBlockNonManager()";
            const SELECTOR: [u8; 4] = [144u8, 201u8, 103u8, 112u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testSetIndexFromBlockNonManagerReturn::_tokenize(ret)
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
    ///Container for all the [`SyndicateStorageTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SyndicateStorageTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        admin(adminCall),
        #[allow(missing_docs)]
        appchainId(appchainIdCall),
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
        manager(managerCall),
        #[allow(missing_docs)]
        nonAuthorized(nonAuthorizedCall),
        #[allow(missing_docs)]
        setUp(setUpCall),
        #[allow(missing_docs)]
        syndicateStorage(syndicateStorageCall),
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
        testConstructor(testConstructorCall),
        #[allow(missing_docs)]
        testConstructorZeroAdmin(testConstructorZeroAdminCall),
        #[allow(missing_docs)]
        testConstructorZeroAppChainId(testConstructorZeroAppChainIdCall),
        #[allow(missing_docs)]
        testConstructorZeroManager(testConstructorZeroManagerCall),
        #[allow(missing_docs)]
        testRoleManagement(testRoleManagementCall),
        #[allow(missing_docs)]
        testSave(testSaveCall),
        #[allow(missing_docs)]
        testSaveForMany(testSaveForManyCall),
        #[allow(missing_docs)]
        testSaveForManyArrayLengthMismatch1(testSaveForManyArrayLengthMismatch1Call),
        #[allow(missing_docs)]
        testSaveForManyArrayLengthMismatch2(testSaveForManyArrayLengthMismatch2Call),
        #[allow(missing_docs)]
        testSaveForManyEmptyArrays(testSaveForManyEmptyArraysCall),
        #[allow(missing_docs)]
        testSaveForManyNonManager(testSaveForManyNonManagerCall),
        #[allow(missing_docs)]
        testSaveForManyWithSingleEpoch(testSaveForManyWithSingleEpochCall),
        #[allow(missing_docs)]
        testSaveNonManager(testSaveNonManagerCall),
        #[allow(missing_docs)]
        testSetIndexFromBlock(testSetIndexFromBlockCall),
        #[allow(missing_docs)]
        testSetIndexFromBlockNonManager(testSetIndexFromBlockNonManagerCall),
    }
    impl SyndicateStorageTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [5u8, 165u8, 111u8, 128u8],
            [10u8, 146u8, 84u8, 228u8],
            [29u8, 220u8, 45u8, 16u8],
            [30u8, 215u8, 131u8, 28u8],
            [41u8, 127u8, 238u8, 178u8],
            [42u8, 222u8, 56u8, 128u8],
            [44u8, 65u8, 238u8, 103u8],
            [59u8, 123u8, 50u8, 91u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [72u8, 28u8, 106u8, 117u8],
            [81u8, 245u8, 40u8, 122u8],
            [83u8, 136u8, 44u8, 110u8],
            [89u8, 235u8, 240u8, 73u8],
            [102u8, 217u8, 169u8, 160u8],
            [133u8, 34u8, 108u8, 129u8],
            [141u8, 58u8, 131u8, 185u8],
            [141u8, 61u8, 108u8, 153u8],
            [144u8, 201u8, 103u8, 112u8],
            [145u8, 106u8, 23u8, 198u8],
            [150u8, 81u8, 64u8, 37u8],
            [150u8, 225u8, 129u8, 207u8],
            [172u8, 41u8, 175u8, 51u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [192u8, 206u8, 148u8, 173u8],
            [194u8, 233u8, 242u8, 228u8],
            [202u8, 80u8, 95u8, 226u8],
            [216u8, 120u8, 19u8, 66u8],
            [226u8, 12u8, 159u8, 113u8],
            [248u8, 81u8, 164u8, 64u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(testSaveForManyArrayLengthMismatch2),
            ::core::stringify!(setUp),
            ::core::stringify!(nonAuthorized),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(syndicateStorage),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(testSaveNonManager),
            ::core::stringify!(testSaveForManyArrayLengthMismatch1),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(manager),
            ::core::stringify!(testSetIndexFromBlock),
            ::core::stringify!(testSaveForMany),
            ::core::stringify!(testConstructorZeroManager),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(testConstructorZeroAdmin),
            ::core::stringify!(testRoleManagement),
            ::core::stringify!(testSetIndexFromBlockNonManager),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(testConstructorZeroAppChainId),
            ::core::stringify!(testSave),
            ::core::stringify!(testSaveForManyWithSingleEpoch),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(failed),
            ::core::stringify!(testSaveForManyNonManager),
            ::core::stringify!(testConstructor),
            ::core::stringify!(testSaveForManyEmptyArrays),
            ::core::stringify!(appchainId),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(admin),
            ::core::stringify!(IS_TEST),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <testSaveForManyArrayLengthMismatch2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nonAuthorizedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <syndicateStorageCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSaveNonManagerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSaveForManyArrayLengthMismatch1Call as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <managerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSetIndexFromBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSaveForManyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testConstructorZeroManagerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testConstructorZeroAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testRoleManagementCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSetIndexFromBlockNonManagerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testConstructorZeroAppChainIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSaveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSaveForManyWithSingleEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSaveForManyNonManagerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testConstructorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testSaveForManyEmptyArraysCall as alloy_sol_types::SolCall>::SIGNATURE,
            <appchainIdCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SyndicateStorageTestCalls {
        const NAME: &'static str = "SyndicateStorageTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 33usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::admin(_) => <adminCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::appchainId(_) => {
                    <appchainIdCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::manager(_) => <managerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonAuthorized(_) => {
                    <nonAuthorizedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::syndicateStorage(_) => {
                    <syndicateStorageCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::testConstructor(_) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testConstructorZeroAdmin(_) => {
                    <testConstructorZeroAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testConstructorZeroAppChainId(_) => {
                    <testConstructorZeroAppChainIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testConstructorZeroManager(_) => {
                    <testConstructorZeroManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testRoleManagement(_) => {
                    <testRoleManagementCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSave(_) => <testSaveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::testSaveForMany(_) => {
                    <testSaveForManyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSaveForManyArrayLengthMismatch1(_) => {
                    <testSaveForManyArrayLengthMismatch1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSaveForManyArrayLengthMismatch2(_) => {
                    <testSaveForManyArrayLengthMismatch2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSaveForManyEmptyArrays(_) => {
                    <testSaveForManyEmptyArraysCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSaveForManyNonManager(_) => {
                    <testSaveForManyNonManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSaveForManyWithSingleEpoch(_) => {
                    <testSaveForManyWithSingleEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSaveNonManager(_) => {
                    <testSaveNonManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSetIndexFromBlock(_) => {
                    <testSetIndexFromBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSetIndexFromBlockNonManager(_) => {
                    <testSetIndexFromBlockNonManagerCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SyndicateStorageTestCalls>] = &[
                {
                    fn testSaveForManyArrayLengthMismatch2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyArrayLengthMismatch2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testSaveForManyArrayLengthMismatch2,
                            )
                    }
                    testSaveForManyArrayLengthMismatch2
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateStorageTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn nonAuthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <nonAuthorizedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::nonAuthorized)
                    }
                    nonAuthorized
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn syndicateStorage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <syndicateStorageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::syndicateStorage)
                    }
                    syndicateStorage
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testSaveNonManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveNonManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSaveNonManager)
                    }
                    testSaveNonManager
                },
                {
                    fn testSaveForManyArrayLengthMismatch1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyArrayLengthMismatch1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testSaveForManyArrayLengthMismatch1,
                            )
                    }
                    testSaveForManyArrayLengthMismatch1
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn manager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <managerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateStorageTestCalls::manager)
                    }
                    manager
                },
                {
                    fn testSetIndexFromBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSetIndexFromBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSetIndexFromBlock)
                    }
                    testSetIndexFromBlock
                },
                {
                    fn testSaveForMany(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSaveForMany)
                    }
                    testSaveForMany
                },
                {
                    fn testConstructorZeroManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testConstructorZeroManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testConstructorZeroManager)
                    }
                    testConstructorZeroManager
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn testConstructorZeroAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testConstructorZeroAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testConstructorZeroAdmin)
                    }
                    testConstructorZeroAdmin
                },
                {
                    fn testRoleManagement(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testRoleManagementCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testRoleManagement)
                    }
                    testRoleManagement
                },
                {
                    fn testSetIndexFromBlockNonManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSetIndexFromBlockNonManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testSetIndexFromBlockNonManager,
                            )
                    }
                    testSetIndexFromBlockNonManager
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testConstructorZeroAppChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testConstructorZeroAppChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testConstructorZeroAppChainId,
                            )
                    }
                    testConstructorZeroAppChainId
                },
                {
                    fn testSave(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateStorageTestCalls::testSave)
                    }
                    testSave
                },
                {
                    fn testSaveForManyWithSingleEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyWithSingleEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testSaveForManyWithSingleEpoch,
                            )
                    }
                    testSaveForManyWithSingleEpoch
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateStorageTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testSaveForManyNonManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyNonManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSaveForManyNonManager)
                    }
                    testSaveForManyNonManager
                },
                {
                    fn testConstructor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testConstructorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testConstructor)
                    }
                    testConstructor
                },
                {
                    fn testSaveForManyEmptyArrays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyEmptyArraysCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSaveForManyEmptyArrays)
                    }
                    testSaveForManyEmptyArrays
                },
                {
                    fn appchainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <appchainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::appchainId)
                    }
                    appchainId
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateStorageTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateStorageTestCalls::IS_TEST)
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
            ) -> alloy_sol_types::Result<SyndicateStorageTestCalls>] = &[
                {
                    fn testSaveForManyArrayLengthMismatch2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyArrayLengthMismatch2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testSaveForManyArrayLengthMismatch2,
                            )
                    }
                    testSaveForManyArrayLengthMismatch2
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn nonAuthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <nonAuthorizedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::nonAuthorized)
                    }
                    nonAuthorized
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn syndicateStorage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <syndicateStorageCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::syndicateStorage)
                    }
                    syndicateStorage
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testSaveNonManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveNonManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSaveNonManager)
                    }
                    testSaveNonManager
                },
                {
                    fn testSaveForManyArrayLengthMismatch1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyArrayLengthMismatch1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testSaveForManyArrayLengthMismatch1,
                            )
                    }
                    testSaveForManyArrayLengthMismatch1
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn manager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <managerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::manager)
                    }
                    manager
                },
                {
                    fn testSetIndexFromBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSetIndexFromBlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSetIndexFromBlock)
                    }
                    testSetIndexFromBlock
                },
                {
                    fn testSaveForMany(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSaveForMany)
                    }
                    testSaveForMany
                },
                {
                    fn testConstructorZeroManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testConstructorZeroManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testConstructorZeroManager)
                    }
                    testConstructorZeroManager
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn testConstructorZeroAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testConstructorZeroAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testConstructorZeroAdmin)
                    }
                    testConstructorZeroAdmin
                },
                {
                    fn testRoleManagement(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testRoleManagementCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testRoleManagement)
                    }
                    testRoleManagement
                },
                {
                    fn testSetIndexFromBlockNonManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSetIndexFromBlockNonManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testSetIndexFromBlockNonManager,
                            )
                    }
                    testSetIndexFromBlockNonManager
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testConstructorZeroAppChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testConstructorZeroAppChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testConstructorZeroAppChainId,
                            )
                    }
                    testConstructorZeroAppChainId
                },
                {
                    fn testSave(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSave)
                    }
                    testSave
                },
                {
                    fn testSaveForManyWithSingleEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyWithSingleEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SyndicateStorageTestCalls::testSaveForManyWithSingleEpoch,
                            )
                    }
                    testSaveForManyWithSingleEpoch
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testSaveForManyNonManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyNonManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSaveForManyNonManager)
                    }
                    testSaveForManyNonManager
                },
                {
                    fn testConstructor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testConstructorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testConstructor)
                    }
                    testConstructor
                },
                {
                    fn testSaveForManyEmptyArrays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <testSaveForManyEmptyArraysCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::testSaveForManyEmptyArrays)
                    }
                    testSaveForManyEmptyArrays
                },
                {
                    fn appchainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <appchainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::appchainId)
                    }
                    appchainId
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateStorageTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateStorageTestCalls::IS_TEST)
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
                Self::appchainId(inner) => {
                    <appchainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::manager(inner) => {
                    <managerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonAuthorized(inner) => {
                    <nonAuthorizedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::syndicateStorage(inner) => {
                    <syndicateStorageCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::testConstructor(inner) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testConstructorZeroAdmin(inner) => {
                    <testConstructorZeroAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testConstructorZeroAppChainId(inner) => {
                    <testConstructorZeroAppChainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testConstructorZeroManager(inner) => {
                    <testConstructorZeroManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testRoleManagement(inner) => {
                    <testRoleManagementCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSave(inner) => {
                    <testSaveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::testSaveForMany(inner) => {
                    <testSaveForManyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSaveForManyArrayLengthMismatch1(inner) => {
                    <testSaveForManyArrayLengthMismatch1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSaveForManyArrayLengthMismatch2(inner) => {
                    <testSaveForManyArrayLengthMismatch2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSaveForManyEmptyArrays(inner) => {
                    <testSaveForManyEmptyArraysCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSaveForManyNonManager(inner) => {
                    <testSaveForManyNonManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSaveForManyWithSingleEpoch(inner) => {
                    <testSaveForManyWithSingleEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSaveNonManager(inner) => {
                    <testSaveNonManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSetIndexFromBlock(inner) => {
                    <testSetIndexFromBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSetIndexFromBlockNonManager(inner) => {
                    <testSetIndexFromBlockNonManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::appchainId(inner) => {
                    <appchainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::manager(inner) => {
                    <managerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonAuthorized(inner) => {
                    <nonAuthorizedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::syndicateStorage(inner) => {
                    <syndicateStorageCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testConstructor(inner) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testConstructorZeroAdmin(inner) => {
                    <testConstructorZeroAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testConstructorZeroAppChainId(inner) => {
                    <testConstructorZeroAppChainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testConstructorZeroManager(inner) => {
                    <testConstructorZeroManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testRoleManagement(inner) => {
                    <testRoleManagementCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSave(inner) => {
                    <testSaveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSaveForMany(inner) => {
                    <testSaveForManyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSaveForManyArrayLengthMismatch1(inner) => {
                    <testSaveForManyArrayLengthMismatch1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSaveForManyArrayLengthMismatch2(inner) => {
                    <testSaveForManyArrayLengthMismatch2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSaveForManyEmptyArrays(inner) => {
                    <testSaveForManyEmptyArraysCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSaveForManyNonManager(inner) => {
                    <testSaveForManyNonManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSaveForManyWithSingleEpoch(inner) => {
                    <testSaveForManyWithSingleEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSaveNonManager(inner) => {
                    <testSaveNonManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSetIndexFromBlock(inner) => {
                    <testSetIndexFromBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSetIndexFromBlockNonManager(inner) => {
                    <testSetIndexFromBlockNonManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SyndicateStorageTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SyndicateStorageTestEvents {
        #[allow(missing_docs)]
        EpochRangeProcessed(EpochRangeProcessed),
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
    impl SyndicateStorageTestEvents {
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
            [
                251u8, 63u8, 148u8, 44u8, 77u8, 135u8, 113u8, 172u8, 90u8, 54u8, 47u8,
                88u8, 189u8, 56u8, 4u8, 224u8, 117u8, 158u8, 189u8, 82u8, 129u8, 7u8,
                35u8, 136u8, 74u8, 124u8, 219u8, 83u8, 29u8, 131u8, 182u8, 243u8,
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
            ::core::stringify!(log_address),
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
            ::core::stringify!(EpochRangeProcessed),
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
            <log_address as alloy_sol_types::SolEvent>::SIGNATURE,
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
            <EpochRangeProcessed as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for SyndicateStorageTestEvents {
        const NAME: &'static str = "SyndicateStorageTestEvents";
        const COUNT: usize = 23usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <EpochRangeProcessed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EpochRangeProcessed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EpochRangeProcessed)
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
    impl alloy_sol_types::private::IntoLogData for SyndicateStorageTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EpochRangeProcessed(inner) => {
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
                Self::EpochRangeProcessed(inner) => {
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
    /**Creates a new wrapper around an on-chain [`SyndicateStorageTest`](self) contract instance.

See the [wrapper's documentation](`SyndicateStorageTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> SyndicateStorageTestInstance<P, N> {
        SyndicateStorageTestInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<SyndicateStorageTestInstance<P, N>>,
    > {
        SyndicateStorageTestInstance::<P, N>::deploy(__provider)
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
        SyndicateStorageTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`SyndicateStorageTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SyndicateStorageTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SyndicateStorageTestInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SyndicateStorageTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyndicateStorageTestInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateStorageTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SyndicateStorageTest`](self) contract instance.

See the [wrapper's documentation](`SyndicateStorageTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SyndicateStorageTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> SyndicateStorageTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SyndicateStorageTestInstance<P, N> {
            SyndicateStorageTestInstance {
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
    > SyndicateStorageTestInstance<P, N> {
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
        ///Creates a new call builder for the [`appchainId`] function.
        pub fn appchainId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, appchainIdCall, N> {
            self.call_builder(&appchainIdCall)
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
        ///Creates a new call builder for the [`manager`] function.
        pub fn manager(&self) -> alloy_contract::SolCallBuilder<&P, managerCall, N> {
            self.call_builder(&managerCall)
        }
        ///Creates a new call builder for the [`nonAuthorized`] function.
        pub fn nonAuthorized(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, nonAuthorizedCall, N> {
            self.call_builder(&nonAuthorizedCall)
        }
        ///Creates a new call builder for the [`setUp`] function.
        pub fn setUp(&self) -> alloy_contract::SolCallBuilder<&P, setUpCall, N> {
            self.call_builder(&setUpCall)
        }
        ///Creates a new call builder for the [`syndicateStorage`] function.
        pub fn syndicateStorage(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, syndicateStorageCall, N> {
            self.call_builder(&syndicateStorageCall)
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
        ///Creates a new call builder for the [`testConstructor`] function.
        pub fn testConstructor(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testConstructorCall, N> {
            self.call_builder(&testConstructorCall)
        }
        ///Creates a new call builder for the [`testConstructorZeroAdmin`] function.
        pub fn testConstructorZeroAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testConstructorZeroAdminCall, N> {
            self.call_builder(&testConstructorZeroAdminCall)
        }
        ///Creates a new call builder for the [`testConstructorZeroAppChainId`] function.
        pub fn testConstructorZeroAppChainId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testConstructorZeroAppChainIdCall, N> {
            self.call_builder(&testConstructorZeroAppChainIdCall)
        }
        ///Creates a new call builder for the [`testConstructorZeroManager`] function.
        pub fn testConstructorZeroManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testConstructorZeroManagerCall, N> {
            self.call_builder(&testConstructorZeroManagerCall)
        }
        ///Creates a new call builder for the [`testRoleManagement`] function.
        pub fn testRoleManagement(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testRoleManagementCall, N> {
            self.call_builder(&testRoleManagementCall)
        }
        ///Creates a new call builder for the [`testSave`] function.
        pub fn testSave(&self) -> alloy_contract::SolCallBuilder<&P, testSaveCall, N> {
            self.call_builder(&testSaveCall)
        }
        ///Creates a new call builder for the [`testSaveForMany`] function.
        pub fn testSaveForMany(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testSaveForManyCall, N> {
            self.call_builder(&testSaveForManyCall)
        }
        ///Creates a new call builder for the [`testSaveForManyArrayLengthMismatch1`] function.
        pub fn testSaveForManyArrayLengthMismatch1(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testSaveForManyArrayLengthMismatch1Call,
            N,
        > {
            self.call_builder(&testSaveForManyArrayLengthMismatch1Call)
        }
        ///Creates a new call builder for the [`testSaveForManyArrayLengthMismatch2`] function.
        pub fn testSaveForManyArrayLengthMismatch2(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testSaveForManyArrayLengthMismatch2Call,
            N,
        > {
            self.call_builder(&testSaveForManyArrayLengthMismatch2Call)
        }
        ///Creates a new call builder for the [`testSaveForManyEmptyArrays`] function.
        pub fn testSaveForManyEmptyArrays(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testSaveForManyEmptyArraysCall, N> {
            self.call_builder(&testSaveForManyEmptyArraysCall)
        }
        ///Creates a new call builder for the [`testSaveForManyNonManager`] function.
        pub fn testSaveForManyNonManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testSaveForManyNonManagerCall, N> {
            self.call_builder(&testSaveForManyNonManagerCall)
        }
        ///Creates a new call builder for the [`testSaveForManyWithSingleEpoch`] function.
        pub fn testSaveForManyWithSingleEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testSaveForManyWithSingleEpochCall, N> {
            self.call_builder(&testSaveForManyWithSingleEpochCall)
        }
        ///Creates a new call builder for the [`testSaveNonManager`] function.
        pub fn testSaveNonManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testSaveNonManagerCall, N> {
            self.call_builder(&testSaveNonManagerCall)
        }
        ///Creates a new call builder for the [`testSetIndexFromBlock`] function.
        pub fn testSetIndexFromBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testSetIndexFromBlockCall, N> {
            self.call_builder(&testSetIndexFromBlockCall)
        }
        ///Creates a new call builder for the [`testSetIndexFromBlockNonManager`] function.
        pub fn testSetIndexFromBlockNonManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testSetIndexFromBlockNonManagerCall, N> {
            self.call_builder(&testSetIndexFromBlockNonManagerCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateStorageTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`EpochRangeProcessed`] event.
        pub fn EpochRangeProcessed_filter(
            &self,
        ) -> alloy_contract::Event<&P, EpochRangeProcessed, N> {
            self.event_filter::<EpochRangeProcessed>()
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
