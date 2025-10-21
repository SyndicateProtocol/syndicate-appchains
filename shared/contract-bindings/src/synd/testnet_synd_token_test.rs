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

interface TestnetSyndTokenTest {
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Transfer(address indexed from, address indexed to, uint256 value);
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
    function defaultAdmin() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function minter() external view returns (address);
    function setUp() external;
    function spender() external view returns (address);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testFuzz_Mint_ValidAmounts(address to, uint256 amount) external;
    function testFuzz_Transfer_ValidAmounts(uint256 mintAmount, uint256 transferAmount) external;
    function test_Approve_Success() external;
    function test_Constructor_InitialSetup() external view;
    function test_Constructor_RoleAssignment() external view;
    function test_Delegate_Success() external;
    function test_GetVotingPower_WithTokens() external;
    function test_GetVotingPower_WithoutTokens() external view;
    function test_GrantMinterRole_Success() external;
    function test_Invariant_TotalSupplyMatchesBalances() external;
    function test_Mint_MultipleMints() external;
    function test_Mint_Success() external;
    function test_Permit_Success() external;
    function test_RevertWhen_Constructor_ZeroAdmin() external;
    function test_RevertWhen_Constructor_ZeroMinter() external;
    function test_RevertWhen_GrantRole_NotAdmin() external;
    function test_RevertWhen_Mint_NotMinter() external;
    function test_RevertWhen_Mint_ZeroAddress() external;
    function test_RevertWhen_Mint_ZeroAmount() external;
    function test_RevertWhen_Permit_ExpiredDeadline() external;
    function test_RevokeMinterRole_Success() external;
    function test_TransferFrom_Success() external;
    function test_Transfer_Success() external;
    function token() external view returns (address);
    function user() external view returns (address);
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
    "name": "defaultAdmin",
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
    "name": "minter",
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
    "name": "spender",
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
    "name": "testFuzz_Mint_ValidAmounts",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "testFuzz_Transfer_ValidAmounts",
    "inputs": [
      {
        "name": "mintAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "transferAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Approve_Success",
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
    "name": "test_Delegate_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_GetVotingPower_WithTokens",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_GetVotingPower_WithoutTokens",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_GrantMinterRole_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Invariant_TotalSupplyMatchesBalances",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Mint_MultipleMints",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Mint_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Permit_Success",
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
    "name": "test_RevertWhen_Constructor_ZeroMinter",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_GrantRole_NotAdmin",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_Mint_NotMinter",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_Mint_ZeroAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_Mint_ZeroAmount",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_Permit_ExpiredDeadline",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevokeMinterRole_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_TransferFrom_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Transfer_Success",
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
        "internalType": "contract TestnetSyndToken"
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
    "name": "Approval",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "Transfer",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
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
pub mod TestnetSyndTokenTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234602f57600160ff19600c541617600c55600160ff19601f541617601f55619c8a90816100348239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c9081630481205414615c9c575080630642dde514615c255780630754617214615bfe5780630a9254e4146154795780631074a21f1461533d578063123a4a5f146151b85780631ed7831c1461513a5780632246e5cc14614c135780632ade388014614a1f5780633e5e3c23146149a15780633f7286f41461492357806341686ff21461423657806347483c5d14613d515780634f8632ba14613d2a57806355f7d47714613a805780635bb177811461354b5780636338aa86146132c1578063640f725a1461302857806366d9a9a014612eeb57806371d7dabf14612b48578063746a9bcf146129c757806376029e78146126cf57806384ef8ffc146126a957806385226c811461261f57806388c5671b146122495780638d31ed5314611e735780638f08ece714611b7b5780638f310dfe14611782578063916a17c6146116d857806391dc0b2d14611352578063b0464fdc146112a8578063b5508aa91461121e578063b78b596714610d1b578063ba414fa614610cf6578063bbb155331461079f578063c01e9428146104f8578063dccc57f1146102e5578063e20c9f7114610257578063e8edc81614610230578063fa7626d41461020d5763fc0c546a146101e1575f80fd5b3461020a578060031936011261020a5760206001600160a01b03601f5460081c16604051908152f35b80fd5b503461020a578060031936011261020a57602060ff601f54166040519015158152f35b503461020a578060031936011261020a5760206001600160a01b0360235416604051908152f35b503461020a578060031936011261020a5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106102c6576102c2856102b681870382615fbd565b60405191829182615da4565b0390f35b82546001600160a01b031684526020909301926001928301920161029f565b503461020a578060031936011261020a576001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba5783916104c5575b50602080546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101939093526001600160a01b0316602483015281604481855afa80156104ba576103a1918491610448575b50616a0a565b6040517fd5391393000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba578391610482575b506021546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529060209082908180604481015b03915afa8015610477576104459183916104485750616a0a565b80f35b61046a915060203d602011610470575b6104628183615fbd565b810190616170565b5f61039b565b503d610458565b6040513d84823e3d90fd5b90506020813d6020116104b2575b8161049d60209383615fbd565b810103126104ae575161042b6103de565b5f80fd5b3d9150610490565b6040513d85823e3d90fd5b90506020813d6020116104f0575b816104e060209383615fbd565b810103126104ae57516020610342565b3d91506104d3565b503461020a57604060031936011261020a576004356001600160a01b0381169081810361076d57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561076d57826040517f4c63e56200000000000000000000000000000000000000000000000000000000815283151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156104775761078a575b5061059c6024356168c6565b916001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561076d576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104ba578391610771575b50506001600160a01b03601f5460081c16803b1561076d576040516340c10f1960e01b81526001600160a01b039290921660048301526024820184905282908290604490829084905af1801561047757610758575b50506001600160a01b03601f5460081c1691604051906370a0823160e01b82526004820152602081602481865afa90811561074d57849161071a575b506004926106a383602093616850565b604051938480927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa80156104ba5783906106e6575b6104459250616850565b506020823d602011610712575b8161070060209383615fbd565b810103126104ae5761044591516106dc565b3d91506106f3565b90506020813d602011610745575b8161073560209383615fbd565b810103126104ae57516004610693565b3d9150610728565b6040513d86823e3d90fd5b8161076291615fbd565b61076d57825f610657565b8280fd5b8161077b91615fbd565b61078657815f610602565b5080fd5b8161079491615fbd565b61076d57825f610590565b503461020a578060031936011261020a576040517fffa18649000000000000000000000000000000000000000000000000000000008152620a11ce6004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610477578291610cc7575b50610e10420190814211610b57576001600160a01b03601f5460081c169183604051937f7ecebe000000000000000000000000000000000000000000000000000000000085526001600160a01b03841692836004870152602086602481855afa9586156104ba578396610c90575b506001600160a01b0360235416906040516020810190610904816108f6858c888d88909493926001600160a01b0360a0938160c08501987f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98652166020850152166040830152683635c9adc5dea00000606083015260808201520152565b03601f198101835282615fbd565b519020604051907f3644e515000000000000000000000000000000000000000000000000000000008252602082600481885afa918215610c85578692610c4c575b506040517f1901000000000000000000000000000000000000000000000000000000000000602082019081526022820193909352604281019190915261098e81606281016108f6565b51902092604051937fe341eaa4000000000000000000000000000000000000000000000000000000008552620a11ce60048601526024850152606084604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa928315610bec57859486908795610c14575b50823b15610c105788610a7d88968793604051998a98899788967fd505accf00000000000000000000000000000000000000000000000000000000885260048801929360c0946001600160a01b0360ff93999897948160e088019b168752166020860152683635c9adc5dea000006040860152606085015216608083015260a08201520152565b03925af1801561047757610bf7575b5050601f546023546040517fdd62ed3e0000000000000000000000000000000000000000000000000000000081526001600160a01b039485166004820152908416602482015260089190911c9092169190602082604481865afa918215610bec578592610bb6575b50610b00602092616654565b6024604051809481937f7ecebe0000000000000000000000000000000000000000000000000000000000835260048301525afa9081156104ba578391610b84575b5060018201809211610b57579061044591616850565b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011610bae575b81610b9f60209383615fbd565b810103126104ae57515f610b41565b3d9150610b92565b91506020823d602011610be4575b81610bd160209383615fbd565b810103126104ae57905190610b00610af4565b3d9150610bc4565b6040513d87823e3d90fd5b81610c0191615fbd565b610c0c57835f610a8c565b8380fd5b8680fd5b91955050610c3b91935060603d606011610c45575b610c338183615fbd565b8101906164d9565b939194905f6109f6565b503d610c29565b955090506020853d602011610c7d575b81610c6960209383615fbd565b810103126104ae579351889461098e610945565b3d9150610c5c565b6040513d88823e3d90fd5b925094506020823d602011610cbf575b81610cad60209383615fbd565b810103126104ae57859151945f610878565b3d9150610ca0565b610ce9915060203d602011610cef575b610ce18183615fbd565b8101906164ba565b5f61080a565b503d610cd7565b503461020a578060031936011261020a576020610d116164fb565b6040519015158152f35b503461020a578060031936011261020a57604051907fffa18649000000000000000000000000000000000000000000000000000000008252620a11ce6004830152602082602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9182156112115781926111f0575b507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4201914283116111c35781926001600160a01b03601f5460081c166040517f7ecebe000000000000000000000000000000000000000000000000000000000081526001600160a01b0384166004820152602081602481855afa908115610bec57859161118b575b5090602060049284610e936001600160a01b0360235416926108f6604051938492878401968c88909493926001600160a01b0360a0938160c08501987f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98652166020850152166040830152683635c9adc5dea00000606083015260808201520152565b51902091604051938480927f3644e5150000000000000000000000000000000000000000000000000000000082525afa918215610bec578592611152575b506040517f19010000000000000000000000000000000000000000000000000000000000006020820190815260228201939093526042810191909152610f1a81606281016108f6565b51902090604051917fe341eaa4000000000000000000000000000000000000000000000000000000008352620a11ce60048401526024830152606082604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561074d57849385938692611128575b506040517f6279130200000000000000000000000000000000000000000000000000000000602082015283602482015260248152610fc0604482615fbd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c10578661101b91604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561111d578791611104575b50506001600160a01b03601f5460081c16916001600160a01b036023541691833b15611100576040517fd505accf0000000000000000000000000000000000000000000000000000000081526001600160a01b03918216600482015292166024830152683635c9adc5dea000006044830152606482019390935260ff94909416608485015260a484019290925260c48301528290829060e490829084905af18015610477576110ef5750f35b816110f991615fbd565b61020a5780f35b8780fd5b8161110e91615fbd565b61111957855f611043565b8580fd5b6040513d89823e3d90fd5b9150935061114691925060603d606011610c4557610c338183615fbd565b9291939092905f610f81565b945090506020843d602011611183575b8161116f60209383615fbd565b810103126104ae5792518493610f1a610ed1565b3d9150611162565b919450506020813d6020116111bb575b816111a860209383615fbd565b810103126104ae57518493906020610e10565b3d915061119b565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b61120a91925060203d602011610cef57610ce18183615fbd565b905f610d87565b50604051903d90823e3d90fd5b503461020a578060031936011261020a5760195461123b81615fe0565b916112496040519384615fbd565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b83831061128b57604051806102c28782615e60565b60016020819261129a85615ff8565b815201920192019190611276565b503461020a578060031936011261020a57601c546112c581615fe0565b916112d36040519384615fbd565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b83831061131557604051806102c28782615edd565b6002602060019260405161132881615f74565b6001600160a01b038654168152611340858701616195565b83820152815201920192019190611300565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576116c3575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af18015610477576116ae575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611696575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156116265782916024839260405194859384927f5c19a95c00000000000000000000000000000000000000000000000000000000845260048401525af1801561047757611681575b506001600160a01b03601f5460081c166001600160a01b0360235416906040517f9ab24eb0000000000000000000000000000000000000000000000000000000008152826004820152602081602481855afa90811561074d578491611649575b5061155590616654565b60206001600160a01b03602254166024604051809481937f587cde1e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156104ba57839161162a575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611626576040517f515361f60000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152911660248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610477576110ef5750f35b5050fd5b611643915060203d602011610cef57610ce18183615fbd565b5f6115a4565b9350506020833d602011611679575b8161166560209383615fbd565b810103126104ae576115558493519061154b565b3d9150611658565b8161168b91615fbd565b61020a57805f6114eb565b816116a091615fbd565b61020a57805f611485565b50fd5b816116b891615fbd565b61020a57805f611422565b816116cd91615fbd565b61020a57805f6113c6565b503461020a578060031936011261020a57601d546116f581615fe0565b916117036040519384615fbd565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b83831061174557604051806102c28782615edd565b6002602060019260405161175881615f74565b6001600160a01b038654168152611770858701616195565b83820152815201920192019190611730565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611b66575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152681b1ae4d6e2ef50000060248401525af1801561047757611b51575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152681043561a882930000060248401525af1801561047757611b3c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611b27575b506001600160a01b03601f5460081c166001600160a01b0360225416604051906370a0823160e01b82526004820152602081602481855afa9081156104ba578391611aef575b50611983906167d1565b6001600160a01b0360235416604051906370a0823160e01b82526004820152602081602481855afa9081156104ba578391611ab7575b506004916119c8602092616752565b604051928380927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa908115610477578291611a82575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152682b5e3af16b1880000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610477576110ef5750f35b9150506020813d602011611aaf575b81611a9e60209383615fbd565b810103126104ae578190515f611a02565b3d9150611a91565b9250506020823d602011611ae7575b81611ad360209383615fbd565b810103126104ae57905182919060046119b9565b3d9150611ac6565b9250506020823d602011611b1f575b81611b0b60209383615fbd565b810103126104ae5761198383925190611979565b3d9150611afe565b81611b3191615fbd565b61020a57805f611933565b81611b4691615fbd565b61020a57805f6118c7565b81611b5b91615fbd565b61020a57805f61186b565b81611b7091615fbd565b61020a57805f61180f565b503461020a578060031936011261020a57806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611e5e575b5060046001600160a01b036022541660206001600160a01b03601f5460081c16604051938480927fd53913930000000000000000000000000000000000000000000000000000000082525afa9182156104ba578392611e25575b506040517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b0390911660248201526044810191909152611cb181606481016108f6565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab5781611d0c91604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611e10575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af1801561047757611dfb575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576110ef5750f35b81611e0591615fbd565b61020a57805f611d8d565b81611e1a91615fbd565b61020a57805f611d31565b925090506020823d602011611e56575b81611e4260209383615fbd565b810103126104ae5790518291611cb1611c62565b3d9150611e35565b81611e6891615fbd565b61020a57805f611c08565b503461020a578060031936011261020a57806001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba578391612211575b50602080546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101939093526001600160a01b03166024830181905292829060449082905afa80156104ba57611f349184916104485750616a0a565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576121fc575b506001600160a01b03601f5460081c166040517fd5391393000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba5783916121c7575b506001600160a01b0360225416823b156121c2576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af18015610477576121ad575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757612198575b50506001600160a01b03601f5460081c166040517fd5391393000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba578391612164575b506022546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b0316602482015290602090829081806044810161042b565b90506020813d602011612190575b8161217f60209383615fbd565b810103126104ae575161042b612113565b3d9150612172565b816121a291615fbd565b61020a57805f6120c5565b816121b791615fbd565b61020a57805f612059565b505050fd5b9250506020823d6020116121f4575b816121e360209383615fbd565b810103126104ae578291515f611ff0565b3d91506121d6565b8161220691615fbd565b61020a57805f611fa3565b9250506020823d602011612241575b8161222d60209383615fbd565b810103126104ae5790518291906020611ed1565b3d9150612220565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104775761260a575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af18015610477576125f5575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281818061238460048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576125e0575b506001600160a01b03602254166001600160a01b0360235416817fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6020604051681043561a88293000008152a3737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576125cb575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fa9059cbb0000000000000000000000000000000000000000000000000000000084526004840152681043561a882930000060248401525af18015610477576125ae575b506001600160a01b03601f5460081c166001600160a01b0360225416604051906370a0823160e01b82526004820152602081602481855afa80156104ba57839061257a575b61250791506166d3565b60206001600160a01b03602354166024604051809481936370a0823160e01b835260048301525afa8015610477578290612546575b6104459150616752565b506020813d602011612572575b8161256060209383615fbd565b810103126104ae57610445905161253c565b3d9150612553565b506020813d6020116125a6575b8161259460209383615fbd565b810103126104ae5761250790516124fd565b3d9150612587565b6125c69060203d602011610470576104628183615fbd565b6124b8565b816125d591615fbd565b61020a57805f61244c565b816125ea91615fbd565b61020a57805f6123a9565b816125ff91615fbd565b61020a57805f612319565b8161261491615fbd565b61020a57805f6122bd565b503461020a578060031936011261020a57601a5461263c81615fe0565b9161264a6040519384615fbd565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b83831061268c57604051806102c28782615e60565b60016020819261269b85615ff8565b815201920192019190612677565b503461020a578060031936011261020a5760206001600160a01b03815416604051908152f35b503461020a578060031936011261020a576001600160a01b03601f5460081c166040517f06fdde030000000000000000000000000000000000000000000000000000000081528281600481855afa9081156104ba5783916129ad575b5061276e60409182519061273f8483615fbd565b601182527f546573746e65742053796e6469636174650000000000000000000000000000006020830152616973565b80517f95d89b410000000000000000000000000000000000000000000000000000000081528381600481865afa9081156129a357906127eb918591612981575b508251906127bc8483615fbd565b600b82527f546573746e657453594e440000000000000000000000000000000000000000006020830152616973565b8281517f313ce567000000000000000000000000000000000000000000000000000000008152602081600481875afa90811561293d578291612947575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107865760ff8351917f98296c54000000000000000000000000000000000000000000000000000000008352166004820152601260248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561293d57612928575b505060206004928251938480927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa90811561291f575082906128eb575b61044591506165d4565b506020813d602011612917575b8161290560209383615fbd565b810103126104ae5761044590516128e1565b3d91506128f8565b513d84823e3d90fd5b8161293291615fbd565b61076d57825f6128a0565b83513d84823e3d90fd5b90506020813d602011612979575b8161296260209383615fbd565b8101031261078657612973906164ac565b5f612828565b3d9150612955565b61299d91503d8087833e6129958183615fbd565b810190616439565b5f6127ae565b82513d86823e3d90fd5b6129c191503d8085833e6129958183615fbd565b5f61272b565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757612b33575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757612b1e575b506001600160a01b03601f5460081c16803b156116ab578180916044604051809481936340c10f1960e01b8352816004840152683635c9adc5dea0000060248401525af18015610477576110ef5750f35b81612b2891615fbd565b61020a57805f612acd565b81612b3d91615fbd565b61020a57805f612a3b565b503461020a57604060031936011261020a57612b656004356168c6565b612b73816001602435616ab3565b604083808251612b838482615fbd565b600c81527f426f756e6420726573756c74000000000000000000000000000000000000000060208201528351612c0281612bee60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528860248401526064830190615de6565b88604483015203601f198101835282615fbd565b51906a636f6e736f6c652e6c6f675afa50836001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107865782519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561293d57612ed6575b506001600160a01b03601f5460081c166001600160a01b036022541690803b1561076d5783516340c10f1960e01b81526001600160a01b039290921660048301526024820186905282908290604490829084905af1801561293d57612ec1575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107865782519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561293d57612eac575b5050601f5460235482517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201526024810185905291602091839160081c168188816044810103925af18015612ea257612e85575b506001600160a01b03601f5460081c16926001600160a01b0360225416908251916370a0823160e01b83526004830152602082602481885afa918215612e7b57908492918792612e42575b50612dfa92612df491616188565b90616850565b60206001600160a01b036023541660248351809681936370a0823160e01b835260048301525afa908115612e39575083906106e6576104459250616850565b513d85823e3d90fd5b925090506020823d602011612e73575b81612e5f60209383615fbd565b810103126104ae5790518391612dfa612de6565b3d9150612e52565b83513d88823e3d90fd5b612e9d9060203d602011610470576104628183615fbd565b612d9b565b82513d87823e3d90fd5b81612eb691615fbd565b610c0c57835f612d37565b81612ecb91615fbd565b610c0c57835f612cd5565b81612ee091615fbd565b610c0c57835f612c75565b503461020a578060031936011261020a57601b54612f0881615fe0565b612f156040519182615fbd565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310612fed57868587604051928392602084019060208552518091526040840160408260051b8601019392905b828210612f8257505050500390f35b91936020612fdd827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083612fcd8351604084526040840190615de6565b9201519084818403910152615e0b565b9601920192018594939192612f73565b6002602060019260405161300081615f74565b61300986615ff8565b8152613016858701616195565b83820152815201920192019190612f45565b503461020a578060031936011261020a57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f491cc7c20000000000000000000000000000000000000000000000000000000081528181806130a260048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576132ac575b506001600160a01b03602254166001600160a01b0360235416817f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9256020604051681b1ae4d6e2ef5000008152a3737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613297575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152681b1ae4d6e2ef50000060248401525af180156104775761327a575b50601f546022546023546040517fdd62ed3e0000000000000000000000000000000000000000000000000000000081526001600160a01b0392831660048201529082166024820152916020918391604491839160081c165afa8015610477578290613246575b61044591506167d1565b506020813d602011613272575b8161326060209383615fbd565b810103126104ae57610445905161323c565b3d9150613253565b6132929060203d602011610470576104628183615fbd565b6131d6565b816132a191615fbd565b61020a57805f61316a565b816132b691615fbd565b61020a57805f6130c7565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613536575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af1801561047757613521575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104775761350c575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916024839260405194859384927f5c19a95c00000000000000000000000000000000000000000000000000000000845260048401525af18015610477576134f7575b50506001600160a01b03601f5460081c1660206001600160a01b03602254166024604051809481937fbb4d443600000000000000000000000000000000000000000000000000000000835260048301525afa80156104775782906134c3575b6104459150616654565b506020813d6020116134ef575b816134dd60209383615fbd565b810103126104ae5761044590516134b9565b3d91506134d0565b8161350191615fbd565b61020a57805f61345a565b8161351691615fbd565b61020a57805f6133f4565b8161352b91615fbd565b61020a57805f613391565b8161354091615fbd565b61020a57805f613335565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613a6b575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af1801561047757613a56575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613a41575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152681b1ae4d6e2ef50000060248401525af1801561047757613a24575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613a0f575b50506001600160a01b03601f5460081c1660206001600160a01b036022541660646001600160a01b0360235416918560405195869485937f23b872dd00000000000000000000000000000000000000000000000000000000855260048501526024840152681043561a882930000060448401525af18015610477576139f2575b50806001600160a01b03601f5460081c166001600160a01b0360225416906040516370a0823160e01b8152826004820152602081602481855afa90811561074d5784916139ba575b50613820906166d3565b6001600160a01b0360235416916040516370a0823160e01b8152836004820152602081602481865afa908115610bec57859161397d575b50916138b69391613869602094616752565b6040518095819482937fdd62ed3e000000000000000000000000000000000000000000000000000000008452600484019092916001600160a01b0360209181604085019616845216910152565b03915afa908115610477578291613948575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152680ad78ebc5ac620000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610477576110ef5750f35b9150506020813d602011613975575b8161396460209383615fbd565b810103126104ae578190515f6138c8565b3d9150613957565b92945050916020823d6020116139b2575b8161399b60209383615fbd565b810103126104ae57905184939192906138b6613857565b3d915061398e565b9350506020833d6020116139ea575b816139d660209383615fbd565b810103126104ae5761382084935190613816565b3d91506139c9565b613a0a9060203d602011610470576104628183615fbd565b6137ce565b81613a1991615fbd565b61020a57805f61374e565b613a3c9060203d602011610470576104628183615fbd565b6136ea565b81613a4b91615fbd565b61020a57805f61367e565b81613a6091615fbd565b61020a57805f61361b565b81613a7591615fbd565b61020a57805f6135bf565b503461020a578060031936011261020a57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152818180613afa60048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613d15575b50506001600160a01b0360225416817fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6020604051683635c9adc5dea000008152a3806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613d00575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af1801561047757613ceb575b50506001600160a01b03601f5460081c166001600160a01b0360225416604051906370a0823160e01b82526004820152602081602481855afa9081156104ba578391613cb8575b50600491613c76602092616654565b604051928380927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa80156104775782906134c3576104459150616654565b90506020813d602011613ce3575b81613cd360209383615fbd565b810103126104ae57516004613c67565b3d9150613cc6565b81613cf591615fbd565b61020a57805f613c20565b81613d0a91615fbd565b61020a57805f613bc4565b81613d1f91615fbd565b61020a57805f613b1f565b503461020a578060031936011261020a5760206001600160a01b0360225416604051908152f35b503461020a578060031936011261020a57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f491cc7c200000000000000000000000000000000000000000000000000000000815260016004820152600160248201526001604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757614221575b50600460206001600160a01b03601f5460081c16604051928380927fd53913930000000000000000000000000000000000000000000000000000000082525afa9081156104775782916141ec575b506001600160a01b03602154166001600160a01b03602054168091604051937ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b8680a4737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611626577f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576141d7575b506001600160a01b03601f5460081c166040517fd5391393000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba5783916141a2575b506001600160a01b0360215416823b156121c2576040517fd547741f00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af180156104775761418d575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757614178575b506001600160a01b03601f5460081c16604051907fd5391393000000000000000000000000000000000000000000000000000000008252602082600481845afa9182156104ba578392614140575b506021546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101939093526001600160a01b03166024830152602090829060449082905afa908115610477578291614121575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610477576110ef5750f35b61413a915060203d602011610470576104628183615fbd565b5f6140ae565b925090506020823d602011614170575b8161415d60209383615fbd565b810103126104ae57905182916020614054565b3d9150614150565b8161418291615fbd565b61020a57805f614006565b8161419791615fbd565b61020a57805f613f9a565b9250506020823d6020116141cf575b816141be60209383615fbd565b810103126104ae578291515f613f31565b3d91506141b1565b816141e191615fbd565b61020a57805f613ee4565b9150506020813d602011614219575b8161420860209383615fbd565b810103126104ae578190515f613e36565b3d91506141fb565b8161422b91615fbd565b61020a57805f613de8565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104775761490e575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af18015610477576148f9575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152686c6b935b8bbd40000060248401525af18015610477576148e4575b506001600160a01b03601f5460081c166143936160fb565b60405160208101906143bf602082855180838801875e810188838201520301601f198101835282615fbd565b519020604051907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561074d5784916148c5575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156121c257836001600160a01b036144869260405193849283927fc657c7180000000000000000000000000000000000000000000000000000000084521695866004840152604060248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561074d5784916148b0575b5050813b156116265782916044839260405194859384926340c10f1960e01b8452600484015268a2a15d09519be0000060248401525af180156104775761489b575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757614886575b50506001600160a01b03601f5460081c166040517f18160ddd000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba578391614854575b506001600160a01b0360225416604051906370a0823160e01b82526004820152602081602481865afa90811561074d578491614822575b506001600160a01b036023541690604051916370a0823160e01b83526004830152602082602481875afa908115610bec5785916147ec575b6146239250616136565b9161462c6160fb565b846040516020810190614659602082865180838901875e810186838201520301601f198101835282615fbd565b519020604051907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156104775782916147cd575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b036147209260405193849283927fc657c7180000000000000000000000000000000000000000000000000000000084521696876004840152604060248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576147b4575b50506020906024604051809481936370a0823160e01b835260048301525afa90811561074d578491614780575b50612df49061044593616136565b90506020813d6020116147ac575b8161479b60209383615fbd565b810103126104ae5751610445614772565b3d915061478e565b816147be91615fbd565b6147c957845f614745565b8480fd5b6147e6915060203d602011610cef57610ce18183615fbd565b5f6146b3565b90506020823d60201161481a575b8161480760209383615fbd565b810103126104ae57614623915190614619565b3d91506147fa565b90506020813d60201161484c575b8161483d60209383615fbd565b810103126104ae57515f6145e1565b3d9150614830565b90506020813d60201161487e575b8161486f60209383615fbd565b810103126104ae57515f6145aa565b3d9150614862565b8161489091615fbd565b61020a57805f61455c565b816148a591615fbd565b61020a57805f6144f0565b816148ba91615fbd565b61162657825f6144ae565b6148de915060203d602011610cef57610ce18183615fbd565b5f614419565b816148ee91615fbd565b61020a57805f61437b565b8161490391615fbd565b61020a57805f61431f565b8161491891615fbd565b61020a57805f6142c3565b503461020a578060031936011261020a5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110614982576102c2856102b681870382615fbd565b82546001600160a01b031684526020909301926001928301920161496b565b503461020a578060031936011261020a5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110614a00576102c2856102b681870382615fbd565b82546001600160a01b03168452602090930192600192830192016149e9565b503461020a578060031936011261020a57601e54614a3c81615fe0565b614a496040519182615fbd565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b838310614b8a5786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310614ab55786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110614b4157505050505060208060019297019301930190928695949293614aa8565b9091929394602080614b7d837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951615de6565b9701950193929101614b1d565b604051614b9681615f74565b6001600160a01b038354168152600183018054614bb281615fe0565b91614bc06040519384615fbd565b8183528a526020808b20908b9084015b838210614bf6575050505060019282602092836002950152815201920192019190614a79565b600160208192614c0586615ff8565b815201930191019091614bd0565b503461020a578060031936011261020a57604081815191614c348184615fbd565b600c8352602083017f77726f6e6741646472657373000000000000000000000000000000000000000081528151600c6020820192835e83602c820152600c8152614c7f602c82615fbd565b5190208151907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561503c57839161511b575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561076d5781517fc657c718000000000000000000000000000000000000000000000000000000008152838180614d406001600160a01b038616988960048401528760248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561505d57908491615106575b50506001600160a01b03601f5460081c16908251917fa217fddf000000000000000000000000000000000000000000000000000000008352602083600481845afa9283156150fc5785936150c0575b5094602084959660049551958680927fd53913930000000000000000000000000000000000000000000000000000000082525afa93841561507f578694615089575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611119578451907f06447d560000000000000000000000000000000000000000000000000000000082526004820152858160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561507f5790869161506a575b505083517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b0390911660248201526044810191909152614eba81606481016108f6565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156121c25783614f14918451809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561505d57908491615048575b50506001600160a01b03601f5460081c16906001600160a01b0360235416823b156147c95783517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529083908290604490829084905af1801561503c57908391615027575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561291f57506110ef5750f35b8161503191615fbd565b6116ab57815f614fb8565b505051903d90823e3d90fd5b8161505291615fbd565b61162657825f614f3c565b50505051903d90823e3d90fd5b8161507491615fbd565b6147c957845f614e6b565b85513d88823e3d90fd5b955092506020853d6020116150b8575b816150a660209383615fbd565b810103126104ae57859451925f614df9565b3d9150615099565b93945091506020833d6020116150f4575b816150de60209383615fbd565b810103126104ae57915185939290916020614db7565b3d91506150d1565b84513d87823e3d90fd5b8161511091615fbd565b61076d57825f614d68565b615134915060203d602011610cef57610ce18183615fbd565b5f614cd8565b503461020a578060031936011261020a5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110615199576102c2856102b681870382615fbd565b82546001600160a01b0316845260209093019260019283019201615182565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757615328575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f1f2a2005000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757615313575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b845260048401528160248401525af18015610477576110ef5750f35b8161531d91615fbd565b61020a57805f6152be565b8161533291615fbd565b61020a57805f61522c565b503461020a578060031936011261020a57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757615464575b50506001600160a01b036021541660405190613013908183019183831067ffffffffffffffff84111761543757918391604093616c778439858252602082015203019082f01561542b5780f35b604051903d90823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b8161546e91615fbd565b61020a57805f6153de565b503461020a578060031936011261020a57604080516154988282615fbd565b600c815282602082017f64656661756c7441646d696e000000000000000000000000000000000000000081528351600c6020820192835e82602c820152600c81526154e4602c82615fbd565b5190208351907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615b39578291615bdf575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b036155a892865193849283927fc657c71800000000000000000000000000000000000000000000000000000000845216968760048401528860248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615b3957615bca575b50507fffffffffffffffffffffffff0000000000000000000000000000000000000000602054161760205580516156048282615fbd565b6006815282602082017f6d696e74657200000000000000000000000000000000000000000000000000008152835160066020820192835e82602682015260068152615650602682615fbd565b5190208351907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615b39578291615bab575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b0361571492865193849283927fc657c71800000000000000000000000000000000000000000000000000000000845216968760048401528860248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615b3957615b96575b50507fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215580516157708282615fbd565b6004815282602082017f75736572000000000000000000000000000000000000000000000000000000008152835160046020820192835e826024820152600481526157bc602482615fbd565b5190208351907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615b39578291615b77575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b0361588092865193849283927fc657c71800000000000000000000000000000000000000000000000000000000845216968760048401528860248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615b3957615b62575b50507fffffffffffffffffffffffff0000000000000000000000000000000000000000602254161760225580516158dc8282615fbd565b6007815282602082017f7370656e646572000000000000000000000000000000000000000000000000008152835160076020820192835e82602782015260078152615928602782615fbd565b5190208351907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615b39578291615b43575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b036159ec92865193849283927fc657c71800000000000000000000000000000000000000000000000000000000845216968760048401528860248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615b3957615b24575b50507fffffffffffffffffffffffff000000000000000000000000000000000000000060235416176023556001600160a01b03602054166001600160a01b03602154168251916130138084019084821067ffffffffffffffff831117615af75791849391615a9893616c7786396001600160a01b0391821681529116602082015260400190565b039083f0908115615aec57507fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f5580f35b51913d9150823e3d90fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b81615b2e91615fbd565b61076d57825f615a11565b84513d84823e3d90fd5b615b5c915060203d602011610cef57610ce18183615fbd565b5f615981565b81615b6c91615fbd565b61076d57825f6158a5565b615b90915060203d602011610cef57610ce18183615fbd565b5f615815565b81615ba091615fbd565b61076d57825f615739565b615bc4915060203d602011610cef57610ce18183615fbd565b5f6156a9565b81615bd491615fbd565b61076d57825f6155cd565b615bf8915060203d602011610cef57610ce18183615fbd565b5f61553d565b503461020a578060031936011261020a5760206001600160a01b0360215416604051908152f35b503461020a578060031936011261020a576001600160a01b03601f5460081c1660206001600160a01b03602254166024604051809481937fbb4d443600000000000000000000000000000000000000000000000000000000835260048301525afa80156104775782906128eb5761044591506165d4565b9050346104ae575f6003193601126104ae57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae577fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d0000000000000000000000000000000000000000000000000000000060048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615d9957615d86575b506001600160a01b036020541660405190613013908183019183831067ffffffffffffffff84111761543757918391604093616c778439815284602082015203019082f01561542b5780f35b615d9291505f90615fbd565b5f5f615d3a565b6040513d5f823e3d90fd5b60206040818301928281528451809452019201905f5b818110615dc75750505090565b82516001600160a01b0316845260209384019390920191600101615dba565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b818110615e285750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101615e1b565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310615e9257505050505090565b9091929394602080615ece837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951615de6565b97019301930191939290615e83565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310615f0f57505050505090565b9091929394602080615f65837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b03815116845201519181858201520190615e0b565b97019301930191939290615f00565b6040810190811067ffffffffffffffff821117615f9057604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff821117615f9057604052565b67ffffffffffffffff8111615f905760051b60200190565b90604051915f8154908160011c92600183169283156160f1575b6020851084146160c45784875286939081156160845750600114616040575b5061603e92500383615fbd565b565b90505f9291925260205f20905f915b81831061606857505090602061603e928201015f616031565b602091935080600191548385890101520191019091849261604f565b6020935061603e9592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f616031565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f1693616012565b6040519061610a604083615fbd565b600582527f75736572330000000000000000000000000000000000000000000000000000006020830152565b9190820180921161614357565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b908160209103126104ae575180151581036104ae5790565b9190820391821161614357565b90604051918281549182825260208201905f5260205f20925f905b8060078301106163ac5761603e945491818110616376575b818110616340575b81811061630a575b8181106162d4575b81811061629e575b818110616268575b818110616233575b10616206575b500383615fbd565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f6161fe565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b1681520193016161f8565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b1681520193016161f0565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b1681520193016161e8565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b1681520193016161e0565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b1681520193016161d8565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b1681520193016161d0565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b1681520193016161c8565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e08201520194019201859293916161b0565b6020818303126104ae5780519067ffffffffffffffff82116104ae570181601f820112156104ae5780519067ffffffffffffffff8211615f90576040519261648b601f8401601f191660200185615fbd565b828452602083830101116104ae57815f9260208093018386015e8301015290565b519060ff821682036104ae57565b908160209103126104ae57516001600160a01b03811681036104ae5790565b908160609103126104ae576164ed816164ac565b916040602083015192015190565b60085460ff16801561650a5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615d99575f916165a2575b50151590565b90506020813d6020116165cc575b816165bd60209383615fbd565b810103126104ae57515f61659c565b3d91506165b0565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b5f61603e91615fbd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152683635c9adc5dea0000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526825f273933db570000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152681043561a882930000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152681b1ae4d6e2ef50000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b6fffffffffffffffffffffffffffffffff60016168e292616ab3565b905f806040516168f3604082615fbd565b600c81527f426f756e6420726573756c740000000000000000000000000000000000000000602082015260405161696081612bee60208201947fb60e72cc000000000000000000000000000000000000000000000000000000008652604060248401526064830190615de6565b51906a636f6e736f6c652e6c6f675afa50565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae576169d25f916169e460405194859384937ff320d963000000000000000000000000000000000000000000000000000000008552604060048601526044850190615de6565b90600319848303016024850152615de6565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b8115616a86570690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f90838311616bf25782811091821580616be8575b616be057616ad68486616188565b926001840180941161614357600383111580616bd7575b616bc85760031983101580616bbe575b616baa5785831115616b6157505090616b1984616b1e93616188565b616a7c565b908115616b5c57616b2f9250616136565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116161435790565b505090565b959492919095616b72575b50505050565b83949550616b1990616b849394616188565b908115616b5c57616b959250616188565b6001810180911161614357905f808080616b6c565b50509050616bbb9291501990616188565b90565b5082198411616afd565b5050919050616bbb9250616136565b50828411616aed565b509250505090565b5084821115616ac8565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152fdfe61016080604052346104b857604081613013803803809161002082856104bc565b8339810103126104b85761003f6020610038836104df565b92016104df565b60405161004d6040826104bc565b601181526020810170546573746e65742053796e64696361746560781b81526040519061007b6040836104bc565b6011825270546573746e65742053796e64696361746560781b6020830152604051926100a86040856104bc565b600b84526a15195cdd1b995d14d6539160aa1b6020850152604051936100cf6040866104bc565b60018552603160f81b60208601908152845190946001600160401b0382116103bb5760035490600182811c921680156104ae575b602083101461039d5781601f849311610440575b50602090601f83116001146103da575f926103cf575b50508160011b915f199060031b1c1916176003555b8051906001600160401b0382116103bb5760045490600182811c921680156103b1575b602083101461039d5781601f84931161032f575b50602090601f83116001146102c9575f926102be575b50508160011b915f199060031b1c1916176004555b6101ad816105fc565b610120526101ba84610783565b61014052519020918260e05251902080610100524660a0526040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8452604083015260608201524660808201523060a082015260a0815261022360c0826104bc565b5190206080523060c0526001600160a01b038216156102af576001600160a01b038116156102af5761025761025d926104f3565b50610569565b506040516126f790816108bc8239608051816117b9015260a05181611876015260c0518161178a015260e051816118080152610100518161182e01526101205181610adc01526101405181610b050152f35b63d92e233d60e01b5f5260045ffd5b015190505f8061018f565b60045f9081528281209350601f198516905b81811061031757509084600195949392106102ff575b505050811b016004556101a4565b01515f1960f88460031b161c191690555f80806102f1565b929360206001819287860151815501950193016102db565b60045f529091507f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b601f840160051c81019160208510610393575b90601f859493920160051c01905b8181106103855750610179565b5f8155849350600101610378565b909150819061036a565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610165565b634e487b7160e01b5f52604160045260245ffd5b015190505f8061012d565b60035f9081528281209350601f198516905b8181106104285750908460019594939210610410575b505050811b01600355610142565b01515f1960f88460031b161c191690555f8080610402565b929360206001819287860151815501950193016103ec565b60035f529091507fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b601f840160051c810191602085106104a4575b90601f859493920160051c01905b8181106104965750610117565b5f8155849350600101610489565b909150819061047b565b91607f1691610103565b5f80fd5b601f909101601f19168101906001600160401b038211908210176103bb57604052565b51906001600160a01b03821682036104b857565b6001600160a01b0381165f9081525f516020612ff35f395f51905f52602052604090205460ff16610564576001600160a01b03165f8181525f516020612ff35f395f51905f5260205260408120805460ff191660011790553391905f516020612fb35f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f516020612fd35f395f51905f52602052604090205460ff16610564576001600160a01b03165f8181525f516020612fd35f395f51905f5260205260408120805460ff191660011790553391907f9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a6905f516020612fb35f395f51905f529080a4600190565b908151602081105f14610676575090601f815111610636576020815191015160208210610627571790565b5f198260200360031b1b161790565b604460209160405192839163305a27a960e01b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fd5b6001600160401b0381116103bb57600654600181811c91168015610779575b602082101461039d57601f8111610746575b50602092601f82116001146106e557928192935f926106da575b50508160011b915f199060031b1c19161760065560ff90565b015190505f806106c1565b601f1982169360065f52805f20915f5b86811061072e5750836001959610610716575b505050811b0160065560ff90565b01515f1960f88460031b161c191690555f8080610708565b919260206001819286850151815501940192016106f5565b60065f52601f60205f20910160051c810190601f830160051c015b81811061076e57506106a7565b5f8155600101610761565b90607f1690610695565b908151602081105f146107ae575090601f815111610636576020815191015160208210610627571790565b6001600160401b0381116103bb57600754600181811c911680156108b1575b602082101461039d57601f811161087e575b50602092601f821160011461081d57928192935f92610812575b50508160011b915f199060031b1c19161760075560ff90565b015190505f806107f9565b601f1982169360075f52805f20915f5b868110610866575083600195961061084e575b505050811b0160075560ff90565b01515f1960f88460031b161c191690555f8080610840565b9192602060018192868501518155019401920161082d565b60075f52601f60205f20910160051c810190601f830160051c015b8181106108a657506107df565b5f8155600101610899565b90607f16906107cd56fe60806040526004361015610011575f80fd5b5f3560e01c806301ffc9a71461023557806306fdde0314610230578063095ea7b31461022b57806318160ddd146101b857806323b872dd14610226578063248a9ca3146102215780632f2ff15d1461021c578063313ce567146102175780633644e5151461021257806336568abe1461020d5780633a46b1a8146101c257806340c10f19146102085780634bf5d7e914610203578063587cde1e146101fe5780635c19a95c146101f95780636fcfff45146101f457806370a08231146101ef5780637ecebe00146101ea57806384b0196e146101e55780638e539e8c146101e057806391d14854146101db57806391ddadf4146101d657806395d89b41146101d15780639ab24eb0146101bd578063a217fddf146101cc578063a9059cbb146101c7578063b0ca253e146101c2578063bb4d4436146101bd578063c02ae754146101b8578063c3cda520146101b3578063d505accf146101ae578063d5391393146101a9578063d547741f146101a4578063dd62ed3e1461019f5763f1127ed81461019a575f80fd5b6111ec565b611193565b611155565b61111b565b610fc1565b610e7a565b610486565b610df7565b610672565b610e34565b610e1a565b610d52565b610d27565b610cd7565b610bfb565b610ac4565b610a8c565b610a57565b6109dc565b6109ba565b610979565b6108d0565b610784565b610615565b6105fb565b6105e0565b61059b565b610568565b6104a3565b610455565b610331565b346102d65760206003193601126102d6576004357fffffffff0000000000000000000000000000000000000000000000000000000081168091036102d657807f7965db0b00000000000000000000000000000000000000000000000000000000602092149081156102ac575b506040519015158152f35b7f01ffc9a7000000000000000000000000000000000000000000000000000000009150145f6102a1565b5f80fd5b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b90602061032e9281815201906102da565b90565b346102d6575f6003193601126102d6576040515f600354610351816112b5565b80845290600181169081156103e75750600114610389575b61038583610379818503826113f4565b6040519182918261031d565b0390f35b60035f9081527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b939250905b8082106103cd57509091508101602001610379610369565b9192600181602092548385880101520191019092916103b5565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506103799050610369565b600435906001600160a01b03821682036102d657565b602435906001600160a01b03821682036102d657565b346102d65760406003193601126102d65761047b610471610429565b6024359033611b03565b602060405160018152f35b346102d6575f6003193601126102d6576020600254604051908152f35b346102d65760606003193601126102d6576104bc610429565b6104c461043f565b604435906001600160a01b0383165f5260016020526104f73360405f20906001600160a01b03165f5260205260405f2090565b54925f198410610518575b61050c9350611499565b60405160018152602090f35b8284106105345761052f8361050c95033383611bd1565b610502565b82847ffb8f41b2000000000000000000000000000000000000000000000000000000005f523360045260245260445260645ffd5b346102d65760206003193601126102d65760206105936004355f526005602052600160405f20015490565b604051908152f35b346102d65760406003193601126102d6576105de6004356105ba61043f565b906105d96105d4825f526005602052600160405f20015490565b611667565b6116c8565b005b346102d6575f6003193601126102d657602060405160128152f35b346102d6575f6003193601126102d6576020610593611780565b346102d65760406003193601126102d65760043561063161043f565b336001600160a01b0382160361064a576105de9161189c565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102d65760406003193601126102d65761068b610429565b6001600160a01b0360243591165f52600a6020526106ac60405f209161194c565b8154905f82916005841161072c575b6106c6935084611e0c565b806106f5575050602079ffffffffffffffffffffffffffffffffffffffffffffffffffff5f5b16604051908152f35b60209161071c79ffffffffffffffffffffffffffffffffffffffffffffffffffff926119cb565b905f52825f20015460301c6106ec565b919261073781611c97565b810390811161077f576106c693855f5265ffffffffffff8260205f2001541665ffffffffffff8516105f1461076d5750916106bb565b929150610779906119d9565b906106bb565b61199e565b346102d65760406003193601126102d65761079d610429565b6024356107a86115df565b6001600160a01b03821680156108a8578115610880576107d26107cd836002546119e7565b600255565b6107ec836001600160a01b03165f525f60205260405f2090565b8054830190556040518281525f907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a36002549179ffffffffffffffffffffffffffffffffffffffffffffffffffff808411610850576105de8383612436565b7f1cb15d26000000000000000000000000000000000000000000000000000000005f52600484905260245260445ffd5b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102d6575f6003193601126102d6576108e943611c18565b65ffffffffffff806108fa43611c18565b16911603610951576103856040516109136040826113f4565b601d81527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c7400000060208201526040519182916020835260208301906102da565b7f6ff07140000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102d65760206003193601126102d6576001600160a01b0361099a610429565b165f52600960205260206001600160a01b0360405f205416604051908152f35b346102d65760206003193601126102d6576105de6109d6610429565b336119f4565b346102d65760206003193601126102d6576001600160a01b036109fd610429565b165f52600a60205260405f205463ffffffff8111610a275760405163ffffffff9091168152602090f35b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52602060045260245260445ffd5b346102d65760206003193601126102d6576020610593610a75610429565b6001600160a01b03165f525f60205260405f205490565b346102d65760206003193601126102d6576001600160a01b03610aad610429565b165f526008602052602060405f2054604051908152f35b346102d6575f6003193601126102d657610ba2610b007f0000000000000000000000000000000000000000000000000000000000000000611fc3565b610b297f000000000000000000000000000000000000000000000000000000000000000061203c565b6020604051610b3882826113f4565b5f815281610bb0818301947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe083013687376040519788977f0f00000000000000000000000000000000000000000000000000000000000000895260e0858a015260e08901906102da565b9087820360408901526102da565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b828110610be457505050500390f35b835185528695509381019392810192600101610bd5565b346102d65760206003193601126102d657610c1760043561194c565b600b54905f829160058411610c83575b610c339350600b611e0c565b80610c61575060205f5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff60405191168152f35b610c7e610c6f6020926119cb565b600b5f52825f20015460301c90565b610c3d565b9192610c8e81611c97565b810390811161077f57610c3393600b5f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14610cc5575091610c27565b929150610cd1906119d9565b90610c27565b346102d65760406003193601126102d657602060ff610d1b600435610cfa61043f565b905f526005845260405f20906001600160a01b03165f5260205260405f2090565b54166040519015158152f35b346102d6575f6003193601126102d6576020610d4243611c18565b65ffffffffffff60405191168152f35b346102d6575f6003193601126102d6576040515f600454610d72816112b5565b80845290600181169081156103e75750600114610d995761038583610379818503826113f4565b60045f9081527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b939250905b808210610ddd57509091508101602001610379610369565b919260018160209254838588010152019101909291610dc5565b346102d65760206003193601126102d6576020610593610e15610429565b611446565b346102d6575f6003193601126102d65760206040515f8152f35b346102d65760406003193601126102d65761047b610e50610429565b6024359033611499565b6064359060ff821682036102d657565b6084359060ff821682036102d657565b346102d65760c06003193601126102d657610e93610429565b60243590604435610ea2610e5a565b6084359060a43592804211610f965791610f289391610f1a610f1f9460405160208101917fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf83526001600160a01b038a1660408301528a6060830152608082015260808152610f1260a0826113f4565b519020611ab3565b612073565b90929192612137565b610f4c816001600160a01b03165f52600860205260405f2080549060018201905590565b809303610f5d576105de92506119f4565b6001600160a01b0391507f752d88c0000000000000000000000000000000000000000000000000000000005f521660045260245260445ffd5b7f4683af0e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346102d65760e06003193601126102d657610fda610429565b610fe261043f565b6044359060643592610ff2610e6a565b60a43560c435908642116110ef5761109b9261109661102b866001600160a01b03165f52600860205260405f2080549060018201905590565b9860405160208101917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983526001600160a01b0389169b8c60408401526001600160a01b038b1660608401528b608084015260a083015260c082015260c08152610f1260e0826113f4565b611af4565b936001600160a01b038516036110b5576105de9350611b03565b7f4b800e46000000000000000000000000000000000000000000000000000000005f526001600160a01b038085166004521660245260445ffd5b867f62791302000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346102d6575f6003193601126102d65760206040517f9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a68152f35b346102d65760406003193601126102d6576105de60043561117461043f565b9061118e6105d4825f526005602052600160405f20015490565b61189c565b346102d65760406003193601126102d65760206111e36111b1610429565b6001600160a01b036111c161043f565b91165f526001835260405f20906001600160a01b03165f5260205260405f2090565b54604051908152f35b346102d65760406003193601126102d657611205610429565b6024359063ffffffff821682036102d657610385916001600160a01b036112529261122e611481565b50611237611481565b50165f52600a60205260405f2061124c611481565b506121fe565b5060405190611260826113d3565b5465ffffffffffff8116825260301c602082015260405191829182919091602079ffffffffffffffffffffffffffffffffffffffffffffffffffff81604084019565ffffffffffff8151168552015116910152565b90600182811c921680156112fc575b60208310146112cf57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f16916112c4565b5f9291815491611315836112b5565b808352926001811690811561136a575060011461133157505050565b5f9081526020812093945091925b838310611350575060209250010190565b60018160209294939454838587010152019101919061133f565b905060209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040810190811067ffffffffffffffff8211176113ef57604052565b6113a6565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176113ef57604052565b604051906114446040836113f4565b565b6001600160a01b03165f52600a60205279ffffffffffffffffffffffffffffffffffffffffffffffffffff61147d60405f20611a8a565b1690565b6040519061148e826113d3565b5f6020838281520152565b9291906001600160a01b0384169384156115b3576001600160a01b0382168015611587576114d7826001600160a01b03165f525f60205260405f2090565b54848110611553579584611444969703611501846001600160a01b03165f525f60205260405f2090565b5561151c846001600160a01b03165f525f60205260405f2090565b8054860190556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a36124b9565b8490877fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd5b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b335f9081527f15a28d26fa1bf736cf7edc9922607171ccb09c3c73b808e7772a3013e068a522602052604090205460ff161561161757565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a660245260445ffd5b805f52600560205260ff61168f3360405f20906001600160a01b03165f5260205260405f2090565b5416156116995750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f52600560205260ff6116f08360405f20906001600160a01b03165f5260205260405f2090565b541661177a57805f52600560205261171c8260405f20906001600160a01b03165f5260205260405f2090565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008254161790556001600160a01b03339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016301480611873575b156117db577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a0815261186d60c0826113f4565b51902090565b507f000000000000000000000000000000000000000000000000000000000000000046146117b2565b805f52600560205260ff6118c48360405f20906001600160a01b03165f5260205260405f2090565b54161561177a57805f5260056020526118f18260405f20906001600160a01b03165f5260205260405f2090565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690556001600160a01b03339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b65ffffffffffff61195c43611c18565b168082101561196f575061032e90611c18565b907fecd3f81e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b905f19820191821161077f57565b906001820180921161077f57565b9190820180921161077f57565b6001600160a01b038181165f81815260096020526040812080548685167fffffffffffffffffffffffff00000000000000000000000000000000000000008216811790925561144496941694611a849390928691907f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9080a46001600160a01b03165f525f60205260405f205490565b91611e70565b805480611a975750505f90565b805f1981011161077f575f19915f5260205f2001015460301c90565b604290611abe611780565b90604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b9161032e9391610f1f93612073565b6001600160a01b0316908115611ba5576001600160a01b038116928315611b795780611b6c7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593855f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55604051908152602090a3565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b0316908115611ba5576001600160a01b03811615611b7957611c15915f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55565b65ffffffffffff8111611c305765ffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52603060045260245260445ffd5b8115611c6a570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b600181111561032e57806001700100000000000000000000000000000000831015611dca575b611d70611d66611d5c611d52611d48611d3e611d2d611d779760048a68010000000000000000611d7c9c1015611dbd575b640100000000811015611db0575b62010000811015611da3575b610100811015611d96575b6010811015611d89575b1015611d81575b60030260011c90565b611d37818b611c60565b0160011c90565b611d37818a611c60565b611d378189611c60565b611d378188611c60565b611d378187611c60565b611d378186611c60565b8093611c60565b821190565b900390565b60011b611d24565b60041c9160021b91611d1d565b60081c9160041b91611d13565b60101c9160081b91611d08565b60201c9160101b91611cfc565b60401c9160201b91611cee565b5050611d7c611d77611d70611d66611d5c611d52611d48611d3e611d2d611df18a60801c90565b9850680100000000000000009750611cbd9650505050505050565b91905b838210611e1c5750505090565b9091928083169080841860011c820180921161077f57845f5265ffffffffffff8260205f2001541665ffffffffffff8416105f14611e5e5750925b9190611e0f565b939250611e6a906119d9565b91611e57565b91906001600160a01b038116926001600160a01b038116908482141580611fba575b611e9e575b5050505050565b81611f44575b505082611eb3575b8080611e97565b611f39611f207fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72493611f1a611f1479ffffffffffffffffffffffffffffffffffffffffffffffffffff956001600160a01b03165f52600a60205260405f2090565b91612240565b90612314565b6040805192851683529316602082015291829190820190565b0390a25f8080611eac565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff611fb0611f20611fa17fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724946001600160a01b03165f52600a60205260405f2090565b611faa88612240565b906122b0565b0390a25f80611ea4565b50831515611e92565b60ff81146120225760ff811690601f8211611ffa5760405191611fe76040846113f4565b6020808452838101919036833783525290565b7fb3512b0c000000000000000000000000000000000000000000000000000000005f5260045ffd5b5060405161032e81612035816006611306565b03826113f4565b60ff81146120605760ff811690601f8211611ffa5760405191611fe76040846113f4565b5060405161032e81612035816007611306565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116120f5579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa156120ea575f516001600160a01b038116156120e057905f905f90565b505f906001905f90565b6040513d5f823e3d90fd5b5050505f9160039190565b6004111561210a57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b61214081612100565b80612149575050565b61215281612100565b60018103612182577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b61218b81612100565b600281036121bf57507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b806121cb600392612100565b146121d35750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b8054821015612213575f5260205f2001905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff81116122805779ffffffffffffffffffffffffffffffffffffffffffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f5260d060045260245260445ffd5b906122ba43611c18565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff806122e085611a8a565b92169116039079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161077f57612310926125c4565b9091565b9061231e43611c18565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff8061234485611a8a565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161077f57612310926125c4565b61237d43611c18565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff806123a4600b611a8a565b921691160179ffffffffffffffffffffffffffffffffffffffffffffffffffff811161077f5761231091600b6125c4565b6123de43611c18565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80612405600b611a8a565b921691160379ffffffffffffffffffffffffffffffffffffffffffffffffffff811161077f5761231091600b6125c4565b906001600160a01b036114449261245461244f84612240565b612374565b50501680156124a1575b60096020527fec8156718a8372b1db44bb411437d0870f3e3790d4a08526d024ce1b0b668f6b545f9182526040909120546001600160a01b039081169116611e70565b6124b26124ad83612240565b6123d5565b505061245e565b906001600160a01b038061144494931691821561251e575b1690811561250b575b5f5260096020526001600160a01b0360405f205416905f5260096020526001600160a01b0360405f20541690611e70565b6125176124ad84612240565b50506124da565b61252a61244f85612240565b50506124d1565b8054680100000000000000008110156113ef57612553916001820181556121fe565b6125985781516020929092015160301b7fffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000001665ffffffffffff92909216919091179055565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b805492939280156126ba576125db6125e6916119cb565b825f5260205f200190565b8054603081901c9365ffffffffffff918216929181168084116126925787930361264b575061264792509065ffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000083549260301b169116179055565b9190565b9150506126479161266b61265d611435565b65ffffffffffff9093168352565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff86166020830152612531565b7f2520601d000000000000000000000000000000000000000000000000000000005f5260045ffd5b50906126f2916126cb61265d611435565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff85166020830152612531565b5f9190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d15a28d26fa1bf736cf7edc9922607171ccb09c3c73b808e7772a3013e068a52205b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`/W`\x01`\xFF\x19`\x0CT\x16\x17`\x0CU`\x01`\xFF\x19`\x1FT\x16\x17`\x1FUa\x9C\x8A\x90\x81a\x004\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x04\x81 T\x14a\\\x9CWP\x80c\x06B\xDD\xE5\x14a\\%W\x80c\x07Tar\x14a[\xFEW\x80c\n\x92T\xE4\x14aTyW\x80c\x10t\xA2\x1F\x14aS=W\x80c\x12:J_\x14aQ\xB8W\x80c\x1E\xD7\x83\x1C\x14aQ:W\x80c\"F\xE5\xCC\x14aL\x13W\x80c*\xDE8\x80\x14aJ\x1FW\x80c>^<#\x14aI\xA1W\x80c?r\x86\xF4\x14aI#W\x80cAho\xF2\x14aB6W\x80cGH<]\x14a=QW\x80cO\x862\xBA\x14a=*W\x80cU\xF7\xD4w\x14a:\x80W\x80c[\xB1w\x81\x14a5KW\x80cc8\xAA\x86\x14a2\xC1W\x80cd\x0FrZ\x14a0(W\x80cf\xD9\xA9\xA0\x14a.\xEBW\x80cq\xD7\xDA\xBF\x14a+HW\x80ctj\x9B\xCF\x14a)\xC7W\x80cv\x02\x9Ex\x14a&\xCFW\x80c\x84\xEF\x8F\xFC\x14a&\xA9W\x80c\x85\"l\x81\x14a&\x1FW\x80c\x88\xC5g\x1B\x14a\"IW\x80c\x8D1\xEDS\x14a\x1EsW\x80c\x8F\x08\xEC\xE7\x14a\x1B{W\x80c\x8F1\r\xFE\x14a\x17\x82W\x80c\x91j\x17\xC6\x14a\x16\xD8W\x80c\x91\xDC\x0B-\x14a\x13RW\x80c\xB0FO\xDC\x14a\x12\xA8W\x80c\xB5P\x8A\xA9\x14a\x12\x1EW\x80c\xB7\x8BYg\x14a\r\x1BW\x80c\xBAAO\xA6\x14a\x0C\xF6W\x80c\xBB\xB1U3\x14a\x07\x9FW\x80c\xC0\x1E\x94(\x14a\x04\xF8W\x80c\xDC\xCCW\xF1\x14a\x02\xE5W\x80c\xE2\x0C\x9Fq\x14a\x02WW\x80c\xE8\xED\xC8\x16\x14a\x020W\x80c\xFAv&\xD4\x14a\x02\rWc\xFC\x0CTj\x14a\x01\xE1W_\x80\xFD[4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[\x80\xFD[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x02\xC6Wa\x02\xC2\x85a\x02\xB6\x81\x87\x03\x82a_\xBDV[`@Q\x91\x82\x91\x82a]\xA4V[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x02\x9FV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x04\xC5W[P` \x80T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R\x81`D\x81\x85Z\xFA\x80\x15a\x04\xBAWa\x03\xA1\x91\x84\x91a\x04HW[Paj\nV[`@Q\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x04\x82W[P`!T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01[\x03\x91Z\xFA\x80\x15a\x04wWa\x04E\x91\x83\x91a\x04HWPaj\nV[\x80\xF3[a\x04j\x91P` =` \x11a\x04pW[a\x04b\x81\x83a_\xBDV[\x81\x01\x90aapV[_a\x03\x9BV[P=a\x04XV[`@Q=\x84\x82>=\x90\xFD[\x90P` \x81=` \x11a\x04\xB2W[\x81a\x04\x9D` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQa\x04+a\x03\xDEV[_\x80\xFD[=\x91Pa\x04\x90V[`@Q=\x85\x82>=\x90\xFD[\x90P` \x81=` \x11a\x04\xF0W[\x81a\x04\xE0` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ` a\x03BV[=\x91Pa\x04\xD3V[P4a\x02\nW`@`\x03\x196\x01\x12a\x02\nW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x07mWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07mW\x82`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x07\x8AW[Pa\x05\x9C`$5ah\xC6V[\x91`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07mW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04\xBAW\x83\x91a\x07qW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07mW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04wWa\x07XW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x07MW\x84\x91a\x07\x1AW[P`\x04\x92a\x06\xA3\x83` \x93ahPV[`@Q\x93\x84\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x04\xBAW\x83\x90a\x06\xE6W[a\x04E\x92PahPV[P` \x82=` \x11a\x07\x12W[\x81a\x07\0` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x91Qa\x06\xDCV[=\x91Pa\x06\xF3V[\x90P` \x81=` \x11a\x07EW[\x81a\x075` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ`\x04a\x06\x93V[=\x91Pa\x07(V[`@Q=\x86\x82>=\x90\xFD[\x81a\x07b\x91a_\xBDV[a\x07mW\x82_a\x06WV[\x82\x80\xFD[\x81a\x07{\x91a_\xBDV[a\x07\x86W\x81_a\x06\x02V[P\x80\xFD[\x81a\x07\x94\x91a_\xBDV[a\x07mW\x82_a\x05\x90V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rb\n\x11\xCE`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x04wW\x82\x91a\x0C\xC7W[Pa\x0E\x10B\x01\x90\x81B\x11a\x0BWW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91\x83`@Q\x93\x7F~\xCE\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92\x83`\x04\x87\x01R` \x86`$\x81\x85Z\xFA\x95\x86\x15a\x04\xBAW\x83\x96a\x0C\x90W[P`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q` \x81\x01\x90a\t\x04\x81a\x08\xF6\x85\x8C\x88\x8D\x88\x90\x94\x93\x92`\x01`\x01`\xA0\x1B\x03`\xA0\x93\x81`\xC0\x85\x01\x98\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x86R\x16` \x85\x01R\x16`@\x83\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0``\x83\x01R`\x80\x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a_\xBDV[Q\x90 `@Q\x90\x7F6D\xE5\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x88Z\xFA\x91\x82\x15a\x0C\x85W\x86\x92a\x0CLW[P`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01\x90\x81R`\"\x82\x01\x93\x90\x93R`B\x81\x01\x91\x90\x91Ra\t\x8E\x81`b\x81\x01a\x08\xF6V[Q\x90 \x92`@Q\x93\x7F\xE3A\xEA\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85Rb\n\x11\xCE`\x04\x86\x01R`$\x85\x01R``\x84`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x92\x83\x15a\x0B\xECW\x85\x94\x86\x90\x87\x95a\x0C\x14W[P\x82;\x15a\x0C\x10W\x88a\n}\x88\x96\x87\x93`@Q\x99\x8A\x98\x89\x97\x88\x96\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R`\x04\x88\x01\x92\x93`\xC0\x94`\x01`\x01`\xA0\x1B\x03`\xFF\x93\x99\x98\x97\x94\x81`\xE0\x88\x01\x9B\x16\x87R\x16` \x86\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`@\x86\x01R``\x85\x01R\x16`\x80\x83\x01R`\xA0\x82\x01R\x01RV[\x03\x92Z\xF1\x80\x15a\x04wWa\x0B\xF7W[PP`\x1FT`#T`@Q\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R`\x08\x91\x90\x91\x1C\x90\x92\x16\x91\x90` \x82`D\x81\x86Z\xFA\x91\x82\x15a\x0B\xECW\x85\x92a\x0B\xB6W[Pa\x0B\0` \x92afTV[`$`@Q\x80\x94\x81\x93\x7F~\xCE\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x0B\x84W[P`\x01\x82\x01\x80\x92\x11a\x0BWW\x90a\x04E\x91ahPV[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x0B\xAEW[\x81a\x0B\x9F` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ_a\x0BAV[=\x91Pa\x0B\x92V[\x91P` \x82=` \x11a\x0B\xE4W[\x81a\x0B\xD1` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x90a\x0B\0a\n\xF4V[=\x91Pa\x0B\xC4V[`@Q=\x87\x82>=\x90\xFD[\x81a\x0C\x01\x91a_\xBDV[a\x0C\x0CW\x83_a\n\x8CV[\x83\x80\xFD[\x86\x80\xFD[\x91\x95PPa\x0C;\x91\x93P``=``\x11a\x0CEW[a\x0C3\x81\x83a_\xBDV[\x81\x01\x90ad\xD9V[\x93\x91\x94\x90_a\t\xF6V[P=a\x0C)V[\x95P\x90P` \x85=` \x11a\x0C}W[\x81a\x0Ci` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x93Q\x88\x94a\t\x8Ea\tEV[=\x91Pa\x0C\\V[`@Q=\x88\x82>=\x90\xFD[\x92P\x94P` \x82=` \x11a\x0C\xBFW[\x81a\x0C\xAD` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x85\x91Q\x94_a\x08xV[=\x91Pa\x0C\xA0V[a\x0C\xE9\x91P` =` \x11a\x0C\xEFW[a\x0C\xE1\x81\x83a_\xBDV[\x81\x01\x90ad\xBAV[_a\x08\nV[P=a\x0C\xD7V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` a\r\x11ad\xFBV[`@Q\x90\x15\x15\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Rb\n\x11\xCE`\x04\x83\x01R` \x82`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x91\x82\x15a\x12\x11W\x81\x92a\x11\xF0W[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x01\x91B\x83\x11a\x11\xC3W\x81\x92`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F~\xCE\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x0B\xECW\x85\x91a\x11\x8BW[P\x90` `\x04\x92\x84a\x0E\x93`\x01`\x01`\xA0\x1B\x03`#T\x16\x92a\x08\xF6`@Q\x93\x84\x92\x87\x84\x01\x96\x8C\x88\x90\x94\x93\x92`\x01`\x01`\xA0\x1B\x03`\xA0\x93\x81`\xC0\x85\x01\x98\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x86R\x16` \x85\x01R\x16`@\x83\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0``\x83\x01R`\x80\x82\x01R\x01RV[Q\x90 \x91`@Q\x93\x84\x80\x92\x7F6D\xE5\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x0B\xECW\x85\x92a\x11RW[P`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01\x90\x81R`\"\x82\x01\x93\x90\x93R`B\x81\x01\x91\x90\x91Ra\x0F\x1A\x81`b\x81\x01a\x08\xF6V[Q\x90 \x90`@Q\x91\x7F\xE3A\xEA\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rb\n\x11\xCE`\x04\x84\x01R`$\x83\x01R``\x82`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x07MW\x84\x93\x85\x93\x86\x92a\x11(W[P`@Q\x7Fby\x13\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x83`$\x82\x01R`$\x81Ra\x0F\xC0`D\x82a_\xBDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\x10W\x86a\x10\x1B\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x11\x1DW\x87\x91a\x11\x04W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91`\x01`\x01`\xA0\x1B\x03`#T\x16\x91\x83;\x15a\x11\0W`@Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x16`$\x83\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`D\x83\x01R`d\x82\x01\x93\x90\x93R`\xFF\x94\x90\x94\x16`\x84\x85\x01R`\xA4\x84\x01\x92\x90\x92R`\xC4\x83\x01R\x82\x90\x82\x90`\xE4\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04wWa\x10\xEFWP\xF3[\x81a\x10\xF9\x91a_\xBDV[a\x02\nW\x80\xF3[\x87\x80\xFD[\x81a\x11\x0E\x91a_\xBDV[a\x11\x19W\x85_a\x10CV[\x85\x80\xFD[`@Q=\x89\x82>=\x90\xFD[\x91P\x93Pa\x11F\x91\x92P``=``\x11a\x0CEWa\x0C3\x81\x83a_\xBDV[\x92\x91\x93\x90\x92\x90_a\x0F\x81V[\x94P\x90P` \x84=` \x11a\x11\x83W[\x81a\x11o` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x92Q\x84\x93a\x0F\x1Aa\x0E\xD1V[=\x91Pa\x11bV[\x91\x94PP` \x81=` \x11a\x11\xBBW[\x81a\x11\xA8` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ\x84\x93\x90` a\x0E\x10V[=\x91Pa\x11\x9BV[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[a\x12\n\x91\x92P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[\x90_a\r\x87V[P`@Q\x90=\x90\x82>=\x90\xFD[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x19Ta\x12;\x81a_\xE0V[\x91a\x12I`@Q\x93\x84a_\xBDV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x12\x8BW`@Q\x80a\x02\xC2\x87\x82a^`V[`\x01` \x81\x92a\x12\x9A\x85a_\xF8V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x12vV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1CTa\x12\xC5\x81a_\xE0V[\x91a\x12\xD3`@Q\x93\x84a_\xBDV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x13\x15W`@Q\x80a\x02\xC2\x87\x82a^\xDDV[`\x02` `\x01\x92`@Qa\x13(\x81a_tV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x13@\x85\x87\x01aa\x95V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x13\0V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x16\xC3W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x16\xAEW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x16\x96W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x16&W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\\\x19\xA9\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x04wWa\x16\x81W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q\x7F\x9A\xB2N\xB0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x07MW\x84\x91a\x16IW[Pa\x15U\x90afTV[` `\x01`\x01`\xA0\x1B\x03`\"T\x16`$`@Q\x80\x94\x81\x93\x7FX|\xDE\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x16*W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16&W`@Q\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x10\xEFWP\xF3[PP\xFD[a\x16C\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_a\x15\xA4V[\x93PP` \x83=` \x11a\x16yW[\x81a\x16e` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x15U\x84\x93Q\x90a\x15KV[=\x91Pa\x16XV[\x81a\x16\x8B\x91a_\xBDV[a\x02\nW\x80_a\x14\xEBV[\x81a\x16\xA0\x91a_\xBDV[a\x02\nW\x80_a\x14\x85V[P\xFD[\x81a\x16\xB8\x91a_\xBDV[a\x02\nW\x80_a\x14\"V[\x81a\x16\xCD\x91a_\xBDV[a\x02\nW\x80_a\x13\xC6V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1DTa\x16\xF5\x81a_\xE0V[\x91a\x17\x03`@Q\x93\x84a_\xBDV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x17EW`@Q\x80a\x02\xC2\x87\x82a^\xDDV[`\x02` `\x01\x92`@Qa\x17X\x81a_tV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x17p\x85\x87\x01aa\x95V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x170V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x1BfW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x1BQW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh\x10CV\x1A\x88)0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x1B<W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x1B'W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x1A\xEFW[Pa\x19\x83\x90ag\xD1V[`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x1A\xB7W[P`\x04\x91a\x19\xC8` \x92agRV[`@Q\x92\x83\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04wW\x82\x91a\x1A\x82W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh+^:\xF1k\x18\x80\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x10\xEFWP\xF3[\x91PP` \x81=` \x11a\x1A\xAFW[\x81a\x1A\x9E` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x81\x90Q_a\x1A\x02V[=\x91Pa\x1A\x91V[\x92PP` \x82=` \x11a\x1A\xE7W[\x81a\x1A\xD3` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x82\x91\x90`\x04a\x19\xB9V[=\x91Pa\x1A\xC6V[\x92PP` \x82=` \x11a\x1B\x1FW[\x81a\x1B\x0B` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x19\x83\x83\x92Q\x90a\x19yV[=\x91Pa\x1A\xFEV[\x81a\x1B1\x91a_\xBDV[a\x02\nW\x80_a\x193V[\x81a\x1BF\x91a_\xBDV[a\x02\nW\x80_a\x18\xC7V[\x81a\x1B[\x91a_\xBDV[a\x02\nW\x80_a\x18kV[\x81a\x1Bp\x91a_\xBDV[a\x02\nW\x80_a\x18\x0FV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x1E^W[P`\x04`\x01`\x01`\xA0\x1B\x03`\"T\x16` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x04\xBAW\x83\x92a\x1E%W[P`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Ra\x1C\xB1\x81`d\x81\x01a\x08\xF6V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW\x81a\x1D\x0C\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x1E\x10W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x1D\xFBW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x10\xEFWP\xF3[\x81a\x1E\x05\x91a_\xBDV[a\x02\nW\x80_a\x1D\x8DV[\x81a\x1E\x1A\x91a_\xBDV[a\x02\nW\x80_a\x1D1V[\x92P\x90P` \x82=` \x11a\x1EVW[\x81a\x1EB` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x82\x91a\x1C\xB1a\x1CbV[=\x91Pa\x1E5V[\x81a\x1Eh\x91a_\xBDV[a\x02\nW\x80_a\x1C\x08V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\"\x11W[P` \x80T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01\x81\x90R\x92\x82\x90`D\x90\x82\x90Z\xFA\x80\x15a\x04\xBAWa\x1F4\x91\x84\x91a\x04HWPaj\nV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa!\xFCW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a!\xC7W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16\x82;\x15a!\xC2W`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04wWa!\xADW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa!\x98W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a!dW[P`\"T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01a\x04+V[\x90P` \x81=` \x11a!\x90W[\x81a!\x7F` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQa\x04+a!\x13V[=\x91Pa!rV[\x81a!\xA2\x91a_\xBDV[a\x02\nW\x80_a \xC5V[\x81a!\xB7\x91a_\xBDV[a\x02\nW\x80_a YV[PPP\xFD[\x92PP` \x82=` \x11a!\xF4W[\x81a!\xE3` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x82\x91Q_a\x1F\xF0V[=\x91Pa!\xD6V[\x81a\"\x06\x91a_\xBDV[a\x02\nW\x80_a\x1F\xA3V[\x92PP` \x82=` \x11a\"AW[\x81a\"-` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x82\x91\x90` a\x1E\xD1V[=\x91Pa\" V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa&\nW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa%\xF5W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a#\x84`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa%\xE0W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF` `@Qh\x10CV\x1A\x88)0\0\0\x81R\xA3sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa%\xCBW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rh\x10CV\x1A\x88)0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa%\xAEW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x80\x15a\x04\xBAW\x83\x90a%zW[a%\x07\x91Paf\xD3V[` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x94\x81\x93cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x04wW\x82\x90a%FW[a\x04E\x91PagRV[P` \x81=` \x11a%rW[\x81a%`` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x90Qa%<V[=\x91Pa%SV[P` \x81=` \x11a%\xA6W[\x81a%\x94` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa%\x07\x90Qa$\xFDV[=\x91Pa%\x87V[a%\xC6\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a$\xB8V[\x81a%\xD5\x91a_\xBDV[a\x02\nW\x80_a$LV[\x81a%\xEA\x91a_\xBDV[a\x02\nW\x80_a#\xA9V[\x81a%\xFF\x91a_\xBDV[a\x02\nW\x80_a#\x19V[\x81a&\x14\x91a_\xBDV[a\x02\nW\x80_a\"\xBDV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1ATa&<\x81a_\xE0V[\x91a&J`@Q\x93\x84a_\xBDV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a&\x8CW`@Q\x80a\x02\xC2\x87\x82a^`V[`\x01` \x81\x92a&\x9B\x85a_\xF8V[\x81R\x01\x92\x01\x92\x01\x91\x90a&wV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x06\xFD\xDE\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a)\xADW[Pa'n`@\x91\x82Q\x90a'?\x84\x83a_\xBDV[`\x11\x82R\x7FTestnet Syndicate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RaisV[\x80Q\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83\x81`\x04\x81\x86Z\xFA\x90\x81\x15a)\xA3W\x90a'\xEB\x91\x85\x91a)\x81W[P\x82Q\x90a'\xBC\x84\x83a_\xBDV[`\x0B\x82R\x7FTestnetSYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RaisV[\x82\x81Q\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA\x90\x81\x15a)=W\x82\x91a)GW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W`\xFF\x83Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01R`\x12`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a)=Wa)(W[PP` `\x04\x92\x82Q\x93\x84\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a)\x1FWP\x82\x90a(\xEBW[a\x04E\x91Pae\xD4V[P` \x81=` \x11a)\x17W[\x81a)\x05` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x90Qa(\xE1V[=\x91Pa(\xF8V[Q=\x84\x82>=\x90\xFD[\x81a)2\x91a_\xBDV[a\x07mW\x82_a(\xA0V[\x83Q=\x84\x82>=\x90\xFD[\x90P` \x81=` \x11a)yW[\x81a)b` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x07\x86Wa)s\x90ad\xACV[_a((V[=\x91Pa)UV[a)\x9D\x91P=\x80\x87\x83>a)\x95\x81\x83a_\xBDV[\x81\x01\x90ad9V[_a'\xAEV[\x82Q=\x86\x82>=\x90\xFD[a)\xC1\x91P=\x80\x85\x83>a)\x95\x81\x83a_\xBDV[_a'+V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa+3W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa+\x1EW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x16\xABW\x81\x80\x91`D`@Q\x80\x94\x81\x93c@\xC1\x0F\x19`\xE0\x1B\x83R\x81`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x10\xEFWP\xF3[\x81a+(\x91a_\xBDV[a\x02\nW\x80_a*\xCDV[\x81a+=\x91a_\xBDV[a\x02\nW\x80_a*;V[P4a\x02\nW`@`\x03\x196\x01\x12a\x02\nWa+e`\x045ah\xC6V[a+s\x81`\x01`$5aj\xB3V[`@\x83\x80\x82Qa+\x83\x84\x82a_\xBDV[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x83Qa,\x02\x81a+\xEE` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x88`$\x84\x01R`d\x83\x01\x90a]\xE6V[\x88`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a_\xBDV[Q\x90jconsole.logZ\xFAP\x83`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x82Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a)=Wa.\xD6W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90\x80;\x15a\x07mW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a)=Wa.\xC1W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x82Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a)=Wa.\xACW[PP`\x1FT`#T\x82Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91` \x91\x83\x91`\x08\x1C\x16\x81\x88\x81`D\x81\x01\x03\x92Z\xF1\x80\x15a.\xA2Wa.\x85W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x92`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90\x82Q\x91cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01R` \x82`$\x81\x88Z\xFA\x91\x82\x15a.{W\x90\x84\x92\x91\x87\x92a.BW[Pa-\xFA\x92a-\xF4\x91aa\x88V[\x90ahPV[` `\x01`\x01`\xA0\x1B\x03`#T\x16`$\x83Q\x80\x96\x81\x93cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a.9WP\x83\x90a\x06\xE6Wa\x04E\x92PahPV[Q=\x85\x82>=\x90\xFD[\x92P\x90P` \x82=` \x11a.sW[\x81a._` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x83\x91a-\xFAa-\xE6V[=\x91Pa.RV[\x83Q=\x88\x82>=\x90\xFD[a.\x9D\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a-\x9BV[\x82Q=\x87\x82>=\x90\xFD[\x81a.\xB6\x91a_\xBDV[a\x0C\x0CW\x83_a-7V[\x81a.\xCB\x91a_\xBDV[a\x0C\x0CW\x83_a,\xD5V[\x81a.\xE0\x91a_\xBDV[a\x0C\x0CW\x83_a,uV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1BTa/\x08\x81a_\xE0V[a/\x15`@Q\x91\x82a_\xBDV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a/\xEDW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a/\x82WPPPP\x03\x90\xF3[\x91\x93` a/\xDD\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a/\xCD\x83Q`@\x84R`@\x84\x01\x90a]\xE6V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra^\x0BV[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a/sV[`\x02` `\x01\x92`@Qa0\0\x81a_tV[a0\t\x86a_\xF8V[\x81Ra0\x16\x85\x87\x01aa\x95V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a/EV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a0\xA2`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa2\xACW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` `@Qh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0\x81R\xA3sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa2\x97W[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa2zW[P`\x1FT`\"T`#T`@Q\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R\x91` \x91\x83\x91`D\x91\x83\x91`\x08\x1C\x16Z\xFA\x80\x15a\x04wW\x82\x90a2FW[a\x04E\x91Pag\xD1V[P` \x81=` \x11a2rW[\x81a2`` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x90Qa2<V[=\x91Pa2SV[a2\x92\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a1\xD6V[\x81a2\xA1\x91a_\xBDV[a\x02\nW\x80_a1jV[\x81a2\xB6\x91a_\xBDV[a\x02\nW\x80_a0\xC7V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa56W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa5!W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa5\x0CW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\\\x19\xA9\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x04wWa4\xF7W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`$`@Q\x80\x94\x81\x93\x7F\xBBMD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x04wW\x82\x90a4\xC3W[a\x04E\x91PafTV[P` \x81=` \x11a4\xEFW[\x81a4\xDD` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x90Qa4\xB9V[=\x91Pa4\xD0V[\x81a5\x01\x91a_\xBDV[a\x02\nW\x80_a4ZV[\x81a5\x16\x91a_\xBDV[a\x02\nW\x80_a3\xF4V[\x81a5+\x91a_\xBDV[a\x02\nW\x80_a3\x91V[\x81a5@\x91a_\xBDV[a\x02\nW\x80_a35V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa:kW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa:VW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa:AW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa:$W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa:\x0FW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`d`\x01`\x01`\xA0\x1B\x03`#T\x16\x91\x85`@Q\x95\x86\x94\x85\x93\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01R`$\x84\x01Rh\x10CV\x1A\x88)0\0\0`D\x84\x01RZ\xF1\x80\x15a\x04wWa9\xF2W[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Qcp\xA0\x821`\xE0\x1B\x81R\x82`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x07MW\x84\x91a9\xBAW[Pa8 \x90af\xD3V[`\x01`\x01`\xA0\x1B\x03`#T\x16\x91`@Qcp\xA0\x821`\xE0\x1B\x81R\x83`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x0B\xECW\x85\x91a9}W[P\x91a8\xB6\x93\x91a8i` \x94agRV[`@Q\x80\x95\x81\x94\x82\x93\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91\x81`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x04wW\x82\x91a9HW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh\n\xD7\x8E\xBCZ\xC6 \0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x10\xEFWP\xF3[\x91PP` \x81=` \x11a9uW[\x81a9d` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x81\x90Q_a8\xC8V[=\x91Pa9WV[\x92\x94PP\x91` \x82=` \x11a9\xB2W[\x81a9\x9B` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x84\x93\x91\x92\x90a8\xB6a8WV[=\x91Pa9\x8EV[\x93PP` \x83=` \x11a9\xEAW[\x81a9\xD6` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa8 \x84\x93Q\x90a8\x16V[=\x91Pa9\xC9V[a:\n\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a7\xCEV[\x81a:\x19\x91a_\xBDV[a\x02\nW\x80_a7NV[a:<\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a6\xEAV[\x81a:K\x91a_\xBDV[a\x02\nW\x80_a6~V[\x81a:`\x91a_\xBDV[a\x02\nW\x80_a6\x1BV[\x81a:u\x91a_\xBDV[a\x02\nW\x80_a5\xBFV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a:\xFA`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa=\x15W[PP`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF` `@Qh65\xC9\xAD\xC5\xDE\xA0\0\0\x81R\xA3\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa=\0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa<\xEBW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a<\xB8W[P`\x04\x91a<v` \x92afTV[`@Q\x92\x83\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x04wW\x82\x90a4\xC3Wa\x04E\x91PafTV[\x90P` \x81=` \x11a<\xE3W[\x81a<\xD3` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ`\x04a<gV[=\x91Pa<\xC6V[\x81a<\xF5\x91a_\xBDV[a\x02\nW\x80_a< V[\x81a=\n\x91a_\xBDV[a\x02\nW\x80_a;\xC4V[\x81a=\x1F\x91a_\xBDV[a\x02\nW\x80_a;\x1FV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R`\x01`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaB!W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04wW\x82\x91aA\xECW[P`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x80\x91`@Q\x93\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x86\x80\xA4sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16&W\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaA\xD7W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91aA\xA2W[P`\x01`\x01`\xA0\x1B\x03`!T\x16\x82;\x15a!\xC2W`@Q\x7F\xD5Gt\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04wWaA\x8DW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaAxW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x04\xBAW\x83\x92aA@W[P`!T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R` \x90\x82\x90`D\x90\x82\x90Z\xFA\x90\x81\x15a\x04wW\x82\x91aA!W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x10\xEFWP\xF3[aA:\x91P` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[_a@\xAEV[\x92P\x90P` \x82=` \x11aApW[\x81aA]` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x82\x91` a@TV[=\x91PaAPV[\x81aA\x82\x91a_\xBDV[a\x02\nW\x80_a@\x06V[\x81aA\x97\x91a_\xBDV[a\x02\nW\x80_a?\x9AV[\x92PP` \x82=` \x11aA\xCFW[\x81aA\xBE` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x82\x91Q_a?1V[=\x91PaA\xB1V[\x81aA\xE1\x91a_\xBDV[a\x02\nW\x80_a>\xE4V[\x91PP` \x81=` \x11aB\x19W[\x81aB\x08` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x81\x90Q_a>6V[=\x91PaA\xFBV[\x81aB+\x91a_\xBDV[a\x02\nW\x80_a=\xE8V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaI\x0EW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWaH\xF9W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rhlk\x93[\x8B\xBD@\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWaH\xE4W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16aC\x93a`\xFBV[`@Q` \x81\x01\x90aC\xBF` \x82\x85Q\x80\x83\x88\x01\x87^\x81\x01\x88\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a_\xBDV[Q\x90 `@Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x07MW\x84\x91aH\xC5W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a!\xC2W\x83`\x01`\x01`\xA0\x1B\x03aD\x86\x92`@Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x95\x86`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07MW\x84\x91aH\xB0W[PP\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh\xA2\xA1]\tQ\x9B\xE0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWaH\x9BW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaH\x86W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91aHTW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x07MW\x84\x91aH\"W[P`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q\x91cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01R` \x82`$\x81\x87Z\xFA\x90\x81\x15a\x0B\xECW\x85\x91aG\xECW[aF#\x92Paa6V[\x91aF,a`\xFBV[\x84`@Q` \x81\x01\x90aFY` \x82\x86Q\x80\x83\x89\x01\x87^\x81\x01\x86\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a_\xBDV[Q\x90 `@Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x04wW\x82\x91aG\xCDW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aG \x92`@Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaG\xB4W[PP` \x90`$`@Q\x80\x94\x81\x93cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07MW\x84\x91aG\x80W[Pa-\xF4\x90a\x04E\x93aa6V[\x90P` \x81=` \x11aG\xACW[\x81aG\x9B` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQa\x04EaGrV[=\x91PaG\x8EV[\x81aG\xBE\x91a_\xBDV[aG\xC9W\x84_aGEV[\x84\x80\xFD[aG\xE6\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aF\xB3V[\x90P` \x82=` \x11aH\x1AW[\x81aH\x07` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWaF#\x91Q\x90aF\x19V[=\x91PaG\xFAV[\x90P` \x81=` \x11aHLW[\x81aH=` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ_aE\xE1V[=\x91PaH0V[\x90P` \x81=` \x11aH~W[\x81aHo` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ_aE\xAAV[=\x91PaHbV[\x81aH\x90\x91a_\xBDV[a\x02\nW\x80_aE\\V[\x81aH\xA5\x91a_\xBDV[a\x02\nW\x80_aD\xF0V[\x81aH\xBA\x91a_\xBDV[a\x16&W\x82_aD\xAEV[aH\xDE\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aD\x19V[\x81aH\xEE\x91a_\xBDV[a\x02\nW\x80_aC{V[\x81aI\x03\x91a_\xBDV[a\x02\nW\x80_aC\x1FV[\x81aI\x18\x91a_\xBDV[a\x02\nW\x80_aB\xC3V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10aI\x82Wa\x02\xC2\x85a\x02\xB6\x81\x87\x03\x82a_\xBDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aIkV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10aJ\0Wa\x02\xC2\x85a\x02\xB6\x81\x87\x03\x82a_\xBDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aI\xE9V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1ETaJ<\x81a_\xE0V[aJI`@Q\x91\x82a_\xBDV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10aK\x8AW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10aJ\xB5W\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10aKAWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93aJ\xA8V[\x90\x91\x92\x93\x94` \x80aK}\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa]\xE6V[\x97\x01\x95\x01\x93\x92\x91\x01aK\x1DV[`@QaK\x96\x81a_tV[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80TaK\xB2\x81a_\xE0V[\x91aK\xC0`@Q\x93\x84a_\xBDV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10aK\xF6WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90aJyV[`\x01` \x81\x92aL\x05\x86a_\xF8V[\x81R\x01\x93\x01\x91\x01\x90\x91aK\xD0V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@\x81\x81Q\x91aL4\x81\x84a_\xBDV[`\x0C\x83R` \x83\x01\x7FwrongAddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81Q`\x0C` \x82\x01\x92\x83^\x83`,\x82\x01R`\x0C\x81RaL\x7F`,\x82a_\xBDV[Q\x90 \x81Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aP<W\x83\x91aQ\x1BW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07mW\x81Q\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83\x81\x80aM@`\x01`\x01`\xA0\x1B\x03\x86\x16\x98\x89`\x04\x84\x01R\x87`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aP]W\x90\x84\x91aQ\x06W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x82Q\x91\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` \x83`\x04\x81\x84Z\xFA\x92\x83\x15aP\xFCW\x85\x93aP\xC0W[P\x94` \x84\x95\x96`\x04\x95Q\x95\x86\x80\x92\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x93\x84\x15aP\x7FW\x86\x94aP\x89W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x11\x19W\x84Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x85\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aP\x7FW\x90\x86\x91aPjW[PP\x83Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91RaN\xBA\x81`d\x81\x01a\x08\xF6V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a!\xC2W\x83aO\x14\x91\x84Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aP]W\x90\x84\x91aPHW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`#T\x16\x82;\x15aG\xC9W\x83Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x83\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15aP<W\x90\x83\x91aP'W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a)\x1FWPa\x10\xEFWP\xF3[\x81aP1\x91a_\xBDV[a\x16\xABW\x81_aO\xB8V[PPQ\x90=\x90\x82>=\x90\xFD[\x81aPR\x91a_\xBDV[a\x16&W\x82_aO<V[PPPQ\x90=\x90\x82>=\x90\xFD[\x81aPt\x91a_\xBDV[aG\xC9W\x84_aNkV[\x85Q=\x88\x82>=\x90\xFD[\x95P\x92P` \x85=` \x11aP\xB8W[\x81aP\xA6` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x85\x94Q\x92_aM\xF9V[=\x91PaP\x99V[\x93\x94P\x91P` \x83=` \x11aP\xF4W[\x81aP\xDE` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x91Q\x85\x93\x92\x90\x91` aM\xB7V[=\x91PaP\xD1V[\x84Q=\x87\x82>=\x90\xFD[\x81aQ\x10\x91a_\xBDV[a\x07mW\x82_aMhV[aQ4\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aL\xD8V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10aQ\x99Wa\x02\xC2\x85a\x02\xB6\x81\x87\x03\x82a_\xBDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aQ\x82V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaS(W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaS\x13W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x10\xEFWP\xF3[\x81aS\x1D\x91a_\xBDV[a\x02\nW\x80_aR\xBEV[\x81aS2\x91a_\xBDV[a\x02\nW\x80_aR,V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaTdW[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90a0\x13\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17aT7W\x91\x83\x91`@\x93alw\x849\x85\x82R` \x82\x01R\x03\x01\x90\x82\xF0\x15aT+W\x80\xF3[`@Q\x90=\x90\x82>=\x90\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81aTn\x91a_\xBDV[a\x02\nW\x80_aS\xDEV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@\x80QaT\x98\x82\x82a_\xBDV[`\x0C\x81R\x82` \x82\x01\x7FdefaultAdmin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83Q`\x0C` \x82\x01\x92\x83^\x82`,\x82\x01R`\x0C\x81RaT\xE4`,\x82a_\xBDV[Q\x90 \x83Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a[9W\x82\x91a[\xDFW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aU\xA8\x92\x86Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R\x88`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a[9Wa[\xCAW[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U\x80QaV\x04\x82\x82a_\xBDV[`\x06\x81R\x82` \x82\x01\x7Fminter\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83Q`\x06` \x82\x01\x92\x83^\x82`&\x82\x01R`\x06\x81RaVP`&\x82a_\xBDV[Q\x90 \x83Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a[9W\x82\x91a[\xABW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aW\x14\x92\x86Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R\x88`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a[9Wa[\x96W[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U\x80QaWp\x82\x82a_\xBDV[`\x04\x81R\x82` \x82\x01\x7Fuser\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83Q`\x04` \x82\x01\x92\x83^\x82`$\x82\x01R`\x04\x81RaW\xBC`$\x82a_\xBDV[Q\x90 \x83Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a[9W\x82\x91a[wW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aX\x80\x92\x86Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R\x88`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a[9Wa[bW[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"U\x80QaX\xDC\x82\x82a_\xBDV[`\x07\x81R\x82` \x82\x01\x7Fspender\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83Q`\x07` \x82\x01\x92\x83^\x82`'\x82\x01R`\x07\x81RaY(`'\x82a_\xBDV[Q\x90 \x83Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a[9W\x82\x91a[CW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aY\xEC\x92\x86Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R\x88`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a[9Wa[$W[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`#T\x16\x17`#U`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x82Q\x91a0\x13\x80\x84\x01\x90\x84\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17aZ\xF7W\x91\x84\x93\x91aZ\x98\x93alw\x869`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[\x03\x90\x83\xF0\x90\x81\x15aZ\xECWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80\xF3[Q\x91=\x91P\x82>=\x90\xFD[`$\x87\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a[.\x91a_\xBDV[a\x07mW\x82_aZ\x11V[\x84Q=\x84\x82>=\x90\xFD[a[\\\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aY\x81V[\x81a[l\x91a_\xBDV[a\x07mW\x82_aX\xA5V[a[\x90\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aX\x15V[\x81a[\xA0\x91a_\xBDV[a\x07mW\x82_aW9V[a[\xC4\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aV\xA9V[\x81a[\xD4\x91a_\xBDV[a\x07mW\x82_aU\xCDV[a[\xF8\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aU=V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`$`@Q\x80\x94\x81\x93\x7F\xBBMD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x04wW\x82\x90a(\xEBWa\x04E\x91Pae\xD4V[\x90P4a\x04\xAEW_`\x03\x196\x01\x12a\x04\xAEWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a]\x99Wa]\x86W[P`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x90a0\x13\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17aT7W\x91\x83\x91`@\x93alw\x849\x81R\x84` \x82\x01R\x03\x01\x90\x82\xF0\x15aT+W\x80\xF3[a]\x92\x91P_\x90a_\xBDV[__a]:V[`@Q=_\x82>=\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a]\xC7WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a]\xBAV[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a^(WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a^\x1BV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a^\x92WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a^\xCE\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa]\xE6V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a^\x83V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a_\x0FWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a_e\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a^\x0BV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a_\0V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a_\x90W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a_\x90W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a_\x90W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a`\xF1W[` \x85\x10\x84\x14a`\xC4W\x84\x87R\x86\x93\x90\x81\x15a`\x84WP`\x01\x14a`@W[Pa`>\x92P\x03\x83a_\xBDV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a`hWPP\x90` a`>\x92\x82\x01\x01_a`1V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a`OV[` \x93Pa`>\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a`1V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a`\x12V[`@Q\x90aa\n`@\x83a_\xBDV[`\x05\x82R\x7Fuser3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[\x91\x90\x82\x01\x80\x92\x11aaCWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90\x81` \x91\x03\x12a\x04\xAEWQ\x80\x15\x15\x81\x03a\x04\xAEW\x90V[\x91\x90\x82\x03\x91\x82\x11aaCWV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10ac\xACWa`>\x94T\x91\x81\x81\x10acvW[\x81\x81\x10ac@W[\x81\x81\x10ac\nW[\x81\x81\x10ab\xD4W[\x81\x81\x10ab\x9EW[\x81\x81\x10abhW[\x81\x81\x10ab3W[\x10ab\x06W[P\x03\x83a_\xBDV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aa\xFEV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aa\xF8V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aa\xF0V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aa\xE8V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aa\xE0V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aa\xD8V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aa\xD0V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aa\xC8V[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aa\xB0V[` \x81\x83\x03\x12a\x04\xAEW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\xAEW\x01\x81`\x1F\x82\x01\x12\x15a\x04\xAEW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a_\x90W`@Q\x92ad\x8B`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a_\xBDV[\x82\x84R` \x83\x83\x01\x01\x11a\x04\xAEW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x04\xAEWV[\x90\x81` \x91\x03\x12a\x04\xAEWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x04\xAEW\x90V[\x90\x81``\x91\x03\x12a\x04\xAEWad\xED\x81ad\xACV[\x91`@` \x83\x01Q\x92\x01Q\x90V[`\x08T`\xFF\x16\x80\x15ae\nW\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a]\x99W_\x91ae\xA2W[P\x15\x15\x90V[\x90P` \x81=` \x11ae\xCCW[\x81ae\xBD` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ_ae\x9CV[=\x91Pae\xB0V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[_a`>\x91a_\xBDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh%\xF2s\x93=\xB5p\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh\x10CV\x1A\x88)0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01ah\xE2\x92aj\xB3V[\x90_\x80`@Qah\xF3`@\x82a_\xBDV[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Qai`\x81a+\xEE` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`@`$\x84\x01R`d\x83\x01\x90a]\xE6V[Q\x90jconsole.logZ\xFAPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEWai\xD2_\x91ai\xE4`@Q\x94\x85\x93\x84\x93\x7F\xF3 \xD9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90a]\xE6V[\x90`\x03\x19\x84\x83\x03\x01`$\x85\x01Ra]\xE6V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[\x81\x15aj\x86W\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x90\x83\x83\x11ak\xF2W\x82\x81\x10\x91\x82\x15\x80ak\xE8W[ak\xE0Waj\xD6\x84\x86aa\x88V[\x92`\x01\x84\x01\x80\x94\x11aaCW`\x03\x83\x11\x15\x80ak\xD7W[ak\xC8W`\x03\x19\x83\x10\x15\x80ak\xBEW[ak\xAAW\x85\x83\x11\x15akaWPP\x90ak\x19\x84ak\x1E\x93aa\x88V[aj|V[\x90\x81\x15ak\\Wak/\x92Paa6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11aaCW\x90V[PP\x90V[\x95\x94\x92\x91\x90\x95akrW[PPPPV[\x83\x94\x95Pak\x19\x90ak\x84\x93\x94aa\x88V[\x90\x81\x15ak\\Wak\x95\x92Paa\x88V[`\x01\x81\x01\x80\x91\x11aaCW\x90_\x80\x80\x80aklV[PP\x90Pak\xBB\x92\x91P\x19\x90aa\x88V[\x90V[P\x82\x19\x84\x11aj\xFDV[PP\x91\x90Pak\xBB\x92Paa6V[P\x82\x84\x11aj\xEDV[P\x92PPP\x90V[P\x84\x82\x11\x15aj\xC8V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R\xFD\xFEa\x01`\x80`@R4a\x04\xB8W`@\x81a0\x13\x808\x03\x80\x91a\0 \x82\x85a\x04\xBCV[\x839\x81\x01\x03\x12a\x04\xB8Wa\0?` a\08\x83a\x04\xDFV[\x92\x01a\x04\xDFV[`@Qa\0M`@\x82a\x04\xBCV[`\x11\x81R` \x81\x01pTestnet Syndicate`x\x1B\x81R`@Q\x90a\0{`@\x83a\x04\xBCV[`\x11\x82RpTestnet Syndicate`x\x1B` \x83\x01R`@Q\x92a\0\xA8`@\x85a\x04\xBCV[`\x0B\x84Rj\x15\x19\\\xDD\x1B\x99]\x14\xD6S\x91`\xAA\x1B` \x85\x01R`@Q\x93a\0\xCF`@\x86a\x04\xBCV[`\x01\x85R`1`\xF8\x1B` \x86\x01\x90\x81R\x84Q\x90\x94`\x01`\x01`@\x1B\x03\x82\x11a\x03\xBBW`\x03T\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x04\xAEW[` \x83\x10\x14a\x03\x9DW\x81`\x1F\x84\x93\x11a\x04@W[P` \x90`\x1F\x83\x11`\x01\x14a\x03\xDAW_\x92a\x03\xCFW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x03U[\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xBBW`\x04T\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x03\xB1W[` \x83\x10\x14a\x03\x9DW\x81`\x1F\x84\x93\x11a\x03/W[P` \x90`\x1F\x83\x11`\x01\x14a\x02\xC9W_\x92a\x02\xBEW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x04U[a\x01\xAD\x81a\x05\xFCV[a\x01 Ra\x01\xBA\x84a\x07\x83V[a\x01@RQ\x90 \x91\x82`\xE0RQ\x90 \x80a\x01\0RF`\xA0R`@Q\x90` \x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x02#`\xC0\x82a\x04\xBCV[Q\x90 `\x80R0`\xC0R`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x02\xAFW`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x02\xAFWa\x02Wa\x02]\x92a\x04\xF3V[Pa\x05iV[P`@Qa&\xF7\x90\x81a\x08\xBC\x829`\x80Q\x81a\x17\xB9\x01R`\xA0Q\x81a\x18v\x01R`\xC0Q\x81a\x17\x8A\x01R`\xE0Q\x81a\x18\x08\x01Ra\x01\0Q\x81a\x18.\x01Ra\x01 Q\x81a\n\xDC\x01Ra\x01@Q\x81a\x0B\x05\x01R\xF3[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[\x01Q\x90P_\x80a\x01\x8FV[`\x04_\x90\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x03\x17WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x02\xFFW[PPP\x81\x1B\x01`\x04Ua\x01\xA4V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02\xF1V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x02\xDBV[`\x04_R\x90\x91P\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x03\x93W[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x03\x85WPa\x01yV[_\x81U\x84\x93P`\x01\x01a\x03xV[\x90\x91P\x81\x90a\x03jV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x01eV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P_\x80a\x01-V[`\x03_\x90\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x04(WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x04\x10W[PPP\x81\x1B\x01`\x03Ua\x01BV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x04\x02V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x03\xECV[`\x03_R\x90\x91P\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x04\xA4W[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x04\x96WPa\x01\x17V[_\x81U\x84\x93P`\x01\x01a\x04\x89V[\x90\x91P\x81\x90a\x04{V[\x91`\x7F\x16\x91a\x01\x03V[_\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x03\xBBW`@RV[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x04\xB8WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a/\xF3_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05dW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a/\xF3_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a/\xB3_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a/\xD3_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05dW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a/\xD3_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6\x90_Q` a/\xB3_9_Q\x90_R\x90\x80\xA4`\x01\x90V[\x90\x81Q` \x81\x10_\x14a\x06vWP\x90`\x1F\x81Q\x11a\x066W` \x81Q\x91\x01Q` \x82\x10a\x06'W\x17\x90V[_\x19\x82` \x03`\x03\x1B\x1B\x16\x17\x90V[`D` \x91`@Q\x92\x83\x91c0Z'\xA9`\xE0\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBBW`\x06T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x07yW[` \x82\x10\x14a\x03\x9DW`\x1F\x81\x11a\x07FW[P` \x92`\x1F\x82\x11`\x01\x14a\x06\xE5W\x92\x81\x92\x93_\x92a\x06\xDAW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x06U`\xFF\x90V[\x01Q\x90P_\x80a\x06\xC1V[`\x1F\x19\x82\x16\x93`\x06_R\x80_ \x91_[\x86\x81\x10a\x07.WP\x83`\x01\x95\x96\x10a\x07\x16W[PPP\x81\x1B\x01`\x06U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x07\x08V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x06\xF5V[`\x06_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90`\x1F\x83\x01`\x05\x1C\x01[\x81\x81\x10a\x07nWPa\x06\xA7V[_\x81U`\x01\x01a\x07aV[\x90`\x7F\x16\x90a\x06\x95V[\x90\x81Q` \x81\x10_\x14a\x07\xAEWP\x90`\x1F\x81Q\x11a\x066W` \x81Q\x91\x01Q` \x82\x10a\x06'W\x17\x90V[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBBW`\x07T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x08\xB1W[` \x82\x10\x14a\x03\x9DW`\x1F\x81\x11a\x08~W[P` \x92`\x1F\x82\x11`\x01\x14a\x08\x1DW\x92\x81\x92\x93_\x92a\x08\x12W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x07U`\xFF\x90V[\x01Q\x90P_\x80a\x07\xF9V[`\x1F\x19\x82\x16\x93`\x07_R\x80_ \x91_[\x86\x81\x10a\x08fWP\x83`\x01\x95\x96\x10a\x08NW[PPP\x81\x1B\x01`\x07U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x08@V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x08-V[`\x07_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90`\x1F\x83\x01`\x05\x1C\x01[\x81\x81\x10a\x08\xA6WPa\x07\xDFV[_\x81U`\x01\x01a\x08\x99V[\x90`\x7F\x16\x90a\x07\xCDV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x025W\x80c\x06\xFD\xDE\x03\x14a\x020W\x80c\t^\xA7\xB3\x14a\x02+W\x80c\x18\x16\r\xDD\x14a\x01\xB8W\x80c#\xB8r\xDD\x14a\x02&W\x80c$\x8A\x9C\xA3\x14a\x02!W\x80c//\xF1]\x14a\x02\x1CW\x80c1<\xE5g\x14a\x02\x17W\x80c6D\xE5\x15\x14a\x02\x12W\x80c6V\x8A\xBE\x14a\x02\rW\x80c:F\xB1\xA8\x14a\x01\xC2W\x80c@\xC1\x0F\x19\x14a\x02\x08W\x80cK\xF5\xD7\xE9\x14a\x02\x03W\x80cX|\xDE\x1E\x14a\x01\xFEW\x80c\\\x19\xA9\\\x14a\x01\xF9W\x80co\xCF\xFFE\x14a\x01\xF4W\x80cp\xA0\x821\x14a\x01\xEFW\x80c~\xCE\xBE\0\x14a\x01\xEAW\x80c\x84\xB0\x19n\x14a\x01\xE5W\x80c\x8ES\x9E\x8C\x14a\x01\xE0W\x80c\x91\xD1HT\x14a\x01\xDBW\x80c\x91\xDD\xAD\xF4\x14a\x01\xD6W\x80c\x95\xD8\x9BA\x14a\x01\xD1W\x80c\x9A\xB2N\xB0\x14a\x01\xBDW\x80c\xA2\x17\xFD\xDF\x14a\x01\xCCW\x80c\xA9\x05\x9C\xBB\x14a\x01\xC7W\x80c\xB0\xCA%>\x14a\x01\xC2W\x80c\xBBMD6\x14a\x01\xBDW\x80c\xC0*\xE7T\x14a\x01\xB8W\x80c\xC3\xCD\xA5 \x14a\x01\xB3W\x80c\xD5\x05\xAC\xCF\x14a\x01\xAEW\x80c\xD59\x13\x93\x14a\x01\xA9W\x80c\xD5Gt\x1F\x14a\x01\xA4W\x80c\xDDb\xED>\x14a\x01\x9FWc\xF1\x12~\xD8\x14a\x01\x9AW_\x80\xFD[a\x11\xECV[a\x11\x93V[a\x11UV[a\x11\x1BV[a\x0F\xC1V[a\x0EzV[a\x04\x86V[a\r\xF7V[a\x06rV[a\x0E4V[a\x0E\x1AV[a\rRV[a\r'V[a\x0C\xD7V[a\x0B\xFBV[a\n\xC4V[a\n\x8CV[a\nWV[a\t\xDCV[a\t\xBAV[a\tyV[a\x08\xD0V[a\x07\x84V[a\x06\x15V[a\x05\xFBV[a\x05\xE0V[a\x05\x9BV[a\x05hV[a\x04\xA3V[a\x04UV[a\x031V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W`\x045\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x80\x91\x03a\x02\xD6W\x80\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x92\x14\x90\x81\x15a\x02\xACW[P`@Q\x90\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14_a\x02\xA1V[_\x80\xFD[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` a\x03.\x92\x81\x81R\x01\x90a\x02\xDAV[\x90V[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W`@Q_`\x03Ta\x03Q\x81a\x12\xB5V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\xE7WP`\x01\x14a\x03\x89W[a\x03\x85\x83a\x03y\x81\x85\x03\x82a\x13\xF4V[`@Q\x91\x82\x91\x82a\x03\x1DV[\x03\x90\xF3[`\x03_\x90\x81R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x93\x92P\x90[\x80\x82\x10a\x03\xCDWP\x90\x91P\x81\x01` \x01a\x03ya\x03iV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x03\xB5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x03y\x90Pa\x03iV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xD6WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xD6WV[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x04{a\x04qa\x04)V[`$5\x903a\x1B\x03V[` `@Q`\x01\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` `\x02T`@Q\x90\x81R\xF3[4a\x02\xD6W```\x03\x196\x01\x12a\x02\xD6Wa\x04\xBCa\x04)V[a\x04\xC4a\x04?V[`D5\x90`\x01`\x01`\xA0\x1B\x03\x83\x16_R`\x01` Ra\x04\xF73`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x92_\x19\x84\x10a\x05\x18W[a\x05\x0C\x93Pa\x14\x99V[`@Q`\x01\x81R` \x90\xF3[\x82\x84\x10a\x054Wa\x05/\x83a\x05\x0C\x95\x033\x83a\x1B\xD1V[a\x05\x02V[\x82\x84\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d_\xFD[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W` a\x05\x93`\x045_R`\x05` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x05\xDE`\x045a\x05\xBAa\x04?V[\x90a\x05\xD9a\x05\xD4\x82_R`\x05` R`\x01`@_ \x01T\x90V[a\x16gV[a\x16\xC8V[\0[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` `@Q`\x12\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` a\x05\x93a\x17\x80V[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6W`\x045a\x061a\x04?V[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x06JWa\x05\xDE\x91a\x18\x9CV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x06\x8Ba\x04)V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\n` Ra\x06\xAC`@_ \x91a\x19LV[\x81T\x90_\x82\x91`\x05\x84\x11a\x07,W[a\x06\xC6\x93P\x84a\x1E\x0CV[\x80a\x06\xF5WPP` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_[\x16`@Q\x90\x81R\xF3[` \x91a\x07\x1Cy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a\x19\xCBV[\x90_R\x82_ \x01T`0\x1Ca\x06\xECV[\x91\x92a\x077\x81a\x1C\x97V[\x81\x03\x90\x81\x11a\x07\x7FWa\x06\xC6\x93\x85_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x07mWP\x91a\x06\xBBV[\x92\x91Pa\x07y\x90a\x19\xD9V[\x90a\x06\xBBV[a\x19\x9EV[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x07\x9Da\x04)V[`$5a\x07\xA8a\x15\xDFV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a\x08\xA8W\x81\x15a\x08\x80Wa\x07\xD2a\x07\xCD\x83`\x02Ta\x19\xE7V[`\x02UV[a\x07\xEC\x83`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x83\x01\x90U`@Q\x82\x81R_\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3`\x02T\x91y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11a\x08PWa\x05\xDE\x83\x83a$6V[\x7F\x1C\xB1]&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x84\x90R`$R`D_\xFD[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6Wa\x08\xE9Ca\x1C\x18V[e\xFF\xFF\xFF\xFF\xFF\xFF\x80a\x08\xFACa\x1C\x18V[\x16\x91\x16\x03a\tQWa\x03\x85`@Qa\t\x13`@\x82a\x13\xF4V[`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02\xDAV[\x7Fo\xF0q@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W`\x01`\x01`\xA0\x1B\x03a\t\x9Aa\x04)V[\x16_R`\t` R` `\x01`\x01`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6Wa\x05\xDEa\t\xD6a\x04)V[3a\x19\xF4V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W`\x01`\x01`\xA0\x1B\x03a\t\xFDa\x04)V[\x16_R`\n` R`@_ Tc\xFF\xFF\xFF\xFF\x81\x11a\n'W`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` `\x04R`$R`D_\xFD[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W` a\x05\x93a\nua\x04)V[`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W`\x01`\x01`\xA0\x1B\x03a\n\xADa\x04)V[\x16_R`\x08` R` `@_ T`@Q\x90\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6Wa\x0B\xA2a\x0B\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1F\xC3V[a\x0B)\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a <V[` `@Qa\x0B8\x82\x82a\x13\xF4V[_\x81R\x81a\x0B\xB0\x81\x83\x01\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x83\x016\x877`@Q\x97\x88\x97\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a\x02\xDAV[\x90\x87\x82\x03`@\x89\x01Ra\x02\xDAV[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\x0B\xE4WPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x0B\xD5V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6Wa\x0C\x17`\x045a\x19LV[`\x0BT\x90_\x82\x91`\x05\x84\x11a\x0C\x83W[a\x0C3\x93P`\x0Ba\x1E\x0CV[\x80a\x0CaWP` _[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[a\x0C~a\x0Co` \x92a\x19\xCBV[`\x0B_R\x82_ \x01T`0\x1C\x90V[a\x0C=V[\x91\x92a\x0C\x8E\x81a\x1C\x97V[\x81\x03\x90\x81\x11a\x07\x7FWa\x0C3\x93`\x0B_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x0C\xC5WP\x91a\x0C'V[\x92\x91Pa\x0C\xD1\x90a\x19\xD9V[\x90a\x0C'V[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6W` `\xFFa\r\x1B`\x045a\x0C\xFAa\x04?V[\x90_R`\x05\x84R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` a\rBCa\x1C\x18V[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W`@Q_`\x04Ta\rr\x81a\x12\xB5V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\xE7WP`\x01\x14a\r\x99Wa\x03\x85\x83a\x03y\x81\x85\x03\x82a\x13\xF4V[`\x04_\x90\x81R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93\x92P\x90[\x80\x82\x10a\r\xDDWP\x90\x91P\x81\x01` \x01a\x03ya\x03iV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\r\xC5V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W` a\x05\x93a\x0E\x15a\x04)V[a\x14FV[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` `@Q_\x81R\xF3[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x04{a\x0EPa\x04)V[`$5\x903a\x14\x99V[`d5\x90`\xFF\x82\x16\x82\x03a\x02\xD6WV[`\x845\x90`\xFF\x82\x16\x82\x03a\x02\xD6WV[4a\x02\xD6W`\xC0`\x03\x196\x01\x12a\x02\xD6Wa\x0E\x93a\x04)V[`$5\x90`D5a\x0E\xA2a\x0EZV[`\x845\x90`\xA45\x92\x80B\x11a\x0F\x96W\x91a\x0F(\x93\x91a\x0F\x1Aa\x0F\x1F\x94`@Q` \x81\x01\x91\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x83R`\x01`\x01`\xA0\x1B\x03\x8A\x16`@\x83\x01R\x8A``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x0F\x12`\xA0\x82a\x13\xF4V[Q\x90 a\x1A\xB3V[a sV[\x90\x92\x91\x92a!7V[a\x0FL\x81`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x80\x93\x03a\x0F]Wa\x05\xDE\x92Pa\x19\xF4V[`\x01`\x01`\xA0\x1B\x03\x91P\x7Fu-\x88\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$R`D_\xFD[\x7FF\x83\xAF\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x02\xD6W`\xE0`\x03\x196\x01\x12a\x02\xD6Wa\x0F\xDAa\x04)V[a\x0F\xE2a\x04?V[`D5\x90`d5\x92a\x0F\xF2a\x0EjV[`\xA45`\xC45\x90\x86B\x11a\x10\xEFWa\x10\x9B\x92a\x10\x96a\x10+\x86`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x98`@Q` \x81\x01\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R`\x01`\x01`\xA0\x1B\x03\x89\x16\x9B\x8C`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16``\x84\x01R\x8B`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x0F\x12`\xE0\x82a\x13\xF4V[a\x1A\xF4V[\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\x10\xB5Wa\x05\xDE\x93Pa\x1B\x03V[\x7FK\x80\x0EF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04R\x16`$R`D_\xFD[\x86\x7Fby\x13\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` `@Q\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6\x81R\xF3[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x05\xDE`\x045a\x11ta\x04?V[\x90a\x11\x8Ea\x05\xD4\x82_R`\x05` R`\x01`@_ \x01T\x90V[a\x18\x9CV[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6W` a\x11\xE3a\x11\xB1a\x04)V[`\x01`\x01`\xA0\x1B\x03a\x11\xC1a\x04?V[\x91\x16_R`\x01\x83R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x12\x05a\x04)V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\xD6Wa\x03\x85\x91`\x01`\x01`\xA0\x1B\x03a\x12R\x92a\x12.a\x14\x81V[Pa\x127a\x14\x81V[P\x16_R`\n` R`@_ a\x12La\x14\x81V[Pa!\xFEV[P`@Q\x90a\x12`\x82a\x13\xD3V[Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82R`0\x1C` \x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`@\x84\x01\x95e\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x12\xFCW[` \x83\x10\x14a\x12\xCFWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x12\xC4V[_\x92\x91\x81T\x91a\x13\x15\x83a\x12\xB5V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a\x13jWP`\x01\x14a\x131WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a\x13PWP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a\x13?V[\x90P` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x13\xEFW`@RV[a\x13\xA6V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x13\xEFW`@RV[`@Q\x90a\x14D`@\x83a\x13\xF4V[V[`\x01`\x01`\xA0\x1B\x03\x16_R`\n` Ry\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x14}`@_ a\x1A\x8AV[\x16\x90V[`@Q\x90a\x14\x8E\x82a\x13\xD3V[_` \x83\x82\x81R\x01RV[\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x93\x84\x15a\x15\xB3W`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a\x15\x87Wa\x14\xD7\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x84\x81\x10a\x15SW\x95\x84a\x14D\x96\x97\x03a\x15\x01\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[Ua\x15\x1C\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x86\x01\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a$\xB9V[\x84\x90\x87\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[3_\x90\x81R\x7F\x15\xA2\x8D&\xFA\x1B\xF76\xCF~\xDC\x99\"`qq\xCC\xB0\x9C<s\xB8\x08\xE7w*0\x13\xE0h\xA5\"` R`@\x90 T`\xFF\x16\x15a\x16\x17WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6`$R`D_\xFD[\x80_R`\x05` R`\xFFa\x16\x8F3`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a\x16\x99WPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R`\x05` R`\xFFa\x16\xF0\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16a\x17zW\x80_R`\x05` Ra\x17\x1C\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x14\x80a\x18sW[\x15a\x17\xDBW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x18m`\xC0\x82a\x13\xF4V[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x17\xB2V[\x80_R`\x05` R`\xFFa\x18\xC4\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a\x17zW\x80_R`\x05` Ra\x18\xF1\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[e\xFF\xFF\xFF\xFF\xFF\xFFa\x19\\Ca\x1C\x18V[\x16\x80\x82\x10\x15a\x19oWPa\x03.\x90a\x1C\x18V[\x90\x7F\xEC\xD3\xF8\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90_\x19\x82\x01\x91\x82\x11a\x07\x7FWV[\x90`\x01\x82\x01\x80\x92\x11a\x07\x7FWV[\x91\x90\x82\x01\x80\x92\x11a\x07\x7FWV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\t` R`@\x81 \x80T\x86\x85\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x81\x17\x90\x92Ua\x14D\x96\x94\x16\x94a\x1A\x84\x93\x90\x92\x86\x91\x90\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x90\x80\xA4`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[\x91a\x1EpV[\x80T\x80a\x1A\x97WPP_\x90V[\x80_\x19\x81\x01\x11a\x07\x7FW_\x19\x91_R` _ \x01\x01T`0\x1C\x90V[`B\x90a\x1A\xBEa\x17\x80V[\x90`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91a\x03.\x93\x91a\x0F\x1F\x93a sV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x1B\xA5W`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x83\x15a\x1ByW\x80a\x1Bl\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93\x85_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[U`@Q\x90\x81R` \x90\xA3V[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x1B\xA5W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1ByWa\x1C\x15\x91_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[UV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1C0We\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`0`\x04R`$R`D_\xFD[\x81\x15a\x1CjW\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[`\x01\x81\x11\x15a\x03.W\x80`\x01p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10\x15a\x1D\xCAW[a\x1Dpa\x1Dfa\x1D\\a\x1DRa\x1DHa\x1D>a\x1D-a\x1Dw\x97`\x04\x8Ah\x01\0\0\0\0\0\0\0\0a\x1D|\x9C\x10\x15a\x1D\xBDW[d\x01\0\0\0\0\x81\x10\x15a\x1D\xB0W[b\x01\0\0\x81\x10\x15a\x1D\xA3W[a\x01\0\x81\x10\x15a\x1D\x96W[`\x10\x81\x10\x15a\x1D\x89W[\x10\x15a\x1D\x81W[`\x03\x02`\x01\x1C\x90V[a\x1D7\x81\x8Ba\x1C`V[\x01`\x01\x1C\x90V[a\x1D7\x81\x8Aa\x1C`V[a\x1D7\x81\x89a\x1C`V[a\x1D7\x81\x88a\x1C`V[a\x1D7\x81\x87a\x1C`V[a\x1D7\x81\x86a\x1C`V[\x80\x93a\x1C`V[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1Ba\x1D$V[`\x04\x1C\x91`\x02\x1B\x91a\x1D\x1DV[`\x08\x1C\x91`\x04\x1B\x91a\x1D\x13V[`\x10\x1C\x91`\x08\x1B\x91a\x1D\x08V[` \x1C\x91`\x10\x1B\x91a\x1C\xFCV[`@\x1C\x91` \x1B\x91a\x1C\xEEV[PPa\x1D|a\x1Dwa\x1Dpa\x1Dfa\x1D\\a\x1DRa\x1DHa\x1D>a\x1D-a\x1D\xF1\x8A`\x80\x1C\x90V[\x98Ph\x01\0\0\0\0\0\0\0\0\x97Pa\x1C\xBD\x96PPPPPPPV[\x91\x90[\x83\x82\x10a\x1E\x1CWPPP\x90V[\x90\x91\x92\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x07\x7FW\x84_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x10_\x14a\x1E^WP\x92[\x91\x90a\x1E\x0FV[\x93\x92Pa\x1Ej\x90a\x19\xD9V[\x91a\x1EWV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x84\x82\x14\x15\x80a\x1F\xBAW[a\x1E\x9EW[PPPPPV[\x81a\x1FDW[PP\x82a\x1E\xB3W[\x80\x80a\x1E\x97V[a\x1F9a\x1F \x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x93a\x1F\x1Aa\x1F\x14y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[\x91a\"@V[\x90a#\x14V[`@\x80Q\x92\x85\x16\x83R\x93\x16` \x82\x01R\x91\x82\x91\x90\x82\x01\x90V[\x03\x90\xA2_\x80\x80a\x1E\xACV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x1F\xB0a\x1F a\x1F\xA1\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x94`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[a\x1F\xAA\x88a\"@V[\x90a\"\xB0V[\x03\x90\xA2_\x80a\x1E\xA4V[P\x83\x15\x15a\x1E\x92V[`\xFF\x81\x14a \"W`\xFF\x81\x16\x90`\x1F\x82\x11a\x1F\xFAW`@Q\x91a\x1F\xE7`@\x84a\x13\xF4V[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[\x7F\xB3Q+\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P`@Qa\x03.\x81a 5\x81`\x06a\x13\x06V[\x03\x82a\x13\xF4V[`\xFF\x81\x14a `W`\xFF\x81\x16\x90`\x1F\x82\x11a\x1F\xFAW`@Q\x91a\x1F\xE7`@\x84a\x13\xF4V[P`@Qa\x03.\x81a 5\x81`\x07a\x13\x06V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a \xF5W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a \xEAW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a \xE0W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[`@Q=_\x82>=\x90\xFD[PPP_\x91`\x03\x91\x90V[`\x04\x11\x15a!\nWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a!@\x81a!\0V[\x80a!IWPPV[a!R\x81a!\0V[`\x01\x81\x03a!\x82W\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a!\x8B\x81a!\0V[`\x02\x81\x03a!\xBFWP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80a!\xCB`\x03\x92a!\0V[\x14a!\xD3WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80T\x82\x10\x15a\"\x13W_R` _ \x01\x90_\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\"\x80Wy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\xD0`\x04R`$R`D_\xFD[\x90a\"\xBACa\x1C\x18V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a\"\xE0\x85a\x1A\x8AV[\x92\x16\x91\x16\x03\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07\x7FWa#\x10\x92a%\xC4V[\x90\x91V[\x90a#\x1ECa\x1C\x18V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a#D\x85a\x1A\x8AV[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07\x7FWa#\x10\x92a%\xC4V[a#}Ca\x1C\x18V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a#\xA4`\x0Ba\x1A\x8AV[\x92\x16\x91\x16\x01y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x7FWa#\x10\x91`\x0Ba%\xC4V[a#\xDECa\x1C\x18V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a$\x05`\x0Ba\x1A\x8AV[\x92\x16\x91\x16\x03y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x7FWa#\x10\x91`\x0Ba%\xC4V[\x90`\x01`\x01`\xA0\x1B\x03a\x14D\x92a$Ta$O\x84a\"@V[a#tV[PP\x16\x80\x15a$\xA1W[`\t` R\x7F\xEC\x81Vq\x8A\x83r\xB1\xDBD\xBBA\x147\xD0\x87\x0F>7\x90\xD4\xA0\x85&\xD0$\xCE\x1B\x0Bf\x8FkT_\x91\x82R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x1EpV[a$\xB2a$\xAD\x83a\"@V[a#\xD5V[PPa$^V[\x90`\x01`\x01`\xA0\x1B\x03\x80a\x14D\x94\x93\x16\x91\x82\x15a%\x1EW[\x16\x90\x81\x15a%\x0BW[_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90a\x1EpV[a%\x17a$\xAD\x84a\"@V[PPa$\xDAV[a%*a$O\x85a\"@V[PPa$\xD1V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x13\xEFWa%S\x91`\x01\x82\x01\x81Ua!\xFEV[a%\x98W\x81Q` \x92\x90\x92\x01Q`0\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a&\xBAWa%\xDBa%\xE6\x91a\x19\xCBV[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a&\x92W\x87\x93\x03a&KWPa&G\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x83T\x92`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa&G\x91a&ka&]a\x145V[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16` \x83\x01Ra%1V[\x7F% `\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P\x90a&\xF2\x91a&\xCBa&]a\x145V[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x83\x01Ra%1V[_\x91\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x15\xA2\x8D&\xFA\x1B\xF76\xCF~\xDC\x99\"`qq\xCC\xB0\x9C<s\xB8\x08\xE7w*0\x13\xE0h\xA5\"\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c9081630481205414615c9c575080630642dde514615c255780630754617214615bfe5780630a9254e4146154795780631074a21f1461533d578063123a4a5f146151b85780631ed7831c1461513a5780632246e5cc14614c135780632ade388014614a1f5780633e5e3c23146149a15780633f7286f41461492357806341686ff21461423657806347483c5d14613d515780634f8632ba14613d2a57806355f7d47714613a805780635bb177811461354b5780636338aa86146132c1578063640f725a1461302857806366d9a9a014612eeb57806371d7dabf14612b48578063746a9bcf146129c757806376029e78146126cf57806384ef8ffc146126a957806385226c811461261f57806388c5671b146122495780638d31ed5314611e735780638f08ece714611b7b5780638f310dfe14611782578063916a17c6146116d857806391dc0b2d14611352578063b0464fdc146112a8578063b5508aa91461121e578063b78b596714610d1b578063ba414fa614610cf6578063bbb155331461079f578063c01e9428146104f8578063dccc57f1146102e5578063e20c9f7114610257578063e8edc81614610230578063fa7626d41461020d5763fc0c546a146101e1575f80fd5b3461020a578060031936011261020a5760206001600160a01b03601f5460081c16604051908152f35b80fd5b503461020a578060031936011261020a57602060ff601f54166040519015158152f35b503461020a578060031936011261020a5760206001600160a01b0360235416604051908152f35b503461020a578060031936011261020a5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106102c6576102c2856102b681870382615fbd565b60405191829182615da4565b0390f35b82546001600160a01b031684526020909301926001928301920161029f565b503461020a578060031936011261020a576001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba5783916104c5575b50602080546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101939093526001600160a01b0316602483015281604481855afa80156104ba576103a1918491610448575b50616a0a565b6040517fd5391393000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba578391610482575b506021546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529060209082908180604481015b03915afa8015610477576104459183916104485750616a0a565b80f35b61046a915060203d602011610470575b6104628183615fbd565b810190616170565b5f61039b565b503d610458565b6040513d84823e3d90fd5b90506020813d6020116104b2575b8161049d60209383615fbd565b810103126104ae575161042b6103de565b5f80fd5b3d9150610490565b6040513d85823e3d90fd5b90506020813d6020116104f0575b816104e060209383615fbd565b810103126104ae57516020610342565b3d91506104d3565b503461020a57604060031936011261020a576004356001600160a01b0381169081810361076d57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561076d57826040517f4c63e56200000000000000000000000000000000000000000000000000000000815283151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156104775761078a575b5061059c6024356168c6565b916001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561076d576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104ba578391610771575b50506001600160a01b03601f5460081c16803b1561076d576040516340c10f1960e01b81526001600160a01b039290921660048301526024820184905282908290604490829084905af1801561047757610758575b50506001600160a01b03601f5460081c1691604051906370a0823160e01b82526004820152602081602481865afa90811561074d57849161071a575b506004926106a383602093616850565b604051938480927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa80156104ba5783906106e6575b6104459250616850565b506020823d602011610712575b8161070060209383615fbd565b810103126104ae5761044591516106dc565b3d91506106f3565b90506020813d602011610745575b8161073560209383615fbd565b810103126104ae57516004610693565b3d9150610728565b6040513d86823e3d90fd5b8161076291615fbd565b61076d57825f610657565b8280fd5b8161077b91615fbd565b61078657815f610602565b5080fd5b8161079491615fbd565b61076d57825f610590565b503461020a578060031936011261020a576040517fffa18649000000000000000000000000000000000000000000000000000000008152620a11ce6004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610477578291610cc7575b50610e10420190814211610b57576001600160a01b03601f5460081c169183604051937f7ecebe000000000000000000000000000000000000000000000000000000000085526001600160a01b03841692836004870152602086602481855afa9586156104ba578396610c90575b506001600160a01b0360235416906040516020810190610904816108f6858c888d88909493926001600160a01b0360a0938160c08501987f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98652166020850152166040830152683635c9adc5dea00000606083015260808201520152565b03601f198101835282615fbd565b519020604051907f3644e515000000000000000000000000000000000000000000000000000000008252602082600481885afa918215610c85578692610c4c575b506040517f1901000000000000000000000000000000000000000000000000000000000000602082019081526022820193909352604281019190915261098e81606281016108f6565b51902092604051937fe341eaa4000000000000000000000000000000000000000000000000000000008552620a11ce60048601526024850152606084604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa928315610bec57859486908795610c14575b50823b15610c105788610a7d88968793604051998a98899788967fd505accf00000000000000000000000000000000000000000000000000000000885260048801929360c0946001600160a01b0360ff93999897948160e088019b168752166020860152683635c9adc5dea000006040860152606085015216608083015260a08201520152565b03925af1801561047757610bf7575b5050601f546023546040517fdd62ed3e0000000000000000000000000000000000000000000000000000000081526001600160a01b039485166004820152908416602482015260089190911c9092169190602082604481865afa918215610bec578592610bb6575b50610b00602092616654565b6024604051809481937f7ecebe0000000000000000000000000000000000000000000000000000000000835260048301525afa9081156104ba578391610b84575b5060018201809211610b57579061044591616850565b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011610bae575b81610b9f60209383615fbd565b810103126104ae57515f610b41565b3d9150610b92565b91506020823d602011610be4575b81610bd160209383615fbd565b810103126104ae57905190610b00610af4565b3d9150610bc4565b6040513d87823e3d90fd5b81610c0191615fbd565b610c0c57835f610a8c565b8380fd5b8680fd5b91955050610c3b91935060603d606011610c45575b610c338183615fbd565b8101906164d9565b939194905f6109f6565b503d610c29565b955090506020853d602011610c7d575b81610c6960209383615fbd565b810103126104ae579351889461098e610945565b3d9150610c5c565b6040513d88823e3d90fd5b925094506020823d602011610cbf575b81610cad60209383615fbd565b810103126104ae57859151945f610878565b3d9150610ca0565b610ce9915060203d602011610cef575b610ce18183615fbd565b8101906164ba565b5f61080a565b503d610cd7565b503461020a578060031936011261020a576020610d116164fb565b6040519015158152f35b503461020a578060031936011261020a57604051907fffa18649000000000000000000000000000000000000000000000000000000008252620a11ce6004830152602082602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9182156112115781926111f0575b507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4201914283116111c35781926001600160a01b03601f5460081c166040517f7ecebe000000000000000000000000000000000000000000000000000000000081526001600160a01b0384166004820152602081602481855afa908115610bec57859161118b575b5090602060049284610e936001600160a01b0360235416926108f6604051938492878401968c88909493926001600160a01b0360a0938160c08501987f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98652166020850152166040830152683635c9adc5dea00000606083015260808201520152565b51902091604051938480927f3644e5150000000000000000000000000000000000000000000000000000000082525afa918215610bec578592611152575b506040517f19010000000000000000000000000000000000000000000000000000000000006020820190815260228201939093526042810191909152610f1a81606281016108f6565b51902090604051917fe341eaa4000000000000000000000000000000000000000000000000000000008352620a11ce60048401526024830152606082604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561074d57849385938692611128575b506040517f6279130200000000000000000000000000000000000000000000000000000000602082015283602482015260248152610fc0604482615fbd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c10578661101b91604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561111d578791611104575b50506001600160a01b03601f5460081c16916001600160a01b036023541691833b15611100576040517fd505accf0000000000000000000000000000000000000000000000000000000081526001600160a01b03918216600482015292166024830152683635c9adc5dea000006044830152606482019390935260ff94909416608485015260a484019290925260c48301528290829060e490829084905af18015610477576110ef5750f35b816110f991615fbd565b61020a5780f35b8780fd5b8161110e91615fbd565b61111957855f611043565b8580fd5b6040513d89823e3d90fd5b9150935061114691925060603d606011610c4557610c338183615fbd565b9291939092905f610f81565b945090506020843d602011611183575b8161116f60209383615fbd565b810103126104ae5792518493610f1a610ed1565b3d9150611162565b919450506020813d6020116111bb575b816111a860209383615fbd565b810103126104ae57518493906020610e10565b3d915061119b565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b61120a91925060203d602011610cef57610ce18183615fbd565b905f610d87565b50604051903d90823e3d90fd5b503461020a578060031936011261020a5760195461123b81615fe0565b916112496040519384615fbd565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b83831061128b57604051806102c28782615e60565b60016020819261129a85615ff8565b815201920192019190611276565b503461020a578060031936011261020a57601c546112c581615fe0565b916112d36040519384615fbd565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b83831061131557604051806102c28782615edd565b6002602060019260405161132881615f74565b6001600160a01b038654168152611340858701616195565b83820152815201920192019190611300565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576116c3575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af18015610477576116ae575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611696575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156116265782916024839260405194859384927f5c19a95c00000000000000000000000000000000000000000000000000000000845260048401525af1801561047757611681575b506001600160a01b03601f5460081c166001600160a01b0360235416906040517f9ab24eb0000000000000000000000000000000000000000000000000000000008152826004820152602081602481855afa90811561074d578491611649575b5061155590616654565b60206001600160a01b03602254166024604051809481937f587cde1e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156104ba57839161162a575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611626576040517f515361f60000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152911660248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610477576110ef5750f35b5050fd5b611643915060203d602011610cef57610ce18183615fbd565b5f6115a4565b9350506020833d602011611679575b8161166560209383615fbd565b810103126104ae576115558493519061154b565b3d9150611658565b8161168b91615fbd565b61020a57805f6114eb565b816116a091615fbd565b61020a57805f611485565b50fd5b816116b891615fbd565b61020a57805f611422565b816116cd91615fbd565b61020a57805f6113c6565b503461020a578060031936011261020a57601d546116f581615fe0565b916117036040519384615fbd565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b83831061174557604051806102c28782615edd565b6002602060019260405161175881615f74565b6001600160a01b038654168152611770858701616195565b83820152815201920192019190611730565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611b66575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152681b1ae4d6e2ef50000060248401525af1801561047757611b51575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152681043561a882930000060248401525af1801561047757611b3c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611b27575b506001600160a01b03601f5460081c166001600160a01b0360225416604051906370a0823160e01b82526004820152602081602481855afa9081156104ba578391611aef575b50611983906167d1565b6001600160a01b0360235416604051906370a0823160e01b82526004820152602081602481855afa9081156104ba578391611ab7575b506004916119c8602092616752565b604051928380927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa908115610477578291611a82575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152682b5e3af16b1880000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610477576110ef5750f35b9150506020813d602011611aaf575b81611a9e60209383615fbd565b810103126104ae578190515f611a02565b3d9150611a91565b9250506020823d602011611ae7575b81611ad360209383615fbd565b810103126104ae57905182919060046119b9565b3d9150611ac6565b9250506020823d602011611b1f575b81611b0b60209383615fbd565b810103126104ae5761198383925190611979565b3d9150611afe565b81611b3191615fbd565b61020a57805f611933565b81611b4691615fbd565b61020a57805f6118c7565b81611b5b91615fbd565b61020a57805f61186b565b81611b7091615fbd565b61020a57805f61180f565b503461020a578060031936011261020a57806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611e5e575b5060046001600160a01b036022541660206001600160a01b03601f5460081c16604051938480927fd53913930000000000000000000000000000000000000000000000000000000082525afa9182156104ba578392611e25575b506040517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b0390911660248201526044810191909152611cb181606481016108f6565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab5781611d0c91604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757611e10575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af1801561047757611dfb575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576110ef5750f35b81611e0591615fbd565b61020a57805f611d8d565b81611e1a91615fbd565b61020a57805f611d31565b925090506020823d602011611e56575b81611e4260209383615fbd565b810103126104ae5790518291611cb1611c62565b3d9150611e35565b81611e6891615fbd565b61020a57805f611c08565b503461020a578060031936011261020a57806001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba578391612211575b50602080546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101939093526001600160a01b03166024830181905292829060449082905afa80156104ba57611f349184916104485750616a0a565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576121fc575b506001600160a01b03601f5460081c166040517fd5391393000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba5783916121c7575b506001600160a01b0360225416823b156121c2576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af18015610477576121ad575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757612198575b50506001600160a01b03601f5460081c166040517fd5391393000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba578391612164575b506022546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b0316602482015290602090829081806044810161042b565b90506020813d602011612190575b8161217f60209383615fbd565b810103126104ae575161042b612113565b3d9150612172565b816121a291615fbd565b61020a57805f6120c5565b816121b791615fbd565b61020a57805f612059565b505050fd5b9250506020823d6020116121f4575b816121e360209383615fbd565b810103126104ae578291515f611ff0565b3d91506121d6565b8161220691615fbd565b61020a57805f611fa3565b9250506020823d602011612241575b8161222d60209383615fbd565b810103126104ae5790518291906020611ed1565b3d9150612220565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104775761260a575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af18015610477576125f5575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281818061238460048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576125e0575b506001600160a01b03602254166001600160a01b0360235416817fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6020604051681043561a88293000008152a3737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576125cb575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fa9059cbb0000000000000000000000000000000000000000000000000000000084526004840152681043561a882930000060248401525af18015610477576125ae575b506001600160a01b03601f5460081c166001600160a01b0360225416604051906370a0823160e01b82526004820152602081602481855afa80156104ba57839061257a575b61250791506166d3565b60206001600160a01b03602354166024604051809481936370a0823160e01b835260048301525afa8015610477578290612546575b6104459150616752565b506020813d602011612572575b8161256060209383615fbd565b810103126104ae57610445905161253c565b3d9150612553565b506020813d6020116125a6575b8161259460209383615fbd565b810103126104ae5761250790516124fd565b3d9150612587565b6125c69060203d602011610470576104628183615fbd565b6124b8565b816125d591615fbd565b61020a57805f61244c565b816125ea91615fbd565b61020a57805f6123a9565b816125ff91615fbd565b61020a57805f612319565b8161261491615fbd565b61020a57805f6122bd565b503461020a578060031936011261020a57601a5461263c81615fe0565b9161264a6040519384615fbd565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b83831061268c57604051806102c28782615e60565b60016020819261269b85615ff8565b815201920192019190612677565b503461020a578060031936011261020a5760206001600160a01b03815416604051908152f35b503461020a578060031936011261020a576001600160a01b03601f5460081c166040517f06fdde030000000000000000000000000000000000000000000000000000000081528281600481855afa9081156104ba5783916129ad575b5061276e60409182519061273f8483615fbd565b601182527f546573746e65742053796e6469636174650000000000000000000000000000006020830152616973565b80517f95d89b410000000000000000000000000000000000000000000000000000000081528381600481865afa9081156129a357906127eb918591612981575b508251906127bc8483615fbd565b600b82527f546573746e657453594e440000000000000000000000000000000000000000006020830152616973565b8281517f313ce567000000000000000000000000000000000000000000000000000000008152602081600481875afa90811561293d578291612947575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107865760ff8351917f98296c54000000000000000000000000000000000000000000000000000000008352166004820152601260248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561293d57612928575b505060206004928251938480927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa90811561291f575082906128eb575b61044591506165d4565b506020813d602011612917575b8161290560209383615fbd565b810103126104ae5761044590516128e1565b3d91506128f8565b513d84823e3d90fd5b8161293291615fbd565b61076d57825f6128a0565b83513d84823e3d90fd5b90506020813d602011612979575b8161296260209383615fbd565b8101031261078657612973906164ac565b5f612828565b3d9150612955565b61299d91503d8087833e6129958183615fbd565b810190616439565b5f6127ae565b82513d86823e3d90fd5b6129c191503d8085833e6129958183615fbd565b5f61272b565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757612b33575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757612b1e575b506001600160a01b03601f5460081c16803b156116ab578180916044604051809481936340c10f1960e01b8352816004840152683635c9adc5dea0000060248401525af18015610477576110ef5750f35b81612b2891615fbd565b61020a57805f612acd565b81612b3d91615fbd565b61020a57805f612a3b565b503461020a57604060031936011261020a57612b656004356168c6565b612b73816001602435616ab3565b604083808251612b838482615fbd565b600c81527f426f756e6420726573756c74000000000000000000000000000000000000000060208201528351612c0281612bee60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528860248401526064830190615de6565b88604483015203601f198101835282615fbd565b51906a636f6e736f6c652e6c6f675afa50836001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107865782519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561293d57612ed6575b506001600160a01b03601f5460081c166001600160a01b036022541690803b1561076d5783516340c10f1960e01b81526001600160a01b039290921660048301526024820186905282908290604490829084905af1801561293d57612ec1575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107865782519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561293d57612eac575b5050601f5460235482517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201526024810185905291602091839160081c168188816044810103925af18015612ea257612e85575b506001600160a01b03601f5460081c16926001600160a01b0360225416908251916370a0823160e01b83526004830152602082602481885afa918215612e7b57908492918792612e42575b50612dfa92612df491616188565b90616850565b60206001600160a01b036023541660248351809681936370a0823160e01b835260048301525afa908115612e39575083906106e6576104459250616850565b513d85823e3d90fd5b925090506020823d602011612e73575b81612e5f60209383615fbd565b810103126104ae5790518391612dfa612de6565b3d9150612e52565b83513d88823e3d90fd5b612e9d9060203d602011610470576104628183615fbd565b612d9b565b82513d87823e3d90fd5b81612eb691615fbd565b610c0c57835f612d37565b81612ecb91615fbd565b610c0c57835f612cd5565b81612ee091615fbd565b610c0c57835f612c75565b503461020a578060031936011261020a57601b54612f0881615fe0565b612f156040519182615fbd565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310612fed57868587604051928392602084019060208552518091526040840160408260051b8601019392905b828210612f8257505050500390f35b91936020612fdd827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083612fcd8351604084526040840190615de6565b9201519084818403910152615e0b565b9601920192018594939192612f73565b6002602060019260405161300081615f74565b61300986615ff8565b8152613016858701616195565b83820152815201920192019190612f45565b503461020a578060031936011261020a57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f491cc7c20000000000000000000000000000000000000000000000000000000081528181806130a260048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576132ac575b506001600160a01b03602254166001600160a01b0360235416817f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9256020604051681b1ae4d6e2ef5000008152a3737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613297575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152681b1ae4d6e2ef50000060248401525af180156104775761327a575b50601f546022546023546040517fdd62ed3e0000000000000000000000000000000000000000000000000000000081526001600160a01b0392831660048201529082166024820152916020918391604491839160081c165afa8015610477578290613246575b61044591506167d1565b506020813d602011613272575b8161326060209383615fbd565b810103126104ae57610445905161323c565b3d9150613253565b6132929060203d602011610470576104628183615fbd565b6131d6565b816132a191615fbd565b61020a57805f61316a565b816132b691615fbd565b61020a57805f6130c7565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613536575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af1801561047757613521575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104775761350c575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916024839260405194859384927f5c19a95c00000000000000000000000000000000000000000000000000000000845260048401525af18015610477576134f7575b50506001600160a01b03601f5460081c1660206001600160a01b03602254166024604051809481937fbb4d443600000000000000000000000000000000000000000000000000000000835260048301525afa80156104775782906134c3575b6104459150616654565b506020813d6020116134ef575b816134dd60209383615fbd565b810103126104ae5761044590516134b9565b3d91506134d0565b8161350191615fbd565b61020a57805f61345a565b8161351691615fbd565b61020a57805f6133f4565b8161352b91615fbd565b61020a57805f613391565b8161354091615fbd565b61020a57805f613335565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613a6b575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af1801561047757613a56575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613a41575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152681b1ae4d6e2ef50000060248401525af1801561047757613a24575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613a0f575b50506001600160a01b03601f5460081c1660206001600160a01b036022541660646001600160a01b0360235416918560405195869485937f23b872dd00000000000000000000000000000000000000000000000000000000855260048501526024840152681043561a882930000060448401525af18015610477576139f2575b50806001600160a01b03601f5460081c166001600160a01b0360225416906040516370a0823160e01b8152826004820152602081602481855afa90811561074d5784916139ba575b50613820906166d3565b6001600160a01b0360235416916040516370a0823160e01b8152836004820152602081602481865afa908115610bec57859161397d575b50916138b69391613869602094616752565b6040518095819482937fdd62ed3e000000000000000000000000000000000000000000000000000000008452600484019092916001600160a01b0360209181604085019616845216910152565b03915afa908115610477578291613948575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152680ad78ebc5ac620000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610477576110ef5750f35b9150506020813d602011613975575b8161396460209383615fbd565b810103126104ae578190515f6138c8565b3d9150613957565b92945050916020823d6020116139b2575b8161399b60209383615fbd565b810103126104ae57905184939192906138b6613857565b3d915061398e565b9350506020833d6020116139ea575b816139d660209383615fbd565b810103126104ae5761382084935190613816565b3d91506139c9565b613a0a9060203d602011610470576104628183615fbd565b6137ce565b81613a1991615fbd565b61020a57805f61374e565b613a3c9060203d602011610470576104628183615fbd565b6136ea565b81613a4b91615fbd565b61020a57805f61367e565b81613a6091615fbd565b61020a57805f61361b565b81613a7591615fbd565b61020a57805f6135bf565b503461020a578060031936011261020a57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152818180613afa60048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613d15575b50506001600160a01b0360225416817fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6020604051683635c9adc5dea000008152a3806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757613d00575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af1801561047757613ceb575b50506001600160a01b03601f5460081c166001600160a01b0360225416604051906370a0823160e01b82526004820152602081602481855afa9081156104ba578391613cb8575b50600491613c76602092616654565b604051928380927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa80156104775782906134c3576104459150616654565b90506020813d602011613ce3575b81613cd360209383615fbd565b810103126104ae57516004613c67565b3d9150613cc6565b81613cf591615fbd565b61020a57805f613c20565b81613d0a91615fbd565b61020a57805f613bc4565b81613d1f91615fbd565b61020a57805f613b1f565b503461020a578060031936011261020a5760206001600160a01b0360225416604051908152f35b503461020a578060031936011261020a57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f491cc7c200000000000000000000000000000000000000000000000000000000815260016004820152600160248201526001604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757614221575b50600460206001600160a01b03601f5460081c16604051928380927fd53913930000000000000000000000000000000000000000000000000000000082525afa9081156104775782916141ec575b506001600160a01b03602154166001600160a01b03602054168091604051937ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b8680a4737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611626577f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576141d7575b506001600160a01b03601f5460081c166040517fd5391393000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba5783916141a2575b506001600160a01b0360215416823b156121c2576040517fd547741f00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af180156104775761418d575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757614178575b506001600160a01b03601f5460081c16604051907fd5391393000000000000000000000000000000000000000000000000000000008252602082600481845afa9182156104ba578392614140575b506021546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101939093526001600160a01b03166024830152602090829060449082905afa908115610477578291614121575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610477576110ef5750f35b61413a915060203d602011610470576104628183615fbd565b5f6140ae565b925090506020823d602011614170575b8161415d60209383615fbd565b810103126104ae57905182916020614054565b3d9150614150565b8161418291615fbd565b61020a57805f614006565b8161419791615fbd565b61020a57805f613f9a565b9250506020823d6020116141cf575b816141be60209383615fbd565b810103126104ae578291515f613f31565b3d91506141b1565b816141e191615fbd565b61020a57805f613ee4565b9150506020813d602011614219575b8161420860209383615fbd565b810103126104ae578190515f613e36565b3d91506141fb565b8161422b91615fbd565b61020a57805f613de8565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104775761490e575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152683635c9adc5dea0000060248401525af18015610477576148f9575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156116265782916044839260405194859384926340c10f1960e01b84526004840152686c6b935b8bbd40000060248401525af18015610477576148e4575b506001600160a01b03601f5460081c166143936160fb565b60405160208101906143bf602082855180838801875e810188838201520301601f198101835282615fbd565b519020604051907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561074d5784916148c5575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156121c257836001600160a01b036144869260405193849283927fc657c7180000000000000000000000000000000000000000000000000000000084521695866004840152604060248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561074d5784916148b0575b5050813b156116265782916044839260405194859384926340c10f1960e01b8452600484015268a2a15d09519be0000060248401525af180156104775761489b575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757614886575b50506001600160a01b03601f5460081c166040517f18160ddd000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104ba578391614854575b506001600160a01b0360225416604051906370a0823160e01b82526004820152602081602481865afa90811561074d578491614822575b506001600160a01b036023541690604051916370a0823160e01b83526004830152602082602481875afa908115610bec5785916147ec575b6146239250616136565b9161462c6160fb565b846040516020810190614659602082865180838901875e810186838201520301601f198101835282615fbd565b519020604051907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156104775782916147cd575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b036147209260405193849283927fc657c7180000000000000000000000000000000000000000000000000000000084521696876004840152604060248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610477576147b4575b50506020906024604051809481936370a0823160e01b835260048301525afa90811561074d578491614780575b50612df49061044593616136565b90506020813d6020116147ac575b8161479b60209383615fbd565b810103126104ae5751610445614772565b3d915061478e565b816147be91615fbd565b6147c957845f614745565b8480fd5b6147e6915060203d602011610cef57610ce18183615fbd565b5f6146b3565b90506020823d60201161481a575b8161480760209383615fbd565b810103126104ae57614623915190614619565b3d91506147fa565b90506020813d60201161484c575b8161483d60209383615fbd565b810103126104ae57515f6145e1565b3d9150614830565b90506020813d60201161487e575b8161486f60209383615fbd565b810103126104ae57515f6145aa565b3d9150614862565b8161489091615fbd565b61020a57805f61455c565b816148a591615fbd565b61020a57805f6144f0565b816148ba91615fbd565b61162657825f6144ae565b6148de915060203d602011610cef57610ce18183615fbd565b5f614419565b816148ee91615fbd565b61020a57805f61437b565b8161490391615fbd565b61020a57805f61431f565b8161491891615fbd565b61020a57805f6142c3565b503461020a578060031936011261020a5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110614982576102c2856102b681870382615fbd565b82546001600160a01b031684526020909301926001928301920161496b565b503461020a578060031936011261020a5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110614a00576102c2856102b681870382615fbd565b82546001600160a01b03168452602090930192600192830192016149e9565b503461020a578060031936011261020a57601e54614a3c81615fe0565b614a496040519182615fbd565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b838310614b8a5786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310614ab55786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110614b4157505050505060208060019297019301930190928695949293614aa8565b9091929394602080614b7d837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951615de6565b9701950193929101614b1d565b604051614b9681615f74565b6001600160a01b038354168152600183018054614bb281615fe0565b91614bc06040519384615fbd565b8183528a526020808b20908b9084015b838210614bf6575050505060019282602092836002950152815201920192019190614a79565b600160208192614c0586615ff8565b815201930191019091614bd0565b503461020a578060031936011261020a57604081815191614c348184615fbd565b600c8352602083017f77726f6e6741646472657373000000000000000000000000000000000000000081528151600c6020820192835e83602c820152600c8152614c7f602c82615fbd565b5190208151907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561503c57839161511b575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561076d5781517fc657c718000000000000000000000000000000000000000000000000000000008152838180614d406001600160a01b038616988960048401528760248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561505d57908491615106575b50506001600160a01b03601f5460081c16908251917fa217fddf000000000000000000000000000000000000000000000000000000008352602083600481845afa9283156150fc5785936150c0575b5094602084959660049551958680927fd53913930000000000000000000000000000000000000000000000000000000082525afa93841561507f578694615089575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611119578451907f06447d560000000000000000000000000000000000000000000000000000000082526004820152858160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561507f5790869161506a575b505083517fe2517d3f0000000000000000000000000000000000000000000000000000000060208201526001600160a01b0390911660248201526044810191909152614eba81606481016108f6565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156121c25783614f14918451809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561505d57908491615048575b50506001600160a01b03601f5460081c16906001600160a01b0360235416823b156147c95783517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529083908290604490829084905af1801561503c57908391615027575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561291f57506110ef5750f35b8161503191615fbd565b6116ab57815f614fb8565b505051903d90823e3d90fd5b8161505291615fbd565b61162657825f614f3c565b50505051903d90823e3d90fd5b8161507491615fbd565b6147c957845f614e6b565b85513d88823e3d90fd5b955092506020853d6020116150b8575b816150a660209383615fbd565b810103126104ae57859451925f614df9565b3d9150615099565b93945091506020833d6020116150f4575b816150de60209383615fbd565b810103126104ae57915185939290916020614db7565b3d91506150d1565b84513d87823e3d90fd5b8161511091615fbd565b61076d57825f614d68565b615134915060203d602011610cef57610ce18183615fbd565b5f614cd8565b503461020a578060031936011261020a5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110615199576102c2856102b681870382615fbd565b82546001600160a01b0316845260209093019260019283019201615182565b503461020a578060031936011261020a57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156116ab576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757615328575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f1f2a2005000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757615313575b506001600160a01b03601f5460081c166001600160a01b0360225416813b156116265782916044839260405194859384926340c10f1960e01b845260048401528160248401525af18015610477576110ef5750f35b8161531d91615fbd565b61020a57805f6152be565b8161533291615fbd565b61020a57805f61522c565b503461020a578060031936011261020a57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020a57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561047757615464575b50506001600160a01b036021541660405190613013908183019183831067ffffffffffffffff84111761543757918391604093616c778439858252602082015203019082f01561542b5780f35b604051903d90823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b8161546e91615fbd565b61020a57805f6153de565b503461020a578060031936011261020a57604080516154988282615fbd565b600c815282602082017f64656661756c7441646d696e000000000000000000000000000000000000000081528351600c6020820192835e82602c820152600c81526154e4602c82615fbd565b5190208351907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615b39578291615bdf575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b036155a892865193849283927fc657c71800000000000000000000000000000000000000000000000000000000845216968760048401528860248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615b3957615bca575b50507fffffffffffffffffffffffff0000000000000000000000000000000000000000602054161760205580516156048282615fbd565b6006815282602082017f6d696e74657200000000000000000000000000000000000000000000000000008152835160066020820192835e82602682015260068152615650602682615fbd565b5190208351907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615b39578291615bab575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b0361571492865193849283927fc657c71800000000000000000000000000000000000000000000000000000000845216968760048401528860248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615b3957615b96575b50507fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215580516157708282615fbd565b6004815282602082017f75736572000000000000000000000000000000000000000000000000000000008152835160046020820192835e826024820152600481526157bc602482615fbd565b5190208351907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615b39578291615b77575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b0361588092865193849283927fc657c71800000000000000000000000000000000000000000000000000000000845216968760048401528860248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615b3957615b62575b50507fffffffffffffffffffffffff0000000000000000000000000000000000000000602254161760225580516158dc8282615fbd565b6007815282602082017f7370656e646572000000000000000000000000000000000000000000000000008152835160076020820192835e82602782015260078152615928602782615fbd565b5190208351907fffa186490000000000000000000000000000000000000000000000000000000082526004820152602081602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615b39578291615b43575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561078657816001600160a01b036159ec92865193849283927fc657c71800000000000000000000000000000000000000000000000000000000845216968760048401528860248401526044830190615de6565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615b3957615b24575b50507fffffffffffffffffffffffff000000000000000000000000000000000000000060235416176023556001600160a01b03602054166001600160a01b03602154168251916130138084019084821067ffffffffffffffff831117615af75791849391615a9893616c7786396001600160a01b0391821681529116602082015260400190565b039083f0908115615aec57507fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f5580f35b51913d9150823e3d90fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b81615b2e91615fbd565b61076d57825f615a11565b84513d84823e3d90fd5b615b5c915060203d602011610cef57610ce18183615fbd565b5f615981565b81615b6c91615fbd565b61076d57825f6158a5565b615b90915060203d602011610cef57610ce18183615fbd565b5f615815565b81615ba091615fbd565b61076d57825f615739565b615bc4915060203d602011610cef57610ce18183615fbd565b5f6156a9565b81615bd491615fbd565b61076d57825f6155cd565b615bf8915060203d602011610cef57610ce18183615fbd565b5f61553d565b503461020a578060031936011261020a5760206001600160a01b0360215416604051908152f35b503461020a578060031936011261020a576001600160a01b03601f5460081c1660206001600160a01b03602254166024604051809481937fbb4d443600000000000000000000000000000000000000000000000000000000835260048301525afa80156104775782906128eb5761044591506165d4565b9050346104ae575f6003193601126104ae57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae577fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d0000000000000000000000000000000000000000000000000000000060048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015615d9957615d86575b506001600160a01b036020541660405190613013908183019183831067ffffffffffffffff84111761543757918391604093616c778439815284602082015203019082f01561542b5780f35b615d9291505f90615fbd565b5f5f615d3a565b6040513d5f823e3d90fd5b60206040818301928281528451809452019201905f5b818110615dc75750505090565b82516001600160a01b0316845260209384019390920191600101615dba565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b818110615e285750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101615e1b565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310615e9257505050505090565b9091929394602080615ece837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951615de6565b97019301930191939290615e83565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310615f0f57505050505090565b9091929394602080615f65837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b03815116845201519181858201520190615e0b565b97019301930191939290615f00565b6040810190811067ffffffffffffffff821117615f9057604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff821117615f9057604052565b67ffffffffffffffff8111615f905760051b60200190565b90604051915f8154908160011c92600183169283156160f1575b6020851084146160c45784875286939081156160845750600114616040575b5061603e92500383615fbd565b565b90505f9291925260205f20905f915b81831061606857505090602061603e928201015f616031565b602091935080600191548385890101520191019091849261604f565b6020935061603e9592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f616031565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f1693616012565b6040519061610a604083615fbd565b600582527f75736572330000000000000000000000000000000000000000000000000000006020830152565b9190820180921161614357565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b908160209103126104ae575180151581036104ae5790565b9190820391821161614357565b90604051918281549182825260208201905f5260205f20925f905b8060078301106163ac5761603e945491818110616376575b818110616340575b81811061630a575b8181106162d4575b81811061629e575b818110616268575b818110616233575b10616206575b500383615fbd565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f6161fe565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b1681520193016161f8565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b1681520193016161f0565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b1681520193016161e8565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b1681520193016161e0565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b1681520193016161d8565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b1681520193016161d0565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b1681520193016161c8565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e08201520194019201859293916161b0565b6020818303126104ae5780519067ffffffffffffffff82116104ae570181601f820112156104ae5780519067ffffffffffffffff8211615f90576040519261648b601f8401601f191660200185615fbd565b828452602083830101116104ae57815f9260208093018386015e8301015290565b519060ff821682036104ae57565b908160209103126104ae57516001600160a01b03811681036104ae5790565b908160609103126104ae576164ed816164ac565b916040602083015192015190565b60085460ff16801561650a5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115615d99575f916165a2575b50151590565b90506020813d6020116165cc575b816165bd60209383615fbd565b810103126104ae57515f61659c565b3d91506165b0565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b5f61603e91615fbd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152683635c9adc5dea0000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526825f273933db570000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152681043561a882930000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152681b1ae4d6e2ef50000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b6fffffffffffffffffffffffffffffffff60016168e292616ab3565b905f806040516168f3604082615fbd565b600c81527f426f756e6420726573756c740000000000000000000000000000000000000000602082015260405161696081612bee60208201947fb60e72cc000000000000000000000000000000000000000000000000000000008652604060248401526064830190615de6565b51906a636f6e736f6c652e6c6f675afa50565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae576169d25f916169e460405194859384937ff320d963000000000000000000000000000000000000000000000000000000008552604060048601526044850190615de6565b90600319848303016024850152615de6565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ae57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015615d995761664a5750565b8115616a86570690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f90838311616bf25782811091821580616be8575b616be057616ad68486616188565b926001840180941161614357600383111580616bd7575b616bc85760031983101580616bbe575b616baa5785831115616b6157505090616b1984616b1e93616188565b616a7c565b908115616b5c57616b2f9250616136565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116161435790565b505090565b959492919095616b72575b50505050565b83949550616b1990616b849394616188565b908115616b5c57616b959250616188565b6001810180911161614357905f808080616b6c565b50509050616bbb9291501990616188565b90565b5082198411616afd565b5050919050616bbb9250616136565b50828411616aed565b509250505090565b5084821115616ac8565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152fdfe61016080604052346104b857604081613013803803809161002082856104bc565b8339810103126104b85761003f6020610038836104df565b92016104df565b60405161004d6040826104bc565b601181526020810170546573746e65742053796e64696361746560781b81526040519061007b6040836104bc565b6011825270546573746e65742053796e64696361746560781b6020830152604051926100a86040856104bc565b600b84526a15195cdd1b995d14d6539160aa1b6020850152604051936100cf6040866104bc565b60018552603160f81b60208601908152845190946001600160401b0382116103bb5760035490600182811c921680156104ae575b602083101461039d5781601f849311610440575b50602090601f83116001146103da575f926103cf575b50508160011b915f199060031b1c1916176003555b8051906001600160401b0382116103bb5760045490600182811c921680156103b1575b602083101461039d5781601f84931161032f575b50602090601f83116001146102c9575f926102be575b50508160011b915f199060031b1c1916176004555b6101ad816105fc565b610120526101ba84610783565b61014052519020918260e05251902080610100524660a0526040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8452604083015260608201524660808201523060a082015260a0815261022360c0826104bc565b5190206080523060c0526001600160a01b038216156102af576001600160a01b038116156102af5761025761025d926104f3565b50610569565b506040516126f790816108bc8239608051816117b9015260a05181611876015260c0518161178a015260e051816118080152610100518161182e01526101205181610adc01526101405181610b050152f35b63d92e233d60e01b5f5260045ffd5b015190505f8061018f565b60045f9081528281209350601f198516905b81811061031757509084600195949392106102ff575b505050811b016004556101a4565b01515f1960f88460031b161c191690555f80806102f1565b929360206001819287860151815501950193016102db565b60045f529091507f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b601f840160051c81019160208510610393575b90601f859493920160051c01905b8181106103855750610179565b5f8155849350600101610378565b909150819061036a565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610165565b634e487b7160e01b5f52604160045260245ffd5b015190505f8061012d565b60035f9081528281209350601f198516905b8181106104285750908460019594939210610410575b505050811b01600355610142565b01515f1960f88460031b161c191690555f8080610402565b929360206001819287860151815501950193016103ec565b60035f529091507fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b601f840160051c810191602085106104a4575b90601f859493920160051c01905b8181106104965750610117565b5f8155849350600101610489565b909150819061047b565b91607f1691610103565b5f80fd5b601f909101601f19168101906001600160401b038211908210176103bb57604052565b51906001600160a01b03821682036104b857565b6001600160a01b0381165f9081525f516020612ff35f395f51905f52602052604090205460ff16610564576001600160a01b03165f8181525f516020612ff35f395f51905f5260205260408120805460ff191660011790553391905f516020612fb35f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f516020612fd35f395f51905f52602052604090205460ff16610564576001600160a01b03165f8181525f516020612fd35f395f51905f5260205260408120805460ff191660011790553391907f9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a6905f516020612fb35f395f51905f529080a4600190565b908151602081105f14610676575090601f815111610636576020815191015160208210610627571790565b5f198260200360031b1b161790565b604460209160405192839163305a27a960e01b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fd5b6001600160401b0381116103bb57600654600181811c91168015610779575b602082101461039d57601f8111610746575b50602092601f82116001146106e557928192935f926106da575b50508160011b915f199060031b1c19161760065560ff90565b015190505f806106c1565b601f1982169360065f52805f20915f5b86811061072e5750836001959610610716575b505050811b0160065560ff90565b01515f1960f88460031b161c191690555f8080610708565b919260206001819286850151815501940192016106f5565b60065f52601f60205f20910160051c810190601f830160051c015b81811061076e57506106a7565b5f8155600101610761565b90607f1690610695565b908151602081105f146107ae575090601f815111610636576020815191015160208210610627571790565b6001600160401b0381116103bb57600754600181811c911680156108b1575b602082101461039d57601f811161087e575b50602092601f821160011461081d57928192935f92610812575b50508160011b915f199060031b1c19161760075560ff90565b015190505f806107f9565b601f1982169360075f52805f20915f5b868110610866575083600195961061084e575b505050811b0160075560ff90565b01515f1960f88460031b161c191690555f8080610840565b9192602060018192868501518155019401920161082d565b60075f52601f60205f20910160051c810190601f830160051c015b8181106108a657506107df565b5f8155600101610899565b90607f16906107cd56fe60806040526004361015610011575f80fd5b5f3560e01c806301ffc9a71461023557806306fdde0314610230578063095ea7b31461022b57806318160ddd146101b857806323b872dd14610226578063248a9ca3146102215780632f2ff15d1461021c578063313ce567146102175780633644e5151461021257806336568abe1461020d5780633a46b1a8146101c257806340c10f19146102085780634bf5d7e914610203578063587cde1e146101fe5780635c19a95c146101f95780636fcfff45146101f457806370a08231146101ef5780637ecebe00146101ea57806384b0196e146101e55780638e539e8c146101e057806391d14854146101db57806391ddadf4146101d657806395d89b41146101d15780639ab24eb0146101bd578063a217fddf146101cc578063a9059cbb146101c7578063b0ca253e146101c2578063bb4d4436146101bd578063c02ae754146101b8578063c3cda520146101b3578063d505accf146101ae578063d5391393146101a9578063d547741f146101a4578063dd62ed3e1461019f5763f1127ed81461019a575f80fd5b6111ec565b611193565b611155565b61111b565b610fc1565b610e7a565b610486565b610df7565b610672565b610e34565b610e1a565b610d52565b610d27565b610cd7565b610bfb565b610ac4565b610a8c565b610a57565b6109dc565b6109ba565b610979565b6108d0565b610784565b610615565b6105fb565b6105e0565b61059b565b610568565b6104a3565b610455565b610331565b346102d65760206003193601126102d6576004357fffffffff0000000000000000000000000000000000000000000000000000000081168091036102d657807f7965db0b00000000000000000000000000000000000000000000000000000000602092149081156102ac575b506040519015158152f35b7f01ffc9a7000000000000000000000000000000000000000000000000000000009150145f6102a1565b5f80fd5b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b90602061032e9281815201906102da565b90565b346102d6575f6003193601126102d6576040515f600354610351816112b5565b80845290600181169081156103e75750600114610389575b61038583610379818503826113f4565b6040519182918261031d565b0390f35b60035f9081527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b939250905b8082106103cd57509091508101602001610379610369565b9192600181602092548385880101520191019092916103b5565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506103799050610369565b600435906001600160a01b03821682036102d657565b602435906001600160a01b03821682036102d657565b346102d65760406003193601126102d65761047b610471610429565b6024359033611b03565b602060405160018152f35b346102d6575f6003193601126102d6576020600254604051908152f35b346102d65760606003193601126102d6576104bc610429565b6104c461043f565b604435906001600160a01b0383165f5260016020526104f73360405f20906001600160a01b03165f5260205260405f2090565b54925f198410610518575b61050c9350611499565b60405160018152602090f35b8284106105345761052f8361050c95033383611bd1565b610502565b82847ffb8f41b2000000000000000000000000000000000000000000000000000000005f523360045260245260445260645ffd5b346102d65760206003193601126102d65760206105936004355f526005602052600160405f20015490565b604051908152f35b346102d65760406003193601126102d6576105de6004356105ba61043f565b906105d96105d4825f526005602052600160405f20015490565b611667565b6116c8565b005b346102d6575f6003193601126102d657602060405160128152f35b346102d6575f6003193601126102d6576020610593611780565b346102d65760406003193601126102d65760043561063161043f565b336001600160a01b0382160361064a576105de9161189c565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102d65760406003193601126102d65761068b610429565b6001600160a01b0360243591165f52600a6020526106ac60405f209161194c565b8154905f82916005841161072c575b6106c6935084611e0c565b806106f5575050602079ffffffffffffffffffffffffffffffffffffffffffffffffffff5f5b16604051908152f35b60209161071c79ffffffffffffffffffffffffffffffffffffffffffffffffffff926119cb565b905f52825f20015460301c6106ec565b919261073781611c97565b810390811161077f576106c693855f5265ffffffffffff8260205f2001541665ffffffffffff8516105f1461076d5750916106bb565b929150610779906119d9565b906106bb565b61199e565b346102d65760406003193601126102d65761079d610429565b6024356107a86115df565b6001600160a01b03821680156108a8578115610880576107d26107cd836002546119e7565b600255565b6107ec836001600160a01b03165f525f60205260405f2090565b8054830190556040518281525f907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a36002549179ffffffffffffffffffffffffffffffffffffffffffffffffffff808411610850576105de8383612436565b7f1cb15d26000000000000000000000000000000000000000000000000000000005f52600484905260245260445ffd5b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102d6575f6003193601126102d6576108e943611c18565b65ffffffffffff806108fa43611c18565b16911603610951576103856040516109136040826113f4565b601d81527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c7400000060208201526040519182916020835260208301906102da565b7f6ff07140000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102d65760206003193601126102d6576001600160a01b0361099a610429565b165f52600960205260206001600160a01b0360405f205416604051908152f35b346102d65760206003193601126102d6576105de6109d6610429565b336119f4565b346102d65760206003193601126102d6576001600160a01b036109fd610429565b165f52600a60205260405f205463ffffffff8111610a275760405163ffffffff9091168152602090f35b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52602060045260245260445ffd5b346102d65760206003193601126102d6576020610593610a75610429565b6001600160a01b03165f525f60205260405f205490565b346102d65760206003193601126102d6576001600160a01b03610aad610429565b165f526008602052602060405f2054604051908152f35b346102d6575f6003193601126102d657610ba2610b007f0000000000000000000000000000000000000000000000000000000000000000611fc3565b610b297f000000000000000000000000000000000000000000000000000000000000000061203c565b6020604051610b3882826113f4565b5f815281610bb0818301947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe083013687376040519788977f0f00000000000000000000000000000000000000000000000000000000000000895260e0858a015260e08901906102da565b9087820360408901526102da565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b828110610be457505050500390f35b835185528695509381019392810192600101610bd5565b346102d65760206003193601126102d657610c1760043561194c565b600b54905f829160058411610c83575b610c339350600b611e0c565b80610c61575060205f5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff60405191168152f35b610c7e610c6f6020926119cb565b600b5f52825f20015460301c90565b610c3d565b9192610c8e81611c97565b810390811161077f57610c3393600b5f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14610cc5575091610c27565b929150610cd1906119d9565b90610c27565b346102d65760406003193601126102d657602060ff610d1b600435610cfa61043f565b905f526005845260405f20906001600160a01b03165f5260205260405f2090565b54166040519015158152f35b346102d6575f6003193601126102d6576020610d4243611c18565b65ffffffffffff60405191168152f35b346102d6575f6003193601126102d6576040515f600454610d72816112b5565b80845290600181169081156103e75750600114610d995761038583610379818503826113f4565b60045f9081527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b939250905b808210610ddd57509091508101602001610379610369565b919260018160209254838588010152019101909291610dc5565b346102d65760206003193601126102d6576020610593610e15610429565b611446565b346102d6575f6003193601126102d65760206040515f8152f35b346102d65760406003193601126102d65761047b610e50610429565b6024359033611499565b6064359060ff821682036102d657565b6084359060ff821682036102d657565b346102d65760c06003193601126102d657610e93610429565b60243590604435610ea2610e5a565b6084359060a43592804211610f965791610f289391610f1a610f1f9460405160208101917fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf83526001600160a01b038a1660408301528a6060830152608082015260808152610f1260a0826113f4565b519020611ab3565b612073565b90929192612137565b610f4c816001600160a01b03165f52600860205260405f2080549060018201905590565b809303610f5d576105de92506119f4565b6001600160a01b0391507f752d88c0000000000000000000000000000000000000000000000000000000005f521660045260245260445ffd5b7f4683af0e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346102d65760e06003193601126102d657610fda610429565b610fe261043f565b6044359060643592610ff2610e6a565b60a43560c435908642116110ef5761109b9261109661102b866001600160a01b03165f52600860205260405f2080549060018201905590565b9860405160208101917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983526001600160a01b0389169b8c60408401526001600160a01b038b1660608401528b608084015260a083015260c082015260c08152610f1260e0826113f4565b611af4565b936001600160a01b038516036110b5576105de9350611b03565b7f4b800e46000000000000000000000000000000000000000000000000000000005f526001600160a01b038085166004521660245260445ffd5b867f62791302000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346102d6575f6003193601126102d65760206040517f9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a68152f35b346102d65760406003193601126102d6576105de60043561117461043f565b9061118e6105d4825f526005602052600160405f20015490565b61189c565b346102d65760406003193601126102d65760206111e36111b1610429565b6001600160a01b036111c161043f565b91165f526001835260405f20906001600160a01b03165f5260205260405f2090565b54604051908152f35b346102d65760406003193601126102d657611205610429565b6024359063ffffffff821682036102d657610385916001600160a01b036112529261122e611481565b50611237611481565b50165f52600a60205260405f2061124c611481565b506121fe565b5060405190611260826113d3565b5465ffffffffffff8116825260301c602082015260405191829182919091602079ffffffffffffffffffffffffffffffffffffffffffffffffffff81604084019565ffffffffffff8151168552015116910152565b90600182811c921680156112fc575b60208310146112cf57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f16916112c4565b5f9291815491611315836112b5565b808352926001811690811561136a575060011461133157505050565b5f9081526020812093945091925b838310611350575060209250010190565b60018160209294939454838587010152019101919061133f565b905060209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040810190811067ffffffffffffffff8211176113ef57604052565b6113a6565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176113ef57604052565b604051906114446040836113f4565b565b6001600160a01b03165f52600a60205279ffffffffffffffffffffffffffffffffffffffffffffffffffff61147d60405f20611a8a565b1690565b6040519061148e826113d3565b5f6020838281520152565b9291906001600160a01b0384169384156115b3576001600160a01b0382168015611587576114d7826001600160a01b03165f525f60205260405f2090565b54848110611553579584611444969703611501846001600160a01b03165f525f60205260405f2090565b5561151c846001600160a01b03165f525f60205260405f2090565b8054860190556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a36124b9565b8490877fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd5b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b335f9081527f15a28d26fa1bf736cf7edc9922607171ccb09c3c73b808e7772a3013e068a522602052604090205460ff161561161757565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a660245260445ffd5b805f52600560205260ff61168f3360405f20906001600160a01b03165f5260205260405f2090565b5416156116995750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f52600560205260ff6116f08360405f20906001600160a01b03165f5260205260405f2090565b541661177a57805f52600560205261171c8260405f20906001600160a01b03165f5260205260405f2090565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008254161790556001600160a01b03339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016301480611873575b156117db577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a0815261186d60c0826113f4565b51902090565b507f000000000000000000000000000000000000000000000000000000000000000046146117b2565b805f52600560205260ff6118c48360405f20906001600160a01b03165f5260205260405f2090565b54161561177a57805f5260056020526118f18260405f20906001600160a01b03165f5260205260405f2090565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690556001600160a01b03339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b65ffffffffffff61195c43611c18565b168082101561196f575061032e90611c18565b907fecd3f81e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b905f19820191821161077f57565b906001820180921161077f57565b9190820180921161077f57565b6001600160a01b038181165f81815260096020526040812080548685167fffffffffffffffffffffffff00000000000000000000000000000000000000008216811790925561144496941694611a849390928691907f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9080a46001600160a01b03165f525f60205260405f205490565b91611e70565b805480611a975750505f90565b805f1981011161077f575f19915f5260205f2001015460301c90565b604290611abe611780565b90604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b9161032e9391610f1f93612073565b6001600160a01b0316908115611ba5576001600160a01b038116928315611b795780611b6c7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593855f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55604051908152602090a3565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b0316908115611ba5576001600160a01b03811615611b7957611c15915f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55565b65ffffffffffff8111611c305765ffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52603060045260245260445ffd5b8115611c6a570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b600181111561032e57806001700100000000000000000000000000000000831015611dca575b611d70611d66611d5c611d52611d48611d3e611d2d611d779760048a68010000000000000000611d7c9c1015611dbd575b640100000000811015611db0575b62010000811015611da3575b610100811015611d96575b6010811015611d89575b1015611d81575b60030260011c90565b611d37818b611c60565b0160011c90565b611d37818a611c60565b611d378189611c60565b611d378188611c60565b611d378187611c60565b611d378186611c60565b8093611c60565b821190565b900390565b60011b611d24565b60041c9160021b91611d1d565b60081c9160041b91611d13565b60101c9160081b91611d08565b60201c9160101b91611cfc565b60401c9160201b91611cee565b5050611d7c611d77611d70611d66611d5c611d52611d48611d3e611d2d611df18a60801c90565b9850680100000000000000009750611cbd9650505050505050565b91905b838210611e1c5750505090565b9091928083169080841860011c820180921161077f57845f5265ffffffffffff8260205f2001541665ffffffffffff8416105f14611e5e5750925b9190611e0f565b939250611e6a906119d9565b91611e57565b91906001600160a01b038116926001600160a01b038116908482141580611fba575b611e9e575b5050505050565b81611f44575b505082611eb3575b8080611e97565b611f39611f207fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72493611f1a611f1479ffffffffffffffffffffffffffffffffffffffffffffffffffff956001600160a01b03165f52600a60205260405f2090565b91612240565b90612314565b6040805192851683529316602082015291829190820190565b0390a25f8080611eac565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff611fb0611f20611fa17fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724946001600160a01b03165f52600a60205260405f2090565b611faa88612240565b906122b0565b0390a25f80611ea4565b50831515611e92565b60ff81146120225760ff811690601f8211611ffa5760405191611fe76040846113f4565b6020808452838101919036833783525290565b7fb3512b0c000000000000000000000000000000000000000000000000000000005f5260045ffd5b5060405161032e81612035816006611306565b03826113f4565b60ff81146120605760ff811690601f8211611ffa5760405191611fe76040846113f4565b5060405161032e81612035816007611306565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116120f5579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa156120ea575f516001600160a01b038116156120e057905f905f90565b505f906001905f90565b6040513d5f823e3d90fd5b5050505f9160039190565b6004111561210a57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b61214081612100565b80612149575050565b61215281612100565b60018103612182577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b61218b81612100565b600281036121bf57507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b806121cb600392612100565b146121d35750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b8054821015612213575f5260205f2001905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff81116122805779ffffffffffffffffffffffffffffffffffffffffffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f5260d060045260245260445ffd5b906122ba43611c18565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff806122e085611a8a565b92169116039079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161077f57612310926125c4565b9091565b9061231e43611c18565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff8061234485611a8a565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161077f57612310926125c4565b61237d43611c18565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff806123a4600b611a8a565b921691160179ffffffffffffffffffffffffffffffffffffffffffffffffffff811161077f5761231091600b6125c4565b6123de43611c18565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80612405600b611a8a565b921691160379ffffffffffffffffffffffffffffffffffffffffffffffffffff811161077f5761231091600b6125c4565b906001600160a01b036114449261245461244f84612240565b612374565b50501680156124a1575b60096020527fec8156718a8372b1db44bb411437d0870f3e3790d4a08526d024ce1b0b668f6b545f9182526040909120546001600160a01b039081169116611e70565b6124b26124ad83612240565b6123d5565b505061245e565b906001600160a01b038061144494931691821561251e575b1690811561250b575b5f5260096020526001600160a01b0360405f205416905f5260096020526001600160a01b0360405f20541690611e70565b6125176124ad84612240565b50506124da565b61252a61244f85612240565b50506124d1565b8054680100000000000000008110156113ef57612553916001820181556121fe565b6125985781516020929092015160301b7fffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000001665ffffffffffff92909216919091179055565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b805492939280156126ba576125db6125e6916119cb565b825f5260205f200190565b8054603081901c9365ffffffffffff918216929181168084116126925787930361264b575061264792509065ffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000083549260301b169116179055565b9190565b9150506126479161266b61265d611435565b65ffffffffffff9093168352565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff86166020830152612531565b7f2520601d000000000000000000000000000000000000000000000000000000005f5260045ffd5b50906126f2916126cb61265d611435565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff85166020830152612531565b5f9190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d15a28d26fa1bf736cf7edc9922607171ccb09c3c73b808e7772a3013e068a52205b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x04\x81 T\x14a\\\x9CWP\x80c\x06B\xDD\xE5\x14a\\%W\x80c\x07Tar\x14a[\xFEW\x80c\n\x92T\xE4\x14aTyW\x80c\x10t\xA2\x1F\x14aS=W\x80c\x12:J_\x14aQ\xB8W\x80c\x1E\xD7\x83\x1C\x14aQ:W\x80c\"F\xE5\xCC\x14aL\x13W\x80c*\xDE8\x80\x14aJ\x1FW\x80c>^<#\x14aI\xA1W\x80c?r\x86\xF4\x14aI#W\x80cAho\xF2\x14aB6W\x80cGH<]\x14a=QW\x80cO\x862\xBA\x14a=*W\x80cU\xF7\xD4w\x14a:\x80W\x80c[\xB1w\x81\x14a5KW\x80cc8\xAA\x86\x14a2\xC1W\x80cd\x0FrZ\x14a0(W\x80cf\xD9\xA9\xA0\x14a.\xEBW\x80cq\xD7\xDA\xBF\x14a+HW\x80ctj\x9B\xCF\x14a)\xC7W\x80cv\x02\x9Ex\x14a&\xCFW\x80c\x84\xEF\x8F\xFC\x14a&\xA9W\x80c\x85\"l\x81\x14a&\x1FW\x80c\x88\xC5g\x1B\x14a\"IW\x80c\x8D1\xEDS\x14a\x1EsW\x80c\x8F\x08\xEC\xE7\x14a\x1B{W\x80c\x8F1\r\xFE\x14a\x17\x82W\x80c\x91j\x17\xC6\x14a\x16\xD8W\x80c\x91\xDC\x0B-\x14a\x13RW\x80c\xB0FO\xDC\x14a\x12\xA8W\x80c\xB5P\x8A\xA9\x14a\x12\x1EW\x80c\xB7\x8BYg\x14a\r\x1BW\x80c\xBAAO\xA6\x14a\x0C\xF6W\x80c\xBB\xB1U3\x14a\x07\x9FW\x80c\xC0\x1E\x94(\x14a\x04\xF8W\x80c\xDC\xCCW\xF1\x14a\x02\xE5W\x80c\xE2\x0C\x9Fq\x14a\x02WW\x80c\xE8\xED\xC8\x16\x14a\x020W\x80c\xFAv&\xD4\x14a\x02\rWc\xFC\x0CTj\x14a\x01\xE1W_\x80\xFD[4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[\x80\xFD[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x02\xC6Wa\x02\xC2\x85a\x02\xB6\x81\x87\x03\x82a_\xBDV[`@Q\x91\x82\x91\x82a]\xA4V[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x02\x9FV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x04\xC5W[P` \x80T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R\x81`D\x81\x85Z\xFA\x80\x15a\x04\xBAWa\x03\xA1\x91\x84\x91a\x04HW[Paj\nV[`@Q\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x04\x82W[P`!T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01[\x03\x91Z\xFA\x80\x15a\x04wWa\x04E\x91\x83\x91a\x04HWPaj\nV[\x80\xF3[a\x04j\x91P` =` \x11a\x04pW[a\x04b\x81\x83a_\xBDV[\x81\x01\x90aapV[_a\x03\x9BV[P=a\x04XV[`@Q=\x84\x82>=\x90\xFD[\x90P` \x81=` \x11a\x04\xB2W[\x81a\x04\x9D` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQa\x04+a\x03\xDEV[_\x80\xFD[=\x91Pa\x04\x90V[`@Q=\x85\x82>=\x90\xFD[\x90P` \x81=` \x11a\x04\xF0W[\x81a\x04\xE0` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ` a\x03BV[=\x91Pa\x04\xD3V[P4a\x02\nW`@`\x03\x196\x01\x12a\x02\nW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x07mWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07mW\x82`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x07\x8AW[Pa\x05\x9C`$5ah\xC6V[\x91`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07mW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04\xBAW\x83\x91a\x07qW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07mW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04wWa\x07XW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x07MW\x84\x91a\x07\x1AW[P`\x04\x92a\x06\xA3\x83` \x93ahPV[`@Q\x93\x84\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x04\xBAW\x83\x90a\x06\xE6W[a\x04E\x92PahPV[P` \x82=` \x11a\x07\x12W[\x81a\x07\0` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x91Qa\x06\xDCV[=\x91Pa\x06\xF3V[\x90P` \x81=` \x11a\x07EW[\x81a\x075` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ`\x04a\x06\x93V[=\x91Pa\x07(V[`@Q=\x86\x82>=\x90\xFD[\x81a\x07b\x91a_\xBDV[a\x07mW\x82_a\x06WV[\x82\x80\xFD[\x81a\x07{\x91a_\xBDV[a\x07\x86W\x81_a\x06\x02V[P\x80\xFD[\x81a\x07\x94\x91a_\xBDV[a\x07mW\x82_a\x05\x90V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rb\n\x11\xCE`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x04wW\x82\x91a\x0C\xC7W[Pa\x0E\x10B\x01\x90\x81B\x11a\x0BWW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91\x83`@Q\x93\x7F~\xCE\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92\x83`\x04\x87\x01R` \x86`$\x81\x85Z\xFA\x95\x86\x15a\x04\xBAW\x83\x96a\x0C\x90W[P`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q` \x81\x01\x90a\t\x04\x81a\x08\xF6\x85\x8C\x88\x8D\x88\x90\x94\x93\x92`\x01`\x01`\xA0\x1B\x03`\xA0\x93\x81`\xC0\x85\x01\x98\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x86R\x16` \x85\x01R\x16`@\x83\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0``\x83\x01R`\x80\x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a_\xBDV[Q\x90 `@Q\x90\x7F6D\xE5\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x88Z\xFA\x91\x82\x15a\x0C\x85W\x86\x92a\x0CLW[P`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01\x90\x81R`\"\x82\x01\x93\x90\x93R`B\x81\x01\x91\x90\x91Ra\t\x8E\x81`b\x81\x01a\x08\xF6V[Q\x90 \x92`@Q\x93\x7F\xE3A\xEA\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85Rb\n\x11\xCE`\x04\x86\x01R`$\x85\x01R``\x84`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x92\x83\x15a\x0B\xECW\x85\x94\x86\x90\x87\x95a\x0C\x14W[P\x82;\x15a\x0C\x10W\x88a\n}\x88\x96\x87\x93`@Q\x99\x8A\x98\x89\x97\x88\x96\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R`\x04\x88\x01\x92\x93`\xC0\x94`\x01`\x01`\xA0\x1B\x03`\xFF\x93\x99\x98\x97\x94\x81`\xE0\x88\x01\x9B\x16\x87R\x16` \x86\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`@\x86\x01R``\x85\x01R\x16`\x80\x83\x01R`\xA0\x82\x01R\x01RV[\x03\x92Z\xF1\x80\x15a\x04wWa\x0B\xF7W[PP`\x1FT`#T`@Q\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R`\x08\x91\x90\x91\x1C\x90\x92\x16\x91\x90` \x82`D\x81\x86Z\xFA\x91\x82\x15a\x0B\xECW\x85\x92a\x0B\xB6W[Pa\x0B\0` \x92afTV[`$`@Q\x80\x94\x81\x93\x7F~\xCE\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x0B\x84W[P`\x01\x82\x01\x80\x92\x11a\x0BWW\x90a\x04E\x91ahPV[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x0B\xAEW[\x81a\x0B\x9F` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ_a\x0BAV[=\x91Pa\x0B\x92V[\x91P` \x82=` \x11a\x0B\xE4W[\x81a\x0B\xD1` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x90a\x0B\0a\n\xF4V[=\x91Pa\x0B\xC4V[`@Q=\x87\x82>=\x90\xFD[\x81a\x0C\x01\x91a_\xBDV[a\x0C\x0CW\x83_a\n\x8CV[\x83\x80\xFD[\x86\x80\xFD[\x91\x95PPa\x0C;\x91\x93P``=``\x11a\x0CEW[a\x0C3\x81\x83a_\xBDV[\x81\x01\x90ad\xD9V[\x93\x91\x94\x90_a\t\xF6V[P=a\x0C)V[\x95P\x90P` \x85=` \x11a\x0C}W[\x81a\x0Ci` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x93Q\x88\x94a\t\x8Ea\tEV[=\x91Pa\x0C\\V[`@Q=\x88\x82>=\x90\xFD[\x92P\x94P` \x82=` \x11a\x0C\xBFW[\x81a\x0C\xAD` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x85\x91Q\x94_a\x08xV[=\x91Pa\x0C\xA0V[a\x0C\xE9\x91P` =` \x11a\x0C\xEFW[a\x0C\xE1\x81\x83a_\xBDV[\x81\x01\x90ad\xBAV[_a\x08\nV[P=a\x0C\xD7V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` a\r\x11ad\xFBV[`@Q\x90\x15\x15\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Rb\n\x11\xCE`\x04\x83\x01R` \x82`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x91\x82\x15a\x12\x11W\x81\x92a\x11\xF0W[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x01\x91B\x83\x11a\x11\xC3W\x81\x92`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F~\xCE\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x0B\xECW\x85\x91a\x11\x8BW[P\x90` `\x04\x92\x84a\x0E\x93`\x01`\x01`\xA0\x1B\x03`#T\x16\x92a\x08\xF6`@Q\x93\x84\x92\x87\x84\x01\x96\x8C\x88\x90\x94\x93\x92`\x01`\x01`\xA0\x1B\x03`\xA0\x93\x81`\xC0\x85\x01\x98\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x86R\x16` \x85\x01R\x16`@\x83\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0``\x83\x01R`\x80\x82\x01R\x01RV[Q\x90 \x91`@Q\x93\x84\x80\x92\x7F6D\xE5\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x0B\xECW\x85\x92a\x11RW[P`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01\x90\x81R`\"\x82\x01\x93\x90\x93R`B\x81\x01\x91\x90\x91Ra\x0F\x1A\x81`b\x81\x01a\x08\xF6V[Q\x90 \x90`@Q\x91\x7F\xE3A\xEA\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rb\n\x11\xCE`\x04\x84\x01R`$\x83\x01R``\x82`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x07MW\x84\x93\x85\x93\x86\x92a\x11(W[P`@Q\x7Fby\x13\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x83`$\x82\x01R`$\x81Ra\x0F\xC0`D\x82a_\xBDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\x10W\x86a\x10\x1B\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x11\x1DW\x87\x91a\x11\x04W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91`\x01`\x01`\xA0\x1B\x03`#T\x16\x91\x83;\x15a\x11\0W`@Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x16`$\x83\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`D\x83\x01R`d\x82\x01\x93\x90\x93R`\xFF\x94\x90\x94\x16`\x84\x85\x01R`\xA4\x84\x01\x92\x90\x92R`\xC4\x83\x01R\x82\x90\x82\x90`\xE4\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04wWa\x10\xEFWP\xF3[\x81a\x10\xF9\x91a_\xBDV[a\x02\nW\x80\xF3[\x87\x80\xFD[\x81a\x11\x0E\x91a_\xBDV[a\x11\x19W\x85_a\x10CV[\x85\x80\xFD[`@Q=\x89\x82>=\x90\xFD[\x91P\x93Pa\x11F\x91\x92P``=``\x11a\x0CEWa\x0C3\x81\x83a_\xBDV[\x92\x91\x93\x90\x92\x90_a\x0F\x81V[\x94P\x90P` \x84=` \x11a\x11\x83W[\x81a\x11o` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x92Q\x84\x93a\x0F\x1Aa\x0E\xD1V[=\x91Pa\x11bV[\x91\x94PP` \x81=` \x11a\x11\xBBW[\x81a\x11\xA8` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ\x84\x93\x90` a\x0E\x10V[=\x91Pa\x11\x9BV[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[a\x12\n\x91\x92P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[\x90_a\r\x87V[P`@Q\x90=\x90\x82>=\x90\xFD[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x19Ta\x12;\x81a_\xE0V[\x91a\x12I`@Q\x93\x84a_\xBDV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x12\x8BW`@Q\x80a\x02\xC2\x87\x82a^`V[`\x01` \x81\x92a\x12\x9A\x85a_\xF8V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x12vV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1CTa\x12\xC5\x81a_\xE0V[\x91a\x12\xD3`@Q\x93\x84a_\xBDV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x13\x15W`@Q\x80a\x02\xC2\x87\x82a^\xDDV[`\x02` `\x01\x92`@Qa\x13(\x81a_tV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x13@\x85\x87\x01aa\x95V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x13\0V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x16\xC3W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x16\xAEW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x16\x96W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x16&W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\\\x19\xA9\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x04wWa\x16\x81W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q\x7F\x9A\xB2N\xB0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x07MW\x84\x91a\x16IW[Pa\x15U\x90afTV[` `\x01`\x01`\xA0\x1B\x03`\"T\x16`$`@Q\x80\x94\x81\x93\x7FX|\xDE\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x16*W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16&W`@Q\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x10\xEFWP\xF3[PP\xFD[a\x16C\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_a\x15\xA4V[\x93PP` \x83=` \x11a\x16yW[\x81a\x16e` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x15U\x84\x93Q\x90a\x15KV[=\x91Pa\x16XV[\x81a\x16\x8B\x91a_\xBDV[a\x02\nW\x80_a\x14\xEBV[\x81a\x16\xA0\x91a_\xBDV[a\x02\nW\x80_a\x14\x85V[P\xFD[\x81a\x16\xB8\x91a_\xBDV[a\x02\nW\x80_a\x14\"V[\x81a\x16\xCD\x91a_\xBDV[a\x02\nW\x80_a\x13\xC6V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1DTa\x16\xF5\x81a_\xE0V[\x91a\x17\x03`@Q\x93\x84a_\xBDV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x17EW`@Q\x80a\x02\xC2\x87\x82a^\xDDV[`\x02` `\x01\x92`@Qa\x17X\x81a_tV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x17p\x85\x87\x01aa\x95V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x170V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x1BfW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x1BQW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh\x10CV\x1A\x88)0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x1B<W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x1B'W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x1A\xEFW[Pa\x19\x83\x90ag\xD1V[`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\x1A\xB7W[P`\x04\x91a\x19\xC8` \x92agRV[`@Q\x92\x83\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04wW\x82\x91a\x1A\x82W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh+^:\xF1k\x18\x80\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x10\xEFWP\xF3[\x91PP` \x81=` \x11a\x1A\xAFW[\x81a\x1A\x9E` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x81\x90Q_a\x1A\x02V[=\x91Pa\x1A\x91V[\x92PP` \x82=` \x11a\x1A\xE7W[\x81a\x1A\xD3` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x82\x91\x90`\x04a\x19\xB9V[=\x91Pa\x1A\xC6V[\x92PP` \x82=` \x11a\x1B\x1FW[\x81a\x1B\x0B` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x19\x83\x83\x92Q\x90a\x19yV[=\x91Pa\x1A\xFEV[\x81a\x1B1\x91a_\xBDV[a\x02\nW\x80_a\x193V[\x81a\x1BF\x91a_\xBDV[a\x02\nW\x80_a\x18\xC7V[\x81a\x1B[\x91a_\xBDV[a\x02\nW\x80_a\x18kV[\x81a\x1Bp\x91a_\xBDV[a\x02\nW\x80_a\x18\x0FV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x1E^W[P`\x04`\x01`\x01`\xA0\x1B\x03`\"T\x16` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x04\xBAW\x83\x92a\x1E%W[P`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Ra\x1C\xB1\x81`d\x81\x01a\x08\xF6V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW\x81a\x1D\x0C\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x1E\x10W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x1D\xFBW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa\x10\xEFWP\xF3[\x81a\x1E\x05\x91a_\xBDV[a\x02\nW\x80_a\x1D\x8DV[\x81a\x1E\x1A\x91a_\xBDV[a\x02\nW\x80_a\x1D1V[\x92P\x90P` \x82=` \x11a\x1EVW[\x81a\x1EB` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x82\x91a\x1C\xB1a\x1CbV[=\x91Pa\x1E5V[\x81a\x1Eh\x91a_\xBDV[a\x02\nW\x80_a\x1C\x08V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a\"\x11W[P` \x80T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01\x81\x90R\x92\x82\x90`D\x90\x82\x90Z\xFA\x80\x15a\x04\xBAWa\x1F4\x91\x84\x91a\x04HWPaj\nV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa!\xFCW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a!\xC7W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16\x82;\x15a!\xC2W`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04wWa!\xADW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa!\x98W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a!dW[P`\"T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01a\x04+V[\x90P` \x81=` \x11a!\x90W[\x81a!\x7F` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQa\x04+a!\x13V[=\x91Pa!rV[\x81a!\xA2\x91a_\xBDV[a\x02\nW\x80_a \xC5V[\x81a!\xB7\x91a_\xBDV[a\x02\nW\x80_a YV[PPP\xFD[\x92PP` \x82=` \x11a!\xF4W[\x81a!\xE3` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x82\x91Q_a\x1F\xF0V[=\x91Pa!\xD6V[\x81a\"\x06\x91a_\xBDV[a\x02\nW\x80_a\x1F\xA3V[\x92PP` \x82=` \x11a\"AW[\x81a\"-` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x82\x91\x90` a\x1E\xD1V[=\x91Pa\" V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa&\nW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa%\xF5W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a#\x84`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa%\xE0W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF` `@Qh\x10CV\x1A\x88)0\0\0\x81R\xA3sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa%\xCBW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rh\x10CV\x1A\x88)0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa%\xAEW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x80\x15a\x04\xBAW\x83\x90a%zW[a%\x07\x91Paf\xD3V[` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x94\x81\x93cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x04wW\x82\x90a%FW[a\x04E\x91PagRV[P` \x81=` \x11a%rW[\x81a%`` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x90Qa%<V[=\x91Pa%SV[P` \x81=` \x11a%\xA6W[\x81a%\x94` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa%\x07\x90Qa$\xFDV[=\x91Pa%\x87V[a%\xC6\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a$\xB8V[\x81a%\xD5\x91a_\xBDV[a\x02\nW\x80_a$LV[\x81a%\xEA\x91a_\xBDV[a\x02\nW\x80_a#\xA9V[\x81a%\xFF\x91a_\xBDV[a\x02\nW\x80_a#\x19V[\x81a&\x14\x91a_\xBDV[a\x02\nW\x80_a\"\xBDV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1ATa&<\x81a_\xE0V[\x91a&J`@Q\x93\x84a_\xBDV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a&\x8CW`@Q\x80a\x02\xC2\x87\x82a^`V[`\x01` \x81\x92a&\x9B\x85a_\xF8V[\x81R\x01\x92\x01\x92\x01\x91\x90a&wV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x06\xFD\xDE\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a)\xADW[Pa'n`@\x91\x82Q\x90a'?\x84\x83a_\xBDV[`\x11\x82R\x7FTestnet Syndicate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RaisV[\x80Q\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83\x81`\x04\x81\x86Z\xFA\x90\x81\x15a)\xA3W\x90a'\xEB\x91\x85\x91a)\x81W[P\x82Q\x90a'\xBC\x84\x83a_\xBDV[`\x0B\x82R\x7FTestnetSYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RaisV[\x82\x81Q\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA\x90\x81\x15a)=W\x82\x91a)GW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W`\xFF\x83Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01R`\x12`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a)=Wa)(W[PP` `\x04\x92\x82Q\x93\x84\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a)\x1FWP\x82\x90a(\xEBW[a\x04E\x91Pae\xD4V[P` \x81=` \x11a)\x17W[\x81a)\x05` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x90Qa(\xE1V[=\x91Pa(\xF8V[Q=\x84\x82>=\x90\xFD[\x81a)2\x91a_\xBDV[a\x07mW\x82_a(\xA0V[\x83Q=\x84\x82>=\x90\xFD[\x90P` \x81=` \x11a)yW[\x81a)b` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x07\x86Wa)s\x90ad\xACV[_a((V[=\x91Pa)UV[a)\x9D\x91P=\x80\x87\x83>a)\x95\x81\x83a_\xBDV[\x81\x01\x90ad9V[_a'\xAEV[\x82Q=\x86\x82>=\x90\xFD[a)\xC1\x91P=\x80\x85\x83>a)\x95\x81\x83a_\xBDV[_a'+V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa+3W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa+\x1EW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x16\xABW\x81\x80\x91`D`@Q\x80\x94\x81\x93c@\xC1\x0F\x19`\xE0\x1B\x83R\x81`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x10\xEFWP\xF3[\x81a+(\x91a_\xBDV[a\x02\nW\x80_a*\xCDV[\x81a+=\x91a_\xBDV[a\x02\nW\x80_a*;V[P4a\x02\nW`@`\x03\x196\x01\x12a\x02\nWa+e`\x045ah\xC6V[a+s\x81`\x01`$5aj\xB3V[`@\x83\x80\x82Qa+\x83\x84\x82a_\xBDV[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x83Qa,\x02\x81a+\xEE` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x88`$\x84\x01R`d\x83\x01\x90a]\xE6V[\x88`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a_\xBDV[Q\x90jconsole.logZ\xFAP\x83`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x82Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a)=Wa.\xD6W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90\x80;\x15a\x07mW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a)=Wa.\xC1W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x82Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a)=Wa.\xACW[PP`\x1FT`#T\x82Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91` \x91\x83\x91`\x08\x1C\x16\x81\x88\x81`D\x81\x01\x03\x92Z\xF1\x80\x15a.\xA2Wa.\x85W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x92`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90\x82Q\x91cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01R` \x82`$\x81\x88Z\xFA\x91\x82\x15a.{W\x90\x84\x92\x91\x87\x92a.BW[Pa-\xFA\x92a-\xF4\x91aa\x88V[\x90ahPV[` `\x01`\x01`\xA0\x1B\x03`#T\x16`$\x83Q\x80\x96\x81\x93cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a.9WP\x83\x90a\x06\xE6Wa\x04E\x92PahPV[Q=\x85\x82>=\x90\xFD[\x92P\x90P` \x82=` \x11a.sW[\x81a._` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x83\x91a-\xFAa-\xE6V[=\x91Pa.RV[\x83Q=\x88\x82>=\x90\xFD[a.\x9D\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a-\x9BV[\x82Q=\x87\x82>=\x90\xFD[\x81a.\xB6\x91a_\xBDV[a\x0C\x0CW\x83_a-7V[\x81a.\xCB\x91a_\xBDV[a\x0C\x0CW\x83_a,\xD5V[\x81a.\xE0\x91a_\xBDV[a\x0C\x0CW\x83_a,uV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1BTa/\x08\x81a_\xE0V[a/\x15`@Q\x91\x82a_\xBDV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a/\xEDW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a/\x82WPPPP\x03\x90\xF3[\x91\x93` a/\xDD\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a/\xCD\x83Q`@\x84R`@\x84\x01\x90a]\xE6V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra^\x0BV[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a/sV[`\x02` `\x01\x92`@Qa0\0\x81a_tV[a0\t\x86a_\xF8V[\x81Ra0\x16\x85\x87\x01aa\x95V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a/EV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a0\xA2`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa2\xACW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` `@Qh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0\x81R\xA3sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa2\x97W[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa2zW[P`\x1FT`\"T`#T`@Q\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R\x91` \x91\x83\x91`D\x91\x83\x91`\x08\x1C\x16Z\xFA\x80\x15a\x04wW\x82\x90a2FW[a\x04E\x91Pag\xD1V[P` \x81=` \x11a2rW[\x81a2`` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x90Qa2<V[=\x91Pa2SV[a2\x92\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a1\xD6V[\x81a2\xA1\x91a_\xBDV[a\x02\nW\x80_a1jV[\x81a2\xB6\x91a_\xBDV[a\x02\nW\x80_a0\xC7V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa56W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa5!W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa5\x0CW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\\\x19\xA9\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x04wWa4\xF7W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`$`@Q\x80\x94\x81\x93\x7F\xBBMD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x04wW\x82\x90a4\xC3W[a\x04E\x91PafTV[P` \x81=` \x11a4\xEFW[\x81a4\xDD` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa\x04E\x90Qa4\xB9V[=\x91Pa4\xD0V[\x81a5\x01\x91a_\xBDV[a\x02\nW\x80_a4ZV[\x81a5\x16\x91a_\xBDV[a\x02\nW\x80_a3\xF4V[\x81a5+\x91a_\xBDV[a\x02\nW\x80_a3\x91V[\x81a5@\x91a_\xBDV[a\x02\nW\x80_a35V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa:kW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa:VW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa:AW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa:$W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa:\x0FW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`d`\x01`\x01`\xA0\x1B\x03`#T\x16\x91\x85`@Q\x95\x86\x94\x85\x93\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01R`$\x84\x01Rh\x10CV\x1A\x88)0\0\0`D\x84\x01RZ\xF1\x80\x15a\x04wWa9\xF2W[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Qcp\xA0\x821`\xE0\x1B\x81R\x82`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x07MW\x84\x91a9\xBAW[Pa8 \x90af\xD3V[`\x01`\x01`\xA0\x1B\x03`#T\x16\x91`@Qcp\xA0\x821`\xE0\x1B\x81R\x83`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x0B\xECW\x85\x91a9}W[P\x91a8\xB6\x93\x91a8i` \x94agRV[`@Q\x80\x95\x81\x94\x82\x93\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91\x81`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x04wW\x82\x91a9HW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh\n\xD7\x8E\xBCZ\xC6 \0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x10\xEFWP\xF3[\x91PP` \x81=` \x11a9uW[\x81a9d` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x81\x90Q_a8\xC8V[=\x91Pa9WV[\x92\x94PP\x91` \x82=` \x11a9\xB2W[\x81a9\x9B` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x84\x93\x91\x92\x90a8\xB6a8WV[=\x91Pa9\x8EV[\x93PP` \x83=` \x11a9\xEAW[\x81a9\xD6` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWa8 \x84\x93Q\x90a8\x16V[=\x91Pa9\xC9V[a:\n\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a7\xCEV[\x81a:\x19\x91a_\xBDV[a\x02\nW\x80_a7NV[a:<\x90` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[a6\xEAV[\x81a:K\x91a_\xBDV[a\x02\nW\x80_a6~V[\x81a:`\x91a_\xBDV[a\x02\nW\x80_a6\x1BV[\x81a:u\x91a_\xBDV[a\x02\nW\x80_a5\xBFV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a:\xFA`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa=\x15W[PP`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF` `@Qh65\xC9\xAD\xC5\xDE\xA0\0\0\x81R\xA3\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWa=\0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWa<\xEBW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91a<\xB8W[P`\x04\x91a<v` \x92afTV[`@Q\x92\x83\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x04wW\x82\x90a4\xC3Wa\x04E\x91PafTV[\x90P` \x81=` \x11a<\xE3W[\x81a<\xD3` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ`\x04a<gV[=\x91Pa<\xC6V[\x81a<\xF5\x91a_\xBDV[a\x02\nW\x80_a< V[\x81a=\n\x91a_\xBDV[a\x02\nW\x80_a;\xC4V[\x81a=\x1F\x91a_\xBDV[a\x02\nW\x80_a;\x1FV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R`\x01`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaB!W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04wW\x82\x91aA\xECW[P`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x80\x91`@Q\x93\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x86\x80\xA4sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16&W\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaA\xD7W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91aA\xA2W[P`\x01`\x01`\xA0\x1B\x03`!T\x16\x82;\x15a!\xC2W`@Q\x7F\xD5Gt\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04wWaA\x8DW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaAxW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x04\xBAW\x83\x92aA@W[P`!T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R` \x90\x82\x90`D\x90\x82\x90Z\xFA\x90\x81\x15a\x04wW\x82\x91aA!W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x04wWa\x10\xEFWP\xF3[aA:\x91P` =` \x11a\x04pWa\x04b\x81\x83a_\xBDV[_a@\xAEV[\x92P\x90P` \x82=` \x11aApW[\x81aA]` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x90Q\x82\x91` a@TV[=\x91PaAPV[\x81aA\x82\x91a_\xBDV[a\x02\nW\x80_a@\x06V[\x81aA\x97\x91a_\xBDV[a\x02\nW\x80_a?\x9AV[\x92PP` \x82=` \x11aA\xCFW[\x81aA\xBE` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x82\x91Q_a?1V[=\x91PaA\xB1V[\x81aA\xE1\x91a_\xBDV[a\x02\nW\x80_a>\xE4V[\x91PP` \x81=` \x11aB\x19W[\x81aB\x08` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x81\x90Q_a>6V[=\x91PaA\xFBV[\x81aB+\x91a_\xBDV[a\x02\nW\x80_a=\xE8V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaI\x0EW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWaH\xF9W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rhlk\x93[\x8B\xBD@\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWaH\xE4W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16aC\x93a`\xFBV[`@Q` \x81\x01\x90aC\xBF` \x82\x85Q\x80\x83\x88\x01\x87^\x81\x01\x88\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a_\xBDV[Q\x90 `@Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x07MW\x84\x91aH\xC5W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a!\xC2W\x83`\x01`\x01`\xA0\x1B\x03aD\x86\x92`@Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x95\x86`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x07MW\x84\x91aH\xB0W[PP\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01Rh\xA2\xA1]\tQ\x9B\xE0\0\0`$\x84\x01RZ\xF1\x80\x15a\x04wWaH\x9BW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaH\x86W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x04\xBAW\x83\x91aHTW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90cp\xA0\x821`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x07MW\x84\x91aH\"W[P`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q\x91cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01R` \x82`$\x81\x87Z\xFA\x90\x81\x15a\x0B\xECW\x85\x91aG\xECW[aF#\x92Paa6V[\x91aF,a`\xFBV[\x84`@Q` \x81\x01\x90aFY` \x82\x86Q\x80\x83\x89\x01\x87^\x81\x01\x86\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a_\xBDV[Q\x90 `@Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x04wW\x82\x91aG\xCDW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aG \x92`@Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaG\xB4W[PP` \x90`$`@Q\x80\x94\x81\x93cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07MW\x84\x91aG\x80W[Pa-\xF4\x90a\x04E\x93aa6V[\x90P` \x81=` \x11aG\xACW[\x81aG\x9B` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQa\x04EaGrV[=\x91PaG\x8EV[\x81aG\xBE\x91a_\xBDV[aG\xC9W\x84_aGEV[\x84\x80\xFD[aG\xE6\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aF\xB3V[\x90P` \x82=` \x11aH\x1AW[\x81aH\x07` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWaF#\x91Q\x90aF\x19V[=\x91PaG\xFAV[\x90P` \x81=` \x11aHLW[\x81aH=` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ_aE\xE1V[=\x91PaH0V[\x90P` \x81=` \x11aH~W[\x81aHo` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ_aE\xAAV[=\x91PaHbV[\x81aH\x90\x91a_\xBDV[a\x02\nW\x80_aE\\V[\x81aH\xA5\x91a_\xBDV[a\x02\nW\x80_aD\xF0V[\x81aH\xBA\x91a_\xBDV[a\x16&W\x82_aD\xAEV[aH\xDE\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aD\x19V[\x81aH\xEE\x91a_\xBDV[a\x02\nW\x80_aC{V[\x81aI\x03\x91a_\xBDV[a\x02\nW\x80_aC\x1FV[\x81aI\x18\x91a_\xBDV[a\x02\nW\x80_aB\xC3V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10aI\x82Wa\x02\xC2\x85a\x02\xB6\x81\x87\x03\x82a_\xBDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aIkV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10aJ\0Wa\x02\xC2\x85a\x02\xB6\x81\x87\x03\x82a_\xBDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aI\xE9V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x1ETaJ<\x81a_\xE0V[aJI`@Q\x91\x82a_\xBDV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10aK\x8AW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10aJ\xB5W\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10aKAWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93aJ\xA8V[\x90\x91\x92\x93\x94` \x80aK}\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa]\xE6V[\x97\x01\x95\x01\x93\x92\x91\x01aK\x1DV[`@QaK\x96\x81a_tV[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80TaK\xB2\x81a_\xE0V[\x91aK\xC0`@Q\x93\x84a_\xBDV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10aK\xF6WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90aJyV[`\x01` \x81\x92aL\x05\x86a_\xF8V[\x81R\x01\x93\x01\x91\x01\x90\x91aK\xD0V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@\x81\x81Q\x91aL4\x81\x84a_\xBDV[`\x0C\x83R` \x83\x01\x7FwrongAddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81Q`\x0C` \x82\x01\x92\x83^\x83`,\x82\x01R`\x0C\x81RaL\x7F`,\x82a_\xBDV[Q\x90 \x81Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aP<W\x83\x91aQ\x1BW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07mW\x81Q\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83\x81\x80aM@`\x01`\x01`\xA0\x1B\x03\x86\x16\x98\x89`\x04\x84\x01R\x87`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aP]W\x90\x84\x91aQ\x06W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x82Q\x91\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` \x83`\x04\x81\x84Z\xFA\x92\x83\x15aP\xFCW\x85\x93aP\xC0W[P\x94` \x84\x95\x96`\x04\x95Q\x95\x86\x80\x92\x7F\xD59\x13\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x93\x84\x15aP\x7FW\x86\x94aP\x89W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x11\x19W\x84Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x85\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aP\x7FW\x90\x86\x91aPjW[PP\x83Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x81\x01\x91\x90\x91RaN\xBA\x81`d\x81\x01a\x08\xF6V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a!\xC2W\x83aO\x14\x91\x84Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aP]W\x90\x84\x91aPHW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`#T\x16\x82;\x15aG\xC9W\x83Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x83\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15aP<W\x90\x83\x91aP'W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a)\x1FWPa\x10\xEFWP\xF3[\x81aP1\x91a_\xBDV[a\x16\xABW\x81_aO\xB8V[PPQ\x90=\x90\x82>=\x90\xFD[\x81aPR\x91a_\xBDV[a\x16&W\x82_aO<V[PPPQ\x90=\x90\x82>=\x90\xFD[\x81aPt\x91a_\xBDV[aG\xC9W\x84_aNkV[\x85Q=\x88\x82>=\x90\xFD[\x95P\x92P` \x85=` \x11aP\xB8W[\x81aP\xA6` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x85\x94Q\x92_aM\xF9V[=\x91PaP\x99V[\x93\x94P\x91P` \x83=` \x11aP\xF4W[\x81aP\xDE` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEW\x91Q\x85\x93\x92\x90\x91` aM\xB7V[=\x91PaP\xD1V[\x84Q=\x87\x82>=\x90\xFD[\x81aQ\x10\x91a_\xBDV[a\x07mW\x82_aMhV[aQ4\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aL\xD8V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10aQ\x99Wa\x02\xC2\x85a\x02\xB6\x81\x87\x03\x82a_\xBDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aQ\x82V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x16\xABW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaS(W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaS\x13W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x16&W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x04wWa\x10\xEFWP\xF3[\x81aS\x1D\x91a_\xBDV[a\x02\nW\x80_aR\xBEV[\x81aS2\x91a_\xBDV[a\x02\nW\x80_aR,V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\nW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04wWaTdW[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90a0\x13\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17aT7W\x91\x83\x91`@\x93alw\x849\x85\x82R` \x82\x01R\x03\x01\x90\x82\xF0\x15aT+W\x80\xF3[`@Q\x90=\x90\x82>=\x90\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81aTn\x91a_\xBDV[a\x02\nW\x80_aS\xDEV[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`@\x80QaT\x98\x82\x82a_\xBDV[`\x0C\x81R\x82` \x82\x01\x7FdefaultAdmin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83Q`\x0C` \x82\x01\x92\x83^\x82`,\x82\x01R`\x0C\x81RaT\xE4`,\x82a_\xBDV[Q\x90 \x83Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a[9W\x82\x91a[\xDFW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aU\xA8\x92\x86Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R\x88`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a[9Wa[\xCAW[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U\x80QaV\x04\x82\x82a_\xBDV[`\x06\x81R\x82` \x82\x01\x7Fminter\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83Q`\x06` \x82\x01\x92\x83^\x82`&\x82\x01R`\x06\x81RaVP`&\x82a_\xBDV[Q\x90 \x83Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a[9W\x82\x91a[\xABW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aW\x14\x92\x86Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R\x88`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a[9Wa[\x96W[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U\x80QaWp\x82\x82a_\xBDV[`\x04\x81R\x82` \x82\x01\x7Fuser\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83Q`\x04` \x82\x01\x92\x83^\x82`$\x82\x01R`\x04\x81RaW\xBC`$\x82a_\xBDV[Q\x90 \x83Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a[9W\x82\x91a[wW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aX\x80\x92\x86Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R\x88`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a[9Wa[bW[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"U\x80QaX\xDC\x82\x82a_\xBDV[`\x07\x81R\x82` \x82\x01\x7Fspender\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83Q`\x07` \x82\x01\x92\x83^\x82`'\x82\x01R`\x07\x81RaY(`'\x82a_\xBDV[Q\x90 \x83Q\x90\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a[9W\x82\x91a[CW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x86W\x81`\x01`\x01`\xA0\x1B\x03aY\xEC\x92\x86Q\x93\x84\x92\x83\x92\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x16\x96\x87`\x04\x84\x01R\x88`$\x84\x01R`D\x83\x01\x90a]\xE6V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a[9Wa[$W[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`#T\x16\x17`#U`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x82Q\x91a0\x13\x80\x84\x01\x90\x84\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17aZ\xF7W\x91\x84\x93\x91aZ\x98\x93alw\x869`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[\x03\x90\x83\xF0\x90\x81\x15aZ\xECWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80\xF3[Q\x91=\x91P\x82>=\x90\xFD[`$\x87\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a[.\x91a_\xBDV[a\x07mW\x82_aZ\x11V[\x84Q=\x84\x82>=\x90\xFD[a[\\\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aY\x81V[\x81a[l\x91a_\xBDV[a\x07mW\x82_aX\xA5V[a[\x90\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aX\x15V[\x81a[\xA0\x91a_\xBDV[a\x07mW\x82_aW9V[a[\xC4\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aV\xA9V[\x81a[\xD4\x91a_\xBDV[a\x07mW\x82_aU\xCDV[a[\xF8\x91P` =` \x11a\x0C\xEFWa\x0C\xE1\x81\x83a_\xBDV[_aU=V[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x02\nW\x80`\x03\x196\x01\x12a\x02\nW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`$`@Q\x80\x94\x81\x93\x7F\xBBMD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x04wW\x82\x90a(\xEBWa\x04E\x91Pae\xD4V[\x90P4a\x04\xAEW_`\x03\x196\x01\x12a\x04\xAEWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a]\x99Wa]\x86W[P`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x90a0\x13\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17aT7W\x91\x83\x91`@\x93alw\x849\x81R\x84` \x82\x01R\x03\x01\x90\x82\xF0\x15aT+W\x80\xF3[a]\x92\x91P_\x90a_\xBDV[__a]:V[`@Q=_\x82>=\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a]\xC7WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a]\xBAV[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a^(WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a^\x1BV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a^\x92WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a^\xCE\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa]\xE6V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a^\x83V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a_\x0FWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a_e\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a^\x0BV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a_\0V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a_\x90W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a_\x90W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a_\x90W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a`\xF1W[` \x85\x10\x84\x14a`\xC4W\x84\x87R\x86\x93\x90\x81\x15a`\x84WP`\x01\x14a`@W[Pa`>\x92P\x03\x83a_\xBDV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a`hWPP\x90` a`>\x92\x82\x01\x01_a`1V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a`OV[` \x93Pa`>\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a`1V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a`\x12V[`@Q\x90aa\n`@\x83a_\xBDV[`\x05\x82R\x7Fuser3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[\x91\x90\x82\x01\x80\x92\x11aaCWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90\x81` \x91\x03\x12a\x04\xAEWQ\x80\x15\x15\x81\x03a\x04\xAEW\x90V[\x91\x90\x82\x03\x91\x82\x11aaCWV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10ac\xACWa`>\x94T\x91\x81\x81\x10acvW[\x81\x81\x10ac@W[\x81\x81\x10ac\nW[\x81\x81\x10ab\xD4W[\x81\x81\x10ab\x9EW[\x81\x81\x10abhW[\x81\x81\x10ab3W[\x10ab\x06W[P\x03\x83a_\xBDV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aa\xFEV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aa\xF8V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aa\xF0V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aa\xE8V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aa\xE0V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aa\xD8V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aa\xD0V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aa\xC8V[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aa\xB0V[` \x81\x83\x03\x12a\x04\xAEW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\xAEW\x01\x81`\x1F\x82\x01\x12\x15a\x04\xAEW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a_\x90W`@Q\x92ad\x8B`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a_\xBDV[\x82\x84R` \x83\x83\x01\x01\x11a\x04\xAEW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x04\xAEWV[\x90\x81` \x91\x03\x12a\x04\xAEWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x04\xAEW\x90V[\x90\x81``\x91\x03\x12a\x04\xAEWad\xED\x81ad\xACV[\x91`@` \x83\x01Q\x92\x01Q\x90V[`\x08T`\xFF\x16\x80\x15ae\nW\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a]\x99W_\x91ae\xA2W[P\x15\x15\x90V[\x90P` \x81=` \x11ae\xCCW[\x81ae\xBD` \x93\x83a_\xBDV[\x81\x01\x03\x12a\x04\xAEWQ_ae\x9CV[=\x91Pae\xB0V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[_a`>\x91a_\xBDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh%\xF2s\x93=\xB5p\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh\x10CV\x1A\x88)0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01ah\xE2\x92aj\xB3V[\x90_\x80`@Qah\xF3`@\x82a_\xBDV[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Qai`\x81a+\xEE` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`@`$\x84\x01R`d\x83\x01\x90a]\xE6V[Q\x90jconsole.logZ\xFAPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEWai\xD2_\x91ai\xE4`@Q\x94\x85\x93\x84\x93\x7F\xF3 \xD9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90a]\xE6V[\x90`\x03\x19\x84\x83\x03\x01`$\x85\x01Ra]\xE6V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xAEW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a]\x99WafJWPV[\x81\x15aj\x86W\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x90\x83\x83\x11ak\xF2W\x82\x81\x10\x91\x82\x15\x80ak\xE8W[ak\xE0Waj\xD6\x84\x86aa\x88V[\x92`\x01\x84\x01\x80\x94\x11aaCW`\x03\x83\x11\x15\x80ak\xD7W[ak\xC8W`\x03\x19\x83\x10\x15\x80ak\xBEW[ak\xAAW\x85\x83\x11\x15akaWPP\x90ak\x19\x84ak\x1E\x93aa\x88V[aj|V[\x90\x81\x15ak\\Wak/\x92Paa6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11aaCW\x90V[PP\x90V[\x95\x94\x92\x91\x90\x95akrW[PPPPV[\x83\x94\x95Pak\x19\x90ak\x84\x93\x94aa\x88V[\x90\x81\x15ak\\Wak\x95\x92Paa\x88V[`\x01\x81\x01\x80\x91\x11aaCW\x90_\x80\x80\x80aklV[PP\x90Pak\xBB\x92\x91P\x19\x90aa\x88V[\x90V[P\x82\x19\x84\x11aj\xFDV[PP\x91\x90Pak\xBB\x92Paa6V[P\x82\x84\x11aj\xEDV[P\x92PPP\x90V[P\x84\x82\x11\x15aj\xC8V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R\xFD\xFEa\x01`\x80`@R4a\x04\xB8W`@\x81a0\x13\x808\x03\x80\x91a\0 \x82\x85a\x04\xBCV[\x839\x81\x01\x03\x12a\x04\xB8Wa\0?` a\08\x83a\x04\xDFV[\x92\x01a\x04\xDFV[`@Qa\0M`@\x82a\x04\xBCV[`\x11\x81R` \x81\x01pTestnet Syndicate`x\x1B\x81R`@Q\x90a\0{`@\x83a\x04\xBCV[`\x11\x82RpTestnet Syndicate`x\x1B` \x83\x01R`@Q\x92a\0\xA8`@\x85a\x04\xBCV[`\x0B\x84Rj\x15\x19\\\xDD\x1B\x99]\x14\xD6S\x91`\xAA\x1B` \x85\x01R`@Q\x93a\0\xCF`@\x86a\x04\xBCV[`\x01\x85R`1`\xF8\x1B` \x86\x01\x90\x81R\x84Q\x90\x94`\x01`\x01`@\x1B\x03\x82\x11a\x03\xBBW`\x03T\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x04\xAEW[` \x83\x10\x14a\x03\x9DW\x81`\x1F\x84\x93\x11a\x04@W[P` \x90`\x1F\x83\x11`\x01\x14a\x03\xDAW_\x92a\x03\xCFW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x03U[\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xBBW`\x04T\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x03\xB1W[` \x83\x10\x14a\x03\x9DW\x81`\x1F\x84\x93\x11a\x03/W[P` \x90`\x1F\x83\x11`\x01\x14a\x02\xC9W_\x92a\x02\xBEW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x04U[a\x01\xAD\x81a\x05\xFCV[a\x01 Ra\x01\xBA\x84a\x07\x83V[a\x01@RQ\x90 \x91\x82`\xE0RQ\x90 \x80a\x01\0RF`\xA0R`@Q\x90` \x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x02#`\xC0\x82a\x04\xBCV[Q\x90 `\x80R0`\xC0R`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x02\xAFW`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x02\xAFWa\x02Wa\x02]\x92a\x04\xF3V[Pa\x05iV[P`@Qa&\xF7\x90\x81a\x08\xBC\x829`\x80Q\x81a\x17\xB9\x01R`\xA0Q\x81a\x18v\x01R`\xC0Q\x81a\x17\x8A\x01R`\xE0Q\x81a\x18\x08\x01Ra\x01\0Q\x81a\x18.\x01Ra\x01 Q\x81a\n\xDC\x01Ra\x01@Q\x81a\x0B\x05\x01R\xF3[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[\x01Q\x90P_\x80a\x01\x8FV[`\x04_\x90\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x03\x17WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x02\xFFW[PPP\x81\x1B\x01`\x04Ua\x01\xA4V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02\xF1V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x02\xDBV[`\x04_R\x90\x91P\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x03\x93W[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x03\x85WPa\x01yV[_\x81U\x84\x93P`\x01\x01a\x03xV[\x90\x91P\x81\x90a\x03jV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x01eV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P_\x80a\x01-V[`\x03_\x90\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x04(WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x04\x10W[PPP\x81\x1B\x01`\x03Ua\x01BV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x04\x02V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x03\xECV[`\x03_R\x90\x91P\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x04\xA4W[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x04\x96WPa\x01\x17V[_\x81U\x84\x93P`\x01\x01a\x04\x89V[\x90\x91P\x81\x90a\x04{V[\x91`\x7F\x16\x91a\x01\x03V[_\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x03\xBBW`@RV[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x04\xB8WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a/\xF3_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05dW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a/\xF3_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a/\xB3_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a/\xD3_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05dW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a/\xD3_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6\x90_Q` a/\xB3_9_Q\x90_R\x90\x80\xA4`\x01\x90V[\x90\x81Q` \x81\x10_\x14a\x06vWP\x90`\x1F\x81Q\x11a\x066W` \x81Q\x91\x01Q` \x82\x10a\x06'W\x17\x90V[_\x19\x82` \x03`\x03\x1B\x1B\x16\x17\x90V[`D` \x91`@Q\x92\x83\x91c0Z'\xA9`\xE0\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBBW`\x06T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x07yW[` \x82\x10\x14a\x03\x9DW`\x1F\x81\x11a\x07FW[P` \x92`\x1F\x82\x11`\x01\x14a\x06\xE5W\x92\x81\x92\x93_\x92a\x06\xDAW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x06U`\xFF\x90V[\x01Q\x90P_\x80a\x06\xC1V[`\x1F\x19\x82\x16\x93`\x06_R\x80_ \x91_[\x86\x81\x10a\x07.WP\x83`\x01\x95\x96\x10a\x07\x16W[PPP\x81\x1B\x01`\x06U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x07\x08V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x06\xF5V[`\x06_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90`\x1F\x83\x01`\x05\x1C\x01[\x81\x81\x10a\x07nWPa\x06\xA7V[_\x81U`\x01\x01a\x07aV[\x90`\x7F\x16\x90a\x06\x95V[\x90\x81Q` \x81\x10_\x14a\x07\xAEWP\x90`\x1F\x81Q\x11a\x066W` \x81Q\x91\x01Q` \x82\x10a\x06'W\x17\x90V[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xBBW`\x07T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x08\xB1W[` \x82\x10\x14a\x03\x9DW`\x1F\x81\x11a\x08~W[P` \x92`\x1F\x82\x11`\x01\x14a\x08\x1DW\x92\x81\x92\x93_\x92a\x08\x12W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x07U`\xFF\x90V[\x01Q\x90P_\x80a\x07\xF9V[`\x1F\x19\x82\x16\x93`\x07_R\x80_ \x91_[\x86\x81\x10a\x08fWP\x83`\x01\x95\x96\x10a\x08NW[PPP\x81\x1B\x01`\x07U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x08@V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x08-V[`\x07_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90`\x1F\x83\x01`\x05\x1C\x01[\x81\x81\x10a\x08\xA6WPa\x07\xDFV[_\x81U`\x01\x01a\x08\x99V[\x90`\x7F\x16\x90a\x07\xCDV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x025W\x80c\x06\xFD\xDE\x03\x14a\x020W\x80c\t^\xA7\xB3\x14a\x02+W\x80c\x18\x16\r\xDD\x14a\x01\xB8W\x80c#\xB8r\xDD\x14a\x02&W\x80c$\x8A\x9C\xA3\x14a\x02!W\x80c//\xF1]\x14a\x02\x1CW\x80c1<\xE5g\x14a\x02\x17W\x80c6D\xE5\x15\x14a\x02\x12W\x80c6V\x8A\xBE\x14a\x02\rW\x80c:F\xB1\xA8\x14a\x01\xC2W\x80c@\xC1\x0F\x19\x14a\x02\x08W\x80cK\xF5\xD7\xE9\x14a\x02\x03W\x80cX|\xDE\x1E\x14a\x01\xFEW\x80c\\\x19\xA9\\\x14a\x01\xF9W\x80co\xCF\xFFE\x14a\x01\xF4W\x80cp\xA0\x821\x14a\x01\xEFW\x80c~\xCE\xBE\0\x14a\x01\xEAW\x80c\x84\xB0\x19n\x14a\x01\xE5W\x80c\x8ES\x9E\x8C\x14a\x01\xE0W\x80c\x91\xD1HT\x14a\x01\xDBW\x80c\x91\xDD\xAD\xF4\x14a\x01\xD6W\x80c\x95\xD8\x9BA\x14a\x01\xD1W\x80c\x9A\xB2N\xB0\x14a\x01\xBDW\x80c\xA2\x17\xFD\xDF\x14a\x01\xCCW\x80c\xA9\x05\x9C\xBB\x14a\x01\xC7W\x80c\xB0\xCA%>\x14a\x01\xC2W\x80c\xBBMD6\x14a\x01\xBDW\x80c\xC0*\xE7T\x14a\x01\xB8W\x80c\xC3\xCD\xA5 \x14a\x01\xB3W\x80c\xD5\x05\xAC\xCF\x14a\x01\xAEW\x80c\xD59\x13\x93\x14a\x01\xA9W\x80c\xD5Gt\x1F\x14a\x01\xA4W\x80c\xDDb\xED>\x14a\x01\x9FWc\xF1\x12~\xD8\x14a\x01\x9AW_\x80\xFD[a\x11\xECV[a\x11\x93V[a\x11UV[a\x11\x1BV[a\x0F\xC1V[a\x0EzV[a\x04\x86V[a\r\xF7V[a\x06rV[a\x0E4V[a\x0E\x1AV[a\rRV[a\r'V[a\x0C\xD7V[a\x0B\xFBV[a\n\xC4V[a\n\x8CV[a\nWV[a\t\xDCV[a\t\xBAV[a\tyV[a\x08\xD0V[a\x07\x84V[a\x06\x15V[a\x05\xFBV[a\x05\xE0V[a\x05\x9BV[a\x05hV[a\x04\xA3V[a\x04UV[a\x031V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W`\x045\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x80\x91\x03a\x02\xD6W\x80\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x92\x14\x90\x81\x15a\x02\xACW[P`@Q\x90\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14_a\x02\xA1V[_\x80\xFD[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` a\x03.\x92\x81\x81R\x01\x90a\x02\xDAV[\x90V[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W`@Q_`\x03Ta\x03Q\x81a\x12\xB5V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\xE7WP`\x01\x14a\x03\x89W[a\x03\x85\x83a\x03y\x81\x85\x03\x82a\x13\xF4V[`@Q\x91\x82\x91\x82a\x03\x1DV[\x03\x90\xF3[`\x03_\x90\x81R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x93\x92P\x90[\x80\x82\x10a\x03\xCDWP\x90\x91P\x81\x01` \x01a\x03ya\x03iV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x03\xB5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x03y\x90Pa\x03iV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xD6WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xD6WV[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x04{a\x04qa\x04)V[`$5\x903a\x1B\x03V[` `@Q`\x01\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` `\x02T`@Q\x90\x81R\xF3[4a\x02\xD6W```\x03\x196\x01\x12a\x02\xD6Wa\x04\xBCa\x04)V[a\x04\xC4a\x04?V[`D5\x90`\x01`\x01`\xA0\x1B\x03\x83\x16_R`\x01` Ra\x04\xF73`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x92_\x19\x84\x10a\x05\x18W[a\x05\x0C\x93Pa\x14\x99V[`@Q`\x01\x81R` \x90\xF3[\x82\x84\x10a\x054Wa\x05/\x83a\x05\x0C\x95\x033\x83a\x1B\xD1V[a\x05\x02V[\x82\x84\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d_\xFD[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W` a\x05\x93`\x045_R`\x05` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x05\xDE`\x045a\x05\xBAa\x04?V[\x90a\x05\xD9a\x05\xD4\x82_R`\x05` R`\x01`@_ \x01T\x90V[a\x16gV[a\x16\xC8V[\0[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` `@Q`\x12\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` a\x05\x93a\x17\x80V[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6W`\x045a\x061a\x04?V[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x06JWa\x05\xDE\x91a\x18\x9CV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x06\x8Ba\x04)V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\n` Ra\x06\xAC`@_ \x91a\x19LV[\x81T\x90_\x82\x91`\x05\x84\x11a\x07,W[a\x06\xC6\x93P\x84a\x1E\x0CV[\x80a\x06\xF5WPP` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_[\x16`@Q\x90\x81R\xF3[` \x91a\x07\x1Cy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a\x19\xCBV[\x90_R\x82_ \x01T`0\x1Ca\x06\xECV[\x91\x92a\x077\x81a\x1C\x97V[\x81\x03\x90\x81\x11a\x07\x7FWa\x06\xC6\x93\x85_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x07mWP\x91a\x06\xBBV[\x92\x91Pa\x07y\x90a\x19\xD9V[\x90a\x06\xBBV[a\x19\x9EV[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x07\x9Da\x04)V[`$5a\x07\xA8a\x15\xDFV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a\x08\xA8W\x81\x15a\x08\x80Wa\x07\xD2a\x07\xCD\x83`\x02Ta\x19\xE7V[`\x02UV[a\x07\xEC\x83`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x83\x01\x90U`@Q\x82\x81R_\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3`\x02T\x91y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11a\x08PWa\x05\xDE\x83\x83a$6V[\x7F\x1C\xB1]&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x84\x90R`$R`D_\xFD[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6Wa\x08\xE9Ca\x1C\x18V[e\xFF\xFF\xFF\xFF\xFF\xFF\x80a\x08\xFACa\x1C\x18V[\x16\x91\x16\x03a\tQWa\x03\x85`@Qa\t\x13`@\x82a\x13\xF4V[`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02\xDAV[\x7Fo\xF0q@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W`\x01`\x01`\xA0\x1B\x03a\t\x9Aa\x04)V[\x16_R`\t` R` `\x01`\x01`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6Wa\x05\xDEa\t\xD6a\x04)V[3a\x19\xF4V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W`\x01`\x01`\xA0\x1B\x03a\t\xFDa\x04)V[\x16_R`\n` R`@_ Tc\xFF\xFF\xFF\xFF\x81\x11a\n'W`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` `\x04R`$R`D_\xFD[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W` a\x05\x93a\nua\x04)V[`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W`\x01`\x01`\xA0\x1B\x03a\n\xADa\x04)V[\x16_R`\x08` R` `@_ T`@Q\x90\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6Wa\x0B\xA2a\x0B\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1F\xC3V[a\x0B)\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a <V[` `@Qa\x0B8\x82\x82a\x13\xF4V[_\x81R\x81a\x0B\xB0\x81\x83\x01\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x83\x016\x877`@Q\x97\x88\x97\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a\x02\xDAV[\x90\x87\x82\x03`@\x89\x01Ra\x02\xDAV[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\x0B\xE4WPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x0B\xD5V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6Wa\x0C\x17`\x045a\x19LV[`\x0BT\x90_\x82\x91`\x05\x84\x11a\x0C\x83W[a\x0C3\x93P`\x0Ba\x1E\x0CV[\x80a\x0CaWP` _[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[a\x0C~a\x0Co` \x92a\x19\xCBV[`\x0B_R\x82_ \x01T`0\x1C\x90V[a\x0C=V[\x91\x92a\x0C\x8E\x81a\x1C\x97V[\x81\x03\x90\x81\x11a\x07\x7FWa\x0C3\x93`\x0B_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x0C\xC5WP\x91a\x0C'V[\x92\x91Pa\x0C\xD1\x90a\x19\xD9V[\x90a\x0C'V[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6W` `\xFFa\r\x1B`\x045a\x0C\xFAa\x04?V[\x90_R`\x05\x84R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` a\rBCa\x1C\x18V[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W`@Q_`\x04Ta\rr\x81a\x12\xB5V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\xE7WP`\x01\x14a\r\x99Wa\x03\x85\x83a\x03y\x81\x85\x03\x82a\x13\xF4V[`\x04_\x90\x81R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93\x92P\x90[\x80\x82\x10a\r\xDDWP\x90\x91P\x81\x01` \x01a\x03ya\x03iV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\r\xC5V[4a\x02\xD6W` `\x03\x196\x01\x12a\x02\xD6W` a\x05\x93a\x0E\x15a\x04)V[a\x14FV[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` `@Q_\x81R\xF3[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x04{a\x0EPa\x04)V[`$5\x903a\x14\x99V[`d5\x90`\xFF\x82\x16\x82\x03a\x02\xD6WV[`\x845\x90`\xFF\x82\x16\x82\x03a\x02\xD6WV[4a\x02\xD6W`\xC0`\x03\x196\x01\x12a\x02\xD6Wa\x0E\x93a\x04)V[`$5\x90`D5a\x0E\xA2a\x0EZV[`\x845\x90`\xA45\x92\x80B\x11a\x0F\x96W\x91a\x0F(\x93\x91a\x0F\x1Aa\x0F\x1F\x94`@Q` \x81\x01\x91\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x83R`\x01`\x01`\xA0\x1B\x03\x8A\x16`@\x83\x01R\x8A``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x0F\x12`\xA0\x82a\x13\xF4V[Q\x90 a\x1A\xB3V[a sV[\x90\x92\x91\x92a!7V[a\x0FL\x81`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x80\x93\x03a\x0F]Wa\x05\xDE\x92Pa\x19\xF4V[`\x01`\x01`\xA0\x1B\x03\x91P\x7Fu-\x88\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$R`D_\xFD[\x7FF\x83\xAF\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x02\xD6W`\xE0`\x03\x196\x01\x12a\x02\xD6Wa\x0F\xDAa\x04)V[a\x0F\xE2a\x04?V[`D5\x90`d5\x92a\x0F\xF2a\x0EjV[`\xA45`\xC45\x90\x86B\x11a\x10\xEFWa\x10\x9B\x92a\x10\x96a\x10+\x86`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x98`@Q` \x81\x01\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R`\x01`\x01`\xA0\x1B\x03\x89\x16\x9B\x8C`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16``\x84\x01R\x8B`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x0F\x12`\xE0\x82a\x13\xF4V[a\x1A\xF4V[\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\x10\xB5Wa\x05\xDE\x93Pa\x1B\x03V[\x7FK\x80\x0EF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04R\x16`$R`D_\xFD[\x86\x7Fby\x13\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x02\xD6W_`\x03\x196\x01\x12a\x02\xD6W` `@Q\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6\x81R\xF3[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x05\xDE`\x045a\x11ta\x04?V[\x90a\x11\x8Ea\x05\xD4\x82_R`\x05` R`\x01`@_ \x01T\x90V[a\x18\x9CV[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6W` a\x11\xE3a\x11\xB1a\x04)V[`\x01`\x01`\xA0\x1B\x03a\x11\xC1a\x04?V[\x91\x16_R`\x01\x83R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x02\xD6W`@`\x03\x196\x01\x12a\x02\xD6Wa\x12\x05a\x04)V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\xD6Wa\x03\x85\x91`\x01`\x01`\xA0\x1B\x03a\x12R\x92a\x12.a\x14\x81V[Pa\x127a\x14\x81V[P\x16_R`\n` R`@_ a\x12La\x14\x81V[Pa!\xFEV[P`@Q\x90a\x12`\x82a\x13\xD3V[Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82R`0\x1C` \x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`@\x84\x01\x95e\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x12\xFCW[` \x83\x10\x14a\x12\xCFWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x12\xC4V[_\x92\x91\x81T\x91a\x13\x15\x83a\x12\xB5V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a\x13jWP`\x01\x14a\x131WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a\x13PWP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a\x13?V[\x90P` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x13\xEFW`@RV[a\x13\xA6V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x13\xEFW`@RV[`@Q\x90a\x14D`@\x83a\x13\xF4V[V[`\x01`\x01`\xA0\x1B\x03\x16_R`\n` Ry\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x14}`@_ a\x1A\x8AV[\x16\x90V[`@Q\x90a\x14\x8E\x82a\x13\xD3V[_` \x83\x82\x81R\x01RV[\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x93\x84\x15a\x15\xB3W`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a\x15\x87Wa\x14\xD7\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x84\x81\x10a\x15SW\x95\x84a\x14D\x96\x97\x03a\x15\x01\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[Ua\x15\x1C\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x86\x01\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a$\xB9V[\x84\x90\x87\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[3_\x90\x81R\x7F\x15\xA2\x8D&\xFA\x1B\xF76\xCF~\xDC\x99\"`qq\xCC\xB0\x9C<s\xB8\x08\xE7w*0\x13\xE0h\xA5\"` R`@\x90 T`\xFF\x16\x15a\x16\x17WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6`$R`D_\xFD[\x80_R`\x05` R`\xFFa\x16\x8F3`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a\x16\x99WPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R`\x05` R`\xFFa\x16\xF0\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16a\x17zW\x80_R`\x05` Ra\x17\x1C\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x14\x80a\x18sW[\x15a\x17\xDBW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x18m`\xC0\x82a\x13\xF4V[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x17\xB2V[\x80_R`\x05` R`\xFFa\x18\xC4\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a\x17zW\x80_R`\x05` Ra\x18\xF1\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[e\xFF\xFF\xFF\xFF\xFF\xFFa\x19\\Ca\x1C\x18V[\x16\x80\x82\x10\x15a\x19oWPa\x03.\x90a\x1C\x18V[\x90\x7F\xEC\xD3\xF8\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90_\x19\x82\x01\x91\x82\x11a\x07\x7FWV[\x90`\x01\x82\x01\x80\x92\x11a\x07\x7FWV[\x91\x90\x82\x01\x80\x92\x11a\x07\x7FWV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\t` R`@\x81 \x80T\x86\x85\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x81\x17\x90\x92Ua\x14D\x96\x94\x16\x94a\x1A\x84\x93\x90\x92\x86\x91\x90\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x90\x80\xA4`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[\x91a\x1EpV[\x80T\x80a\x1A\x97WPP_\x90V[\x80_\x19\x81\x01\x11a\x07\x7FW_\x19\x91_R` _ \x01\x01T`0\x1C\x90V[`B\x90a\x1A\xBEa\x17\x80V[\x90`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91a\x03.\x93\x91a\x0F\x1F\x93a sV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x1B\xA5W`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x83\x15a\x1ByW\x80a\x1Bl\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93\x85_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[U`@Q\x90\x81R` \x90\xA3V[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x1B\xA5W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1ByWa\x1C\x15\x91_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[UV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1C0We\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`0`\x04R`$R`D_\xFD[\x81\x15a\x1CjW\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[`\x01\x81\x11\x15a\x03.W\x80`\x01p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10\x15a\x1D\xCAW[a\x1Dpa\x1Dfa\x1D\\a\x1DRa\x1DHa\x1D>a\x1D-a\x1Dw\x97`\x04\x8Ah\x01\0\0\0\0\0\0\0\0a\x1D|\x9C\x10\x15a\x1D\xBDW[d\x01\0\0\0\0\x81\x10\x15a\x1D\xB0W[b\x01\0\0\x81\x10\x15a\x1D\xA3W[a\x01\0\x81\x10\x15a\x1D\x96W[`\x10\x81\x10\x15a\x1D\x89W[\x10\x15a\x1D\x81W[`\x03\x02`\x01\x1C\x90V[a\x1D7\x81\x8Ba\x1C`V[\x01`\x01\x1C\x90V[a\x1D7\x81\x8Aa\x1C`V[a\x1D7\x81\x89a\x1C`V[a\x1D7\x81\x88a\x1C`V[a\x1D7\x81\x87a\x1C`V[a\x1D7\x81\x86a\x1C`V[\x80\x93a\x1C`V[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1Ba\x1D$V[`\x04\x1C\x91`\x02\x1B\x91a\x1D\x1DV[`\x08\x1C\x91`\x04\x1B\x91a\x1D\x13V[`\x10\x1C\x91`\x08\x1B\x91a\x1D\x08V[` \x1C\x91`\x10\x1B\x91a\x1C\xFCV[`@\x1C\x91` \x1B\x91a\x1C\xEEV[PPa\x1D|a\x1Dwa\x1Dpa\x1Dfa\x1D\\a\x1DRa\x1DHa\x1D>a\x1D-a\x1D\xF1\x8A`\x80\x1C\x90V[\x98Ph\x01\0\0\0\0\0\0\0\0\x97Pa\x1C\xBD\x96PPPPPPPV[\x91\x90[\x83\x82\x10a\x1E\x1CWPPP\x90V[\x90\x91\x92\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x07\x7FW\x84_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x10_\x14a\x1E^WP\x92[\x91\x90a\x1E\x0FV[\x93\x92Pa\x1Ej\x90a\x19\xD9V[\x91a\x1EWV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x84\x82\x14\x15\x80a\x1F\xBAW[a\x1E\x9EW[PPPPPV[\x81a\x1FDW[PP\x82a\x1E\xB3W[\x80\x80a\x1E\x97V[a\x1F9a\x1F \x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x93a\x1F\x1Aa\x1F\x14y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[\x91a\"@V[\x90a#\x14V[`@\x80Q\x92\x85\x16\x83R\x93\x16` \x82\x01R\x91\x82\x91\x90\x82\x01\x90V[\x03\x90\xA2_\x80\x80a\x1E\xACV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x1F\xB0a\x1F a\x1F\xA1\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x94`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[a\x1F\xAA\x88a\"@V[\x90a\"\xB0V[\x03\x90\xA2_\x80a\x1E\xA4V[P\x83\x15\x15a\x1E\x92V[`\xFF\x81\x14a \"W`\xFF\x81\x16\x90`\x1F\x82\x11a\x1F\xFAW`@Q\x91a\x1F\xE7`@\x84a\x13\xF4V[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[\x7F\xB3Q+\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P`@Qa\x03.\x81a 5\x81`\x06a\x13\x06V[\x03\x82a\x13\xF4V[`\xFF\x81\x14a `W`\xFF\x81\x16\x90`\x1F\x82\x11a\x1F\xFAW`@Q\x91a\x1F\xE7`@\x84a\x13\xF4V[P`@Qa\x03.\x81a 5\x81`\x07a\x13\x06V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a \xF5W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a \xEAW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a \xE0W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[`@Q=_\x82>=\x90\xFD[PPP_\x91`\x03\x91\x90V[`\x04\x11\x15a!\nWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a!@\x81a!\0V[\x80a!IWPPV[a!R\x81a!\0V[`\x01\x81\x03a!\x82W\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a!\x8B\x81a!\0V[`\x02\x81\x03a!\xBFWP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80a!\xCB`\x03\x92a!\0V[\x14a!\xD3WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80T\x82\x10\x15a\"\x13W_R` _ \x01\x90_\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\"\x80Wy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\xD0`\x04R`$R`D_\xFD[\x90a\"\xBACa\x1C\x18V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a\"\xE0\x85a\x1A\x8AV[\x92\x16\x91\x16\x03\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07\x7FWa#\x10\x92a%\xC4V[\x90\x91V[\x90a#\x1ECa\x1C\x18V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a#D\x85a\x1A\x8AV[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07\x7FWa#\x10\x92a%\xC4V[a#}Ca\x1C\x18V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a#\xA4`\x0Ba\x1A\x8AV[\x92\x16\x91\x16\x01y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x7FWa#\x10\x91`\x0Ba%\xC4V[a#\xDECa\x1C\x18V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a$\x05`\x0Ba\x1A\x8AV[\x92\x16\x91\x16\x03y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x7FWa#\x10\x91`\x0Ba%\xC4V[\x90`\x01`\x01`\xA0\x1B\x03a\x14D\x92a$Ta$O\x84a\"@V[a#tV[PP\x16\x80\x15a$\xA1W[`\t` R\x7F\xEC\x81Vq\x8A\x83r\xB1\xDBD\xBBA\x147\xD0\x87\x0F>7\x90\xD4\xA0\x85&\xD0$\xCE\x1B\x0Bf\x8FkT_\x91\x82R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x1EpV[a$\xB2a$\xAD\x83a\"@V[a#\xD5V[PPa$^V[\x90`\x01`\x01`\xA0\x1B\x03\x80a\x14D\x94\x93\x16\x91\x82\x15a%\x1EW[\x16\x90\x81\x15a%\x0BW[_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90a\x1EpV[a%\x17a$\xAD\x84a\"@V[PPa$\xDAV[a%*a$O\x85a\"@V[PPa$\xD1V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x13\xEFWa%S\x91`\x01\x82\x01\x81Ua!\xFEV[a%\x98W\x81Q` \x92\x90\x92\x01Q`0\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a&\xBAWa%\xDBa%\xE6\x91a\x19\xCBV[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a&\x92W\x87\x93\x03a&KWPa&G\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x83T\x92`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa&G\x91a&ka&]a\x145V[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16` \x83\x01Ra%1V[\x7F% `\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P\x90a&\xF2\x91a&\xCBa&]a\x145V[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x83\x01Ra%1V[_\x91\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x15\xA2\x8D&\xFA\x1B\xF76\xCF~\xDC\x99\"`qq\xCC\xB0\x9C<s\xB8\x08\xE7w*0\x13\xE0h\xA5\"\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Approval(address,address,uint256)` and selector `0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925`.
```solidity
event Approval(address indexed owner, address indexed spender, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Approval {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Approval {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Approval(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8, 113u8,
                66u8, 125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8, 247u8, 178u8,
                41u8, 30u8, 91u8, 32u8, 10u8, 200u8, 199u8, 195u8, 185u8, 37u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    spender: topics.2,
                    value: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.owner.clone(), self.spender.clone())
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
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.spender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Approval {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Approval> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Approval) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `Transfer(address,address,uint256)` and selector `0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef`.
```solidity
event Transfer(address indexed from, address indexed to, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Transfer(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8,
                176u8, 104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8, 99u8,
                196u8, 161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8, 179u8, 239u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    to: topics.2,
                    value: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.to.clone())
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
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `defaultAdmin()` and selector `0x84ef8ffc`.
```solidity
function defaultAdmin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defaultAdminCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`defaultAdmin()`](defaultAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defaultAdminReturn {
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
            impl ::core::convert::From<defaultAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: defaultAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defaultAdminCall {
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
            impl ::core::convert::From<defaultAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: defaultAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defaultAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defaultAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defaultAdmin()";
            const SELECTOR: [u8; 4] = [132u8, 239u8, 143u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: defaultAdminReturn = r.into();
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
                        let r: defaultAdminReturn = r.into();
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
    /**Function with signature `minter()` and selector `0x07546172`.
```solidity
function minter() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minterCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`minter()`](minterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minterReturn {
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
            impl ::core::convert::From<minterCall> for UnderlyingRustTuple<'_> {
                fn from(value: minterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minterCall {
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
            impl ::core::convert::From<minterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minter()";
            const SELECTOR: [u8; 4] = [7u8, 84u8, 97u8, 114u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: minterReturn = r.into();
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
                        let r: minterReturn = r.into();
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
    /**Function with signature `spender()` and selector `0xe8edc816`.
```solidity
function spender() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct spenderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`spender()`](spenderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct spenderReturn {
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
            impl ::core::convert::From<spenderCall> for UnderlyingRustTuple<'_> {
                fn from(value: spenderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for spenderCall {
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
            impl ::core::convert::From<spenderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: spenderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for spenderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for spenderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "spender()";
            const SELECTOR: [u8; 4] = [232u8, 237u8, 200u8, 22u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: spenderReturn = r.into();
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
                        let r: spenderReturn = r.into();
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
    /**Function with signature `testFuzz_Mint_ValidAmounts(address,uint256)` and selector `0xc01e9428`.
```solidity
function testFuzz_Mint_ValidAmounts(address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_Mint_ValidAmountsCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzz_Mint_ValidAmounts(address,uint256)`](testFuzz_Mint_ValidAmountsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_Mint_ValidAmountsReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testFuzz_Mint_ValidAmountsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_Mint_ValidAmountsCall) -> Self {
                    (value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_Mint_ValidAmountsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<testFuzz_Mint_ValidAmountsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_Mint_ValidAmountsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_Mint_ValidAmountsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzz_Mint_ValidAmountsReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzz_Mint_ValidAmountsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzz_Mint_ValidAmountsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzz_Mint_ValidAmountsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzz_Mint_ValidAmounts(address,uint256)";
            const SELECTOR: [u8; 4] = [192u8, 30u8, 148u8, 40u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzz_Mint_ValidAmountsReturn::_tokenize(ret)
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
    /**Function with signature `testFuzz_Transfer_ValidAmounts(uint256,uint256)` and selector `0x71d7dabf`.
```solidity
function testFuzz_Transfer_ValidAmounts(uint256 mintAmount, uint256 transferAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_Transfer_ValidAmountsCall {
        #[allow(missing_docs)]
        pub mintAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub transferAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzz_Transfer_ValidAmounts(uint256,uint256)`](testFuzz_Transfer_ValidAmountsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_Transfer_ValidAmountsReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testFuzz_Transfer_ValidAmountsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_Transfer_ValidAmountsCall) -> Self {
                    (value.mintAmount, value.transferAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_Transfer_ValidAmountsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        mintAmount: tuple.0,
                        transferAmount: tuple.1,
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
            impl ::core::convert::From<testFuzz_Transfer_ValidAmountsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_Transfer_ValidAmountsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_Transfer_ValidAmountsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzz_Transfer_ValidAmountsReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzz_Transfer_ValidAmountsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzz_Transfer_ValidAmountsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzz_Transfer_ValidAmountsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzz_Transfer_ValidAmounts(uint256,uint256)";
            const SELECTOR: [u8; 4] = [113u8, 215u8, 218u8, 191u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.mintAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.transferAmount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzz_Transfer_ValidAmountsReturn::_tokenize(ret)
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
    /**Function with signature `test_Approve_Success()` and selector `0x640f725a`.
```solidity
function test_Approve_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Approve_SuccessCall;
    ///Container type for the return parameters of the [`test_Approve_Success()`](test_Approve_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Approve_SuccessReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_Approve_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Approve_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Approve_SuccessCall {
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
            impl ::core::convert::From<test_Approve_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Approve_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Approve_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Approve_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_Approve_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Approve_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Approve_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Approve_Success()";
            const SELECTOR: [u8; 4] = [100u8, 15u8, 114u8, 90u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_Approve_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_Delegate_Success()` and selector `0x91dc0b2d`.
```solidity
function test_Delegate_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Delegate_SuccessCall;
    ///Container type for the return parameters of the [`test_Delegate_Success()`](test_Delegate_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Delegate_SuccessReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_Delegate_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Delegate_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Delegate_SuccessCall {
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
            impl ::core::convert::From<test_Delegate_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Delegate_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Delegate_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Delegate_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_Delegate_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Delegate_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Delegate_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Delegate_Success()";
            const SELECTOR: [u8; 4] = [145u8, 220u8, 11u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_Delegate_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_GetVotingPower_WithTokens()` and selector `0x6338aa86`.
```solidity
function test_GetVotingPower_WithTokens() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetVotingPower_WithTokensCall;
    ///Container type for the return parameters of the [`test_GetVotingPower_WithTokens()`](test_GetVotingPower_WithTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetVotingPower_WithTokensReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_GetVotingPower_WithTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetVotingPower_WithTokensCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetVotingPower_WithTokensCall {
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
            impl ::core::convert::From<test_GetVotingPower_WithTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetVotingPower_WithTokensReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetVotingPower_WithTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GetVotingPower_WithTokensReturn {
            fn _tokenize(
                &self,
            ) -> <test_GetVotingPower_WithTokensCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GetVotingPower_WithTokensCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GetVotingPower_WithTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GetVotingPower_WithTokens()";
            const SELECTOR: [u8; 4] = [99u8, 56u8, 170u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_GetVotingPower_WithTokensReturn::_tokenize(ret)
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
    /**Function with signature `test_GetVotingPower_WithoutTokens()` and selector `0x0642dde5`.
```solidity
function test_GetVotingPower_WithoutTokens() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetVotingPower_WithoutTokensCall;
    ///Container type for the return parameters of the [`test_GetVotingPower_WithoutTokens()`](test_GetVotingPower_WithoutTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetVotingPower_WithoutTokensReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_GetVotingPower_WithoutTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetVotingPower_WithoutTokensCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetVotingPower_WithoutTokensCall {
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
            impl ::core::convert::From<test_GetVotingPower_WithoutTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetVotingPower_WithoutTokensReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetVotingPower_WithoutTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GetVotingPower_WithoutTokensReturn {
            fn _tokenize(
                &self,
            ) -> <test_GetVotingPower_WithoutTokensCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GetVotingPower_WithoutTokensCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GetVotingPower_WithoutTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GetVotingPower_WithoutTokens()";
            const SELECTOR: [u8; 4] = [6u8, 66u8, 221u8, 229u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_GetVotingPower_WithoutTokensReturn::_tokenize(ret)
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
    /**Function with signature `test_GrantMinterRole_Success()` and selector `0x8d31ed53`.
```solidity
function test_GrantMinterRole_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GrantMinterRole_SuccessCall;
    ///Container type for the return parameters of the [`test_GrantMinterRole_Success()`](test_GrantMinterRole_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GrantMinterRole_SuccessReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_GrantMinterRole_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GrantMinterRole_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GrantMinterRole_SuccessCall {
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
            impl ::core::convert::From<test_GrantMinterRole_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GrantMinterRole_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GrantMinterRole_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GrantMinterRole_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_GrantMinterRole_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GrantMinterRole_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GrantMinterRole_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GrantMinterRole_Success()";
            const SELECTOR: [u8; 4] = [141u8, 49u8, 237u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_GrantMinterRole_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_Invariant_TotalSupplyMatchesBalances()` and selector `0x41686ff2`.
```solidity
function test_Invariant_TotalSupplyMatchesBalances() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Invariant_TotalSupplyMatchesBalancesCall;
    ///Container type for the return parameters of the [`test_Invariant_TotalSupplyMatchesBalances()`](test_Invariant_TotalSupplyMatchesBalancesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Invariant_TotalSupplyMatchesBalancesReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_Invariant_TotalSupplyMatchesBalancesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Invariant_TotalSupplyMatchesBalancesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Invariant_TotalSupplyMatchesBalancesCall {
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
            impl ::core::convert::From<test_Invariant_TotalSupplyMatchesBalancesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Invariant_TotalSupplyMatchesBalancesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Invariant_TotalSupplyMatchesBalancesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Invariant_TotalSupplyMatchesBalancesReturn {
            fn _tokenize(
                &self,
            ) -> <test_Invariant_TotalSupplyMatchesBalancesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Invariant_TotalSupplyMatchesBalancesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Invariant_TotalSupplyMatchesBalancesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Invariant_TotalSupplyMatchesBalances()";
            const SELECTOR: [u8; 4] = [65u8, 104u8, 111u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_Invariant_TotalSupplyMatchesBalancesReturn::_tokenize(ret)
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
    /**Function with signature `test_Mint_MultipleMints()` and selector `0x8f310dfe`.
```solidity
function test_Mint_MultipleMints() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Mint_MultipleMintsCall;
    ///Container type for the return parameters of the [`test_Mint_MultipleMints()`](test_Mint_MultipleMintsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Mint_MultipleMintsReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_Mint_MultipleMintsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Mint_MultipleMintsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Mint_MultipleMintsCall {
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
            impl ::core::convert::From<test_Mint_MultipleMintsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Mint_MultipleMintsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Mint_MultipleMintsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Mint_MultipleMintsReturn {
            fn _tokenize(
                &self,
            ) -> <test_Mint_MultipleMintsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Mint_MultipleMintsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Mint_MultipleMintsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Mint_MultipleMints()";
            const SELECTOR: [u8; 4] = [143u8, 49u8, 13u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_Mint_MultipleMintsReturn::_tokenize(ret)
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
    /**Function with signature `test_Mint_Success()` and selector `0x55f7d477`.
```solidity
function test_Mint_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Mint_SuccessCall;
    ///Container type for the return parameters of the [`test_Mint_Success()`](test_Mint_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Mint_SuccessReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_Mint_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Mint_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Mint_SuccessCall {
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
            impl ::core::convert::From<test_Mint_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Mint_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Mint_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Mint_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_Mint_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Mint_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Mint_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Mint_Success()";
            const SELECTOR: [u8; 4] = [85u8, 247u8, 212u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_Mint_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_Permit_Success()` and selector `0xbbb15533`.
```solidity
function test_Permit_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Permit_SuccessCall;
    ///Container type for the return parameters of the [`test_Permit_Success()`](test_Permit_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Permit_SuccessReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_Permit_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Permit_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Permit_SuccessCall {
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
            impl ::core::convert::From<test_Permit_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Permit_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Permit_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Permit_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_Permit_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Permit_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Permit_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Permit_Success()";
            const SELECTOR: [u8; 4] = [187u8, 177u8, 85u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_Permit_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_Constructor_ZeroMinter()` and selector `0x04812054`.
```solidity
function test_RevertWhen_Constructor_ZeroMinter() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Constructor_ZeroMinterCall;
    ///Container type for the return parameters of the [`test_RevertWhen_Constructor_ZeroMinter()`](test_RevertWhen_Constructor_ZeroMinterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Constructor_ZeroMinterReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_RevertWhen_Constructor_ZeroMinterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Constructor_ZeroMinterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Constructor_ZeroMinterCall {
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
            impl ::core::convert::From<test_RevertWhen_Constructor_ZeroMinterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Constructor_ZeroMinterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Constructor_ZeroMinterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_Constructor_ZeroMinterReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_Constructor_ZeroMinterCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_Constructor_ZeroMinterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_Constructor_ZeroMinterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_Constructor_ZeroMinter()";
            const SELECTOR: [u8; 4] = [4u8, 129u8, 32u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_RevertWhen_Constructor_ZeroMinterReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_GrantRole_NotAdmin()` and selector `0x2246e5cc`.
```solidity
function test_RevertWhen_GrantRole_NotAdmin() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_GrantRole_NotAdminCall;
    ///Container type for the return parameters of the [`test_RevertWhen_GrantRole_NotAdmin()`](test_RevertWhen_GrantRole_NotAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_GrantRole_NotAdminReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_RevertWhen_GrantRole_NotAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_GrantRole_NotAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_GrantRole_NotAdminCall {
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
            impl ::core::convert::From<test_RevertWhen_GrantRole_NotAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_GrantRole_NotAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_GrantRole_NotAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_GrantRole_NotAdminReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_GrantRole_NotAdminCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_GrantRole_NotAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_GrantRole_NotAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_GrantRole_NotAdmin()";
            const SELECTOR: [u8; 4] = [34u8, 70u8, 229u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_RevertWhen_GrantRole_NotAdminReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_Mint_NotMinter()` and selector `0x8f08ece7`.
```solidity
function test_RevertWhen_Mint_NotMinter() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Mint_NotMinterCall;
    ///Container type for the return parameters of the [`test_RevertWhen_Mint_NotMinter()`](test_RevertWhen_Mint_NotMinterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Mint_NotMinterReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_RevertWhen_Mint_NotMinterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Mint_NotMinterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Mint_NotMinterCall {
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
            impl ::core::convert::From<test_RevertWhen_Mint_NotMinterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Mint_NotMinterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Mint_NotMinterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_Mint_NotMinterReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_Mint_NotMinterCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_Mint_NotMinterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_Mint_NotMinterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_Mint_NotMinter()";
            const SELECTOR: [u8; 4] = [143u8, 8u8, 236u8, 231u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_RevertWhen_Mint_NotMinterReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_Mint_ZeroAddress()` and selector `0x746a9bcf`.
```solidity
function test_RevertWhen_Mint_ZeroAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Mint_ZeroAddressCall;
    ///Container type for the return parameters of the [`test_RevertWhen_Mint_ZeroAddress()`](test_RevertWhen_Mint_ZeroAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Mint_ZeroAddressReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_RevertWhen_Mint_ZeroAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Mint_ZeroAddressCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Mint_ZeroAddressCall {
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
            impl ::core::convert::From<test_RevertWhen_Mint_ZeroAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Mint_ZeroAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Mint_ZeroAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_Mint_ZeroAddressReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_Mint_ZeroAddressCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_Mint_ZeroAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_Mint_ZeroAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_Mint_ZeroAddress()";
            const SELECTOR: [u8; 4] = [116u8, 106u8, 155u8, 207u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_RevertWhen_Mint_ZeroAddressReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_Mint_ZeroAmount()` and selector `0x123a4a5f`.
```solidity
function test_RevertWhen_Mint_ZeroAmount() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Mint_ZeroAmountCall;
    ///Container type for the return parameters of the [`test_RevertWhen_Mint_ZeroAmount()`](test_RevertWhen_Mint_ZeroAmountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Mint_ZeroAmountReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_RevertWhen_Mint_ZeroAmountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Mint_ZeroAmountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Mint_ZeroAmountCall {
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
            impl ::core::convert::From<test_RevertWhen_Mint_ZeroAmountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Mint_ZeroAmountReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Mint_ZeroAmountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_Mint_ZeroAmountReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_Mint_ZeroAmountCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_Mint_ZeroAmountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_Mint_ZeroAmountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_Mint_ZeroAmount()";
            const SELECTOR: [u8; 4] = [18u8, 58u8, 74u8, 95u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_RevertWhen_Mint_ZeroAmountReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_Permit_ExpiredDeadline()` and selector `0xb78b5967`.
```solidity
function test_RevertWhen_Permit_ExpiredDeadline() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Permit_ExpiredDeadlineCall;
    ///Container type for the return parameters of the [`test_RevertWhen_Permit_ExpiredDeadline()`](test_RevertWhen_Permit_ExpiredDeadlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Permit_ExpiredDeadlineReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_RevertWhen_Permit_ExpiredDeadlineCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Permit_ExpiredDeadlineCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Permit_ExpiredDeadlineCall {
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
            impl ::core::convert::From<test_RevertWhen_Permit_ExpiredDeadlineReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Permit_ExpiredDeadlineReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Permit_ExpiredDeadlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_Permit_ExpiredDeadlineReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_Permit_ExpiredDeadlineCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_Permit_ExpiredDeadlineCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_Permit_ExpiredDeadlineReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_Permit_ExpiredDeadline()";
            const SELECTOR: [u8; 4] = [183u8, 139u8, 89u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_RevertWhen_Permit_ExpiredDeadlineReturn::_tokenize(ret)
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
    /**Function with signature `test_RevokeMinterRole_Success()` and selector `0x47483c5d`.
```solidity
function test_RevokeMinterRole_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevokeMinterRole_SuccessCall;
    ///Container type for the return parameters of the [`test_RevokeMinterRole_Success()`](test_RevokeMinterRole_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevokeMinterRole_SuccessReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_RevokeMinterRole_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevokeMinterRole_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevokeMinterRole_SuccessCall {
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
            impl ::core::convert::From<test_RevokeMinterRole_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevokeMinterRole_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevokeMinterRole_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevokeMinterRole_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevokeMinterRole_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevokeMinterRole_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevokeMinterRole_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevokeMinterRole_Success()";
            const SELECTOR: [u8; 4] = [71u8, 72u8, 60u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_RevokeMinterRole_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_TransferFrom_Success()` and selector `0x5bb17781`.
```solidity
function test_TransferFrom_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_TransferFrom_SuccessCall;
    ///Container type for the return parameters of the [`test_TransferFrom_Success()`](test_TransferFrom_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_TransferFrom_SuccessReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_TransferFrom_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_TransferFrom_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_TransferFrom_SuccessCall {
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
            impl ::core::convert::From<test_TransferFrom_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_TransferFrom_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_TransferFrom_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_TransferFrom_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_TransferFrom_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_TransferFrom_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_TransferFrom_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_TransferFrom_Success()";
            const SELECTOR: [u8; 4] = [91u8, 177u8, 119u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_TransferFrom_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_Transfer_Success()` and selector `0x88c5671b`.
```solidity
function test_Transfer_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Transfer_SuccessCall;
    ///Container type for the return parameters of the [`test_Transfer_Success()`](test_Transfer_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Transfer_SuccessReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<test_Transfer_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Transfer_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Transfer_SuccessCall {
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
            impl ::core::convert::From<test_Transfer_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Transfer_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Transfer_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Transfer_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_Transfer_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Transfer_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Transfer_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Transfer_Success()";
            const SELECTOR: [u8; 4] = [136u8, 197u8, 103u8, 27u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                test_Transfer_SuccessReturn::_tokenize(ret)
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
    ///Container for all the [`TestnetSyndTokenTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum TestnetSyndTokenTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        defaultAdmin(defaultAdminCall),
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
        minter(minterCall),
        #[allow(missing_docs)]
        setUp(setUpCall),
        #[allow(missing_docs)]
        spender(spenderCall),
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
        testFuzz_Mint_ValidAmounts(testFuzz_Mint_ValidAmountsCall),
        #[allow(missing_docs)]
        testFuzz_Transfer_ValidAmounts(testFuzz_Transfer_ValidAmountsCall),
        #[allow(missing_docs)]
        test_Approve_Success(test_Approve_SuccessCall),
        #[allow(missing_docs)]
        test_Constructor_InitialSetup(test_Constructor_InitialSetupCall),
        #[allow(missing_docs)]
        test_Constructor_RoleAssignment(test_Constructor_RoleAssignmentCall),
        #[allow(missing_docs)]
        test_Delegate_Success(test_Delegate_SuccessCall),
        #[allow(missing_docs)]
        test_GetVotingPower_WithTokens(test_GetVotingPower_WithTokensCall),
        #[allow(missing_docs)]
        test_GetVotingPower_WithoutTokens(test_GetVotingPower_WithoutTokensCall),
        #[allow(missing_docs)]
        test_GrantMinterRole_Success(test_GrantMinterRole_SuccessCall),
        #[allow(missing_docs)]
        test_Invariant_TotalSupplyMatchesBalances(
            test_Invariant_TotalSupplyMatchesBalancesCall,
        ),
        #[allow(missing_docs)]
        test_Mint_MultipleMints(test_Mint_MultipleMintsCall),
        #[allow(missing_docs)]
        test_Mint_Success(test_Mint_SuccessCall),
        #[allow(missing_docs)]
        test_Permit_Success(test_Permit_SuccessCall),
        #[allow(missing_docs)]
        test_RevertWhen_Constructor_ZeroAdmin(test_RevertWhen_Constructor_ZeroAdminCall),
        #[allow(missing_docs)]
        test_RevertWhen_Constructor_ZeroMinter(
            test_RevertWhen_Constructor_ZeroMinterCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_GrantRole_NotAdmin(test_RevertWhen_GrantRole_NotAdminCall),
        #[allow(missing_docs)]
        test_RevertWhen_Mint_NotMinter(test_RevertWhen_Mint_NotMinterCall),
        #[allow(missing_docs)]
        test_RevertWhen_Mint_ZeroAddress(test_RevertWhen_Mint_ZeroAddressCall),
        #[allow(missing_docs)]
        test_RevertWhen_Mint_ZeroAmount(test_RevertWhen_Mint_ZeroAmountCall),
        #[allow(missing_docs)]
        test_RevertWhen_Permit_ExpiredDeadline(
            test_RevertWhen_Permit_ExpiredDeadlineCall,
        ),
        #[allow(missing_docs)]
        test_RevokeMinterRole_Success(test_RevokeMinterRole_SuccessCall),
        #[allow(missing_docs)]
        test_TransferFrom_Success(test_TransferFrom_SuccessCall),
        #[allow(missing_docs)]
        test_Transfer_Success(test_Transfer_SuccessCall),
        #[allow(missing_docs)]
        token(tokenCall),
        #[allow(missing_docs)]
        user(userCall),
    }
    impl TestnetSyndTokenTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 129u8, 32u8, 84u8],
            [6u8, 66u8, 221u8, 229u8],
            [7u8, 84u8, 97u8, 114u8],
            [10u8, 146u8, 84u8, 228u8],
            [16u8, 116u8, 162u8, 31u8],
            [18u8, 58u8, 74u8, 95u8],
            [30u8, 215u8, 131u8, 28u8],
            [34u8, 70u8, 229u8, 204u8],
            [42u8, 222u8, 56u8, 128u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [65u8, 104u8, 111u8, 242u8],
            [71u8, 72u8, 60u8, 93u8],
            [79u8, 134u8, 50u8, 186u8],
            [85u8, 247u8, 212u8, 119u8],
            [91u8, 177u8, 119u8, 129u8],
            [99u8, 56u8, 170u8, 134u8],
            [100u8, 15u8, 114u8, 90u8],
            [102u8, 217u8, 169u8, 160u8],
            [113u8, 215u8, 218u8, 191u8],
            [116u8, 106u8, 155u8, 207u8],
            [118u8, 2u8, 158u8, 120u8],
            [132u8, 239u8, 143u8, 252u8],
            [133u8, 34u8, 108u8, 129u8],
            [136u8, 197u8, 103u8, 27u8],
            [141u8, 49u8, 237u8, 83u8],
            [143u8, 8u8, 236u8, 231u8],
            [143u8, 49u8, 13u8, 254u8],
            [145u8, 106u8, 23u8, 198u8],
            [145u8, 220u8, 11u8, 45u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [183u8, 139u8, 89u8, 103u8],
            [186u8, 65u8, 79u8, 166u8],
            [187u8, 177u8, 85u8, 51u8],
            [192u8, 30u8, 148u8, 40u8],
            [220u8, 204u8, 87u8, 241u8],
            [226u8, 12u8, 159u8, 113u8],
            [232u8, 237u8, 200u8, 22u8],
            [250u8, 118u8, 38u8, 212u8],
            [252u8, 12u8, 84u8, 106u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(test_RevertWhen_Constructor_ZeroMinter),
            ::core::stringify!(test_GetVotingPower_WithoutTokens),
            ::core::stringify!(minter),
            ::core::stringify!(setUp),
            ::core::stringify!(test_RevertWhen_Constructor_ZeroAdmin),
            ::core::stringify!(test_RevertWhen_Mint_ZeroAmount),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(test_RevertWhen_GrantRole_NotAdmin),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(test_Invariant_TotalSupplyMatchesBalances),
            ::core::stringify!(test_RevokeMinterRole_Success),
            ::core::stringify!(user),
            ::core::stringify!(test_Mint_Success),
            ::core::stringify!(test_TransferFrom_Success),
            ::core::stringify!(test_GetVotingPower_WithTokens),
            ::core::stringify!(test_Approve_Success),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(testFuzz_Transfer_ValidAmounts),
            ::core::stringify!(test_RevertWhen_Mint_ZeroAddress),
            ::core::stringify!(test_Constructor_InitialSetup),
            ::core::stringify!(defaultAdmin),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(test_Transfer_Success),
            ::core::stringify!(test_GrantMinterRole_Success),
            ::core::stringify!(test_RevertWhen_Mint_NotMinter),
            ::core::stringify!(test_Mint_MultipleMints),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(test_Delegate_Success),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(test_RevertWhen_Permit_ExpiredDeadline),
            ::core::stringify!(failed),
            ::core::stringify!(test_Permit_Success),
            ::core::stringify!(testFuzz_Mint_ValidAmounts),
            ::core::stringify!(test_Constructor_RoleAssignment),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(spender),
            ::core::stringify!(IS_TEST),
            ::core::stringify!(token),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <test_RevertWhen_Constructor_ZeroMinterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_GetVotingPower_WithoutTokensCall as alloy_sol_types::SolCall>::SIGNATURE,
            <minterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_Mint_ZeroAmountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_GrantRole_NotAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Invariant_TotalSupplyMatchesBalancesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevokeMinterRole_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <userCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Mint_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_TransferFrom_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_GetVotingPower_WithTokensCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Approve_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_Transfer_ValidAmountsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_Mint_ZeroAddressCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::SIGNATURE,
            <defaultAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Transfer_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_GrantMinterRole_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_Mint_NotMinterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Mint_MultipleMintsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Delegate_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_Permit_ExpiredDeadlineCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Permit_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_Mint_ValidAmountsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <spenderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <IS_TESTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tokenCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for TestnetSyndTokenTestCalls {
        const NAME: &'static str = "TestnetSyndTokenTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 41usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::defaultAdmin(_) => {
                    <defaultAdminCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::minter(_) => <minterCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::spender(_) => <spenderCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::testFuzz_Mint_ValidAmounts(_) => {
                    <testFuzz_Mint_ValidAmountsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzz_Transfer_ValidAmounts(_) => {
                    <testFuzz_Transfer_ValidAmountsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Approve_Success(_) => {
                    <test_Approve_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_InitialSetup(_) => {
                    <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_RoleAssignment(_) => {
                    <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Delegate_Success(_) => {
                    <test_Delegate_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GetVotingPower_WithTokens(_) => {
                    <test_GetVotingPower_WithTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GetVotingPower_WithoutTokens(_) => {
                    <test_GetVotingPower_WithoutTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GrantMinterRole_Success(_) => {
                    <test_GrantMinterRole_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Invariant_TotalSupplyMatchesBalances(_) => {
                    <test_Invariant_TotalSupplyMatchesBalancesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Mint_MultipleMints(_) => {
                    <test_Mint_MultipleMintsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Mint_Success(_) => {
                    <test_Mint_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Permit_Success(_) => {
                    <test_Permit_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Constructor_ZeroAdmin(_) => {
                    <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Constructor_ZeroMinter(_) => {
                    <test_RevertWhen_Constructor_ZeroMinterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_GrantRole_NotAdmin(_) => {
                    <test_RevertWhen_GrantRole_NotAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Mint_NotMinter(_) => {
                    <test_RevertWhen_Mint_NotMinterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Mint_ZeroAddress(_) => {
                    <test_RevertWhen_Mint_ZeroAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Mint_ZeroAmount(_) => {
                    <test_RevertWhen_Mint_ZeroAmountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Permit_ExpiredDeadline(_) => {
                    <test_RevertWhen_Permit_ExpiredDeadlineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevokeMinterRole_Success(_) => {
                    <test_RevokeMinterRole_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_TransferFrom_Success(_) => {
                    <test_TransferFrom_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Transfer_Success(_) => {
                    <test_Transfer_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls>] = &[
                {
                    fn test_RevertWhen_Constructor_ZeroMinter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Constructor_ZeroMinterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Constructor_ZeroMinter,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroMinter
                },
                {
                    fn test_GetVotingPower_WithoutTokens(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_GetVotingPower_WithoutTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_GetVotingPower_WithoutTokens,
                            )
                    }
                    test_GetVotingPower_WithoutTokens
                },
                {
                    fn minter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <minterCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(TestnetSyndTokenTestCalls::minter)
                    }
                    minter
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(TestnetSyndTokenTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_RevertWhen_Constructor_ZeroAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Constructor_ZeroAdmin,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroAdmin
                },
                {
                    fn test_RevertWhen_Mint_ZeroAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Mint_ZeroAmountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Mint_ZeroAmount,
                            )
                    }
                    test_RevertWhen_Mint_ZeroAmount
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn test_RevertWhen_GrantRole_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_GrantRole_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_GrantRole_NotAdmin,
                            )
                    }
                    test_RevertWhen_GrantRole_NotAdmin
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn test_Invariant_TotalSupplyMatchesBalances(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Invariant_TotalSupplyMatchesBalancesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_Invariant_TotalSupplyMatchesBalances,
                            )
                    }
                    test_Invariant_TotalSupplyMatchesBalances
                },
                {
                    fn test_RevokeMinterRole_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevokeMinterRole_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevokeMinterRole_Success,
                            )
                    }
                    test_RevokeMinterRole_Success
                },
                {
                    fn user(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <userCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(TestnetSyndTokenTestCalls::user)
                    }
                    user
                },
                {
                    fn test_Mint_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Mint_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Mint_Success)
                    }
                    test_Mint_Success
                },
                {
                    fn test_TransferFrom_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_TransferFrom_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_TransferFrom_Success)
                    }
                    test_TransferFrom_Success
                },
                {
                    fn test_GetVotingPower_WithTokens(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_GetVotingPower_WithTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_GetVotingPower_WithTokens,
                            )
                    }
                    test_GetVotingPower_WithTokens
                },
                {
                    fn test_Approve_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Approve_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Approve_Success)
                    }
                    test_Approve_Success
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testFuzz_Transfer_ValidAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <testFuzz_Transfer_ValidAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::testFuzz_Transfer_ValidAmounts,
                            )
                    }
                    testFuzz_Transfer_ValidAmounts
                },
                {
                    fn test_RevertWhen_Mint_ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Mint_ZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Mint_ZeroAddress,
                            )
                    }
                    test_RevertWhen_Mint_ZeroAddress
                },
                {
                    fn test_Constructor_InitialSetup(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_Constructor_InitialSetup,
                            )
                    }
                    test_Constructor_InitialSetup
                },
                {
                    fn defaultAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <defaultAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::defaultAdmin)
                    }
                    defaultAdmin
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn test_Transfer_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Transfer_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Transfer_Success)
                    }
                    test_Transfer_Success
                },
                {
                    fn test_GrantMinterRole_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_GrantMinterRole_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_GrantMinterRole_Success)
                    }
                    test_GrantMinterRole_Success
                },
                {
                    fn test_RevertWhen_Mint_NotMinter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Mint_NotMinterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Mint_NotMinter,
                            )
                    }
                    test_RevertWhen_Mint_NotMinter
                },
                {
                    fn test_Mint_MultipleMints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Mint_MultipleMintsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Mint_MultipleMints)
                    }
                    test_Mint_MultipleMints
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_Delegate_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Delegate_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Delegate_Success)
                    }
                    test_Delegate_Success
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn test_RevertWhen_Permit_ExpiredDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Permit_ExpiredDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Permit_ExpiredDeadline,
                            )
                    }
                    test_RevertWhen_Permit_ExpiredDeadline
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(TestnetSyndTokenTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_Permit_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Permit_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Permit_Success)
                    }
                    test_Permit_Success
                },
                {
                    fn testFuzz_Mint_ValidAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <testFuzz_Mint_ValidAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::testFuzz_Mint_ValidAmounts)
                    }
                    testFuzz_Mint_ValidAmounts
                },
                {
                    fn test_Constructor_RoleAssignment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_Constructor_RoleAssignment,
                            )
                    }
                    test_Constructor_RoleAssignment
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn spender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <spenderCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(TestnetSyndTokenTestCalls::spender)
                    }
                    spender
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(TestnetSyndTokenTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(TestnetSyndTokenTestCalls::token)
                    }
                    token
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
            ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls>] = &[
                {
                    fn test_RevertWhen_Constructor_ZeroMinter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Constructor_ZeroMinterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Constructor_ZeroMinter,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroMinter
                },
                {
                    fn test_GetVotingPower_WithoutTokens(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_GetVotingPower_WithoutTokensCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_GetVotingPower_WithoutTokens,
                            )
                    }
                    test_GetVotingPower_WithoutTokens
                },
                {
                    fn minter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <minterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::minter)
                    }
                    minter
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_RevertWhen_Constructor_ZeroAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Constructor_ZeroAdmin,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroAdmin
                },
                {
                    fn test_RevertWhen_Mint_ZeroAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Mint_ZeroAmountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Mint_ZeroAmount,
                            )
                    }
                    test_RevertWhen_Mint_ZeroAmount
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn test_RevertWhen_GrantRole_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_GrantRole_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_GrantRole_NotAdmin,
                            )
                    }
                    test_RevertWhen_GrantRole_NotAdmin
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn test_Invariant_TotalSupplyMatchesBalances(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Invariant_TotalSupplyMatchesBalancesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_Invariant_TotalSupplyMatchesBalances,
                            )
                    }
                    test_Invariant_TotalSupplyMatchesBalances
                },
                {
                    fn test_RevokeMinterRole_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevokeMinterRole_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevokeMinterRole_Success,
                            )
                    }
                    test_RevokeMinterRole_Success
                },
                {
                    fn user(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <userCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::user)
                    }
                    user
                },
                {
                    fn test_Mint_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Mint_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Mint_Success)
                    }
                    test_Mint_Success
                },
                {
                    fn test_TransferFrom_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_TransferFrom_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_TransferFrom_Success)
                    }
                    test_TransferFrom_Success
                },
                {
                    fn test_GetVotingPower_WithTokens(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_GetVotingPower_WithTokensCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_GetVotingPower_WithTokens,
                            )
                    }
                    test_GetVotingPower_WithTokens
                },
                {
                    fn test_Approve_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Approve_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Approve_Success)
                    }
                    test_Approve_Success
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testFuzz_Transfer_ValidAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <testFuzz_Transfer_ValidAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::testFuzz_Transfer_ValidAmounts,
                            )
                    }
                    testFuzz_Transfer_ValidAmounts
                },
                {
                    fn test_RevertWhen_Mint_ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Mint_ZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Mint_ZeroAddress,
                            )
                    }
                    test_RevertWhen_Mint_ZeroAddress
                },
                {
                    fn test_Constructor_InitialSetup(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_Constructor_InitialSetup,
                            )
                    }
                    test_Constructor_InitialSetup
                },
                {
                    fn defaultAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <defaultAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::defaultAdmin)
                    }
                    defaultAdmin
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn test_Transfer_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Transfer_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Transfer_Success)
                    }
                    test_Transfer_Success
                },
                {
                    fn test_GrantMinterRole_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_GrantMinterRole_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_GrantMinterRole_Success)
                    }
                    test_GrantMinterRole_Success
                },
                {
                    fn test_RevertWhen_Mint_NotMinter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Mint_NotMinterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Mint_NotMinter,
                            )
                    }
                    test_RevertWhen_Mint_NotMinter
                },
                {
                    fn test_Mint_MultipleMints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Mint_MultipleMintsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Mint_MultipleMints)
                    }
                    test_Mint_MultipleMints
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_Delegate_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Delegate_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Delegate_Success)
                    }
                    test_Delegate_Success
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn test_RevertWhen_Permit_ExpiredDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_RevertWhen_Permit_ExpiredDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_RevertWhen_Permit_ExpiredDeadline,
                            )
                    }
                    test_RevertWhen_Permit_ExpiredDeadline
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_Permit_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Permit_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::test_Permit_Success)
                    }
                    test_Permit_Success
                },
                {
                    fn testFuzz_Mint_ValidAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <testFuzz_Mint_ValidAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::testFuzz_Mint_ValidAmounts)
                    }
                    testFuzz_Mint_ValidAmounts
                },
                {
                    fn test_Constructor_RoleAssignment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                TestnetSyndTokenTestCalls::test_Constructor_RoleAssignment,
                            )
                    }
                    test_Constructor_RoleAssignment
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn spender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <spenderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::spender)
                    }
                    spender
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<TestnetSyndTokenTestCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(TestnetSyndTokenTestCalls::token)
                    }
                    token
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
                Self::defaultAdmin(inner) => {
                    <defaultAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::minter(inner) => {
                    <minterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::spender(inner) => {
                    <spenderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::testFuzz_Mint_ValidAmounts(inner) => {
                    <testFuzz_Mint_ValidAmountsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzz_Transfer_ValidAmounts(inner) => {
                    <testFuzz_Transfer_ValidAmountsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Approve_Success(inner) => {
                    <test_Approve_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::test_Delegate_Success(inner) => {
                    <test_Delegate_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GetVotingPower_WithTokens(inner) => {
                    <test_GetVotingPower_WithTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GetVotingPower_WithoutTokens(inner) => {
                    <test_GetVotingPower_WithoutTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GrantMinterRole_Success(inner) => {
                    <test_GrantMinterRole_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Invariant_TotalSupplyMatchesBalances(inner) => {
                    <test_Invariant_TotalSupplyMatchesBalancesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Mint_MultipleMints(inner) => {
                    <test_Mint_MultipleMintsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Mint_Success(inner) => {
                    <test_Mint_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Permit_Success(inner) => {
                    <test_Permit_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Constructor_ZeroAdmin(inner) => {
                    <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Constructor_ZeroMinter(inner) => {
                    <test_RevertWhen_Constructor_ZeroMinterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_GrantRole_NotAdmin(inner) => {
                    <test_RevertWhen_GrantRole_NotAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Mint_NotMinter(inner) => {
                    <test_RevertWhen_Mint_NotMinterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Mint_ZeroAddress(inner) => {
                    <test_RevertWhen_Mint_ZeroAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Mint_ZeroAmount(inner) => {
                    <test_RevertWhen_Mint_ZeroAmountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Permit_ExpiredDeadline(inner) => {
                    <test_RevertWhen_Permit_ExpiredDeadlineCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevokeMinterRole_Success(inner) => {
                    <test_RevokeMinterRole_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_TransferFrom_Success(inner) => {
                    <test_TransferFrom_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Transfer_Success(inner) => {
                    <test_Transfer_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::defaultAdmin(inner) => {
                    <defaultAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::minter(inner) => {
                    <minterCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::spender(inner) => {
                    <spenderCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::testFuzz_Mint_ValidAmounts(inner) => {
                    <testFuzz_Mint_ValidAmountsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzz_Transfer_ValidAmounts(inner) => {
                    <testFuzz_Transfer_ValidAmountsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Approve_Success(inner) => {
                    <test_Approve_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::test_Delegate_Success(inner) => {
                    <test_Delegate_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_GetVotingPower_WithTokens(inner) => {
                    <test_GetVotingPower_WithTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_GetVotingPower_WithoutTokens(inner) => {
                    <test_GetVotingPower_WithoutTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_GrantMinterRole_Success(inner) => {
                    <test_GrantMinterRole_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Invariant_TotalSupplyMatchesBalances(inner) => {
                    <test_Invariant_TotalSupplyMatchesBalancesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Mint_MultipleMints(inner) => {
                    <test_Mint_MultipleMintsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Mint_Success(inner) => {
                    <test_Mint_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Permit_Success(inner) => {
                    <test_Permit_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::test_RevertWhen_Constructor_ZeroMinter(inner) => {
                    <test_RevertWhen_Constructor_ZeroMinterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_GrantRole_NotAdmin(inner) => {
                    <test_RevertWhen_GrantRole_NotAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_Mint_NotMinter(inner) => {
                    <test_RevertWhen_Mint_NotMinterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_Mint_ZeroAddress(inner) => {
                    <test_RevertWhen_Mint_ZeroAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_Mint_ZeroAmount(inner) => {
                    <test_RevertWhen_Mint_ZeroAmountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_Permit_ExpiredDeadline(inner) => {
                    <test_RevertWhen_Permit_ExpiredDeadlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevokeMinterRole_Success(inner) => {
                    <test_RevokeMinterRole_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_TransferFrom_Success(inner) => {
                    <test_TransferFrom_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Transfer_Success(inner) => {
                    <test_Transfer_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`TestnetSyndTokenTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum TestnetSyndTokenTestEvents {
        #[allow(missing_docs)]
        Approval(Approval),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        Transfer(Transfer),
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
    impl TestnetSyndTokenTestEvents {
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
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
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
                140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8, 113u8,
                66u8, 125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8, 247u8, 178u8,
                41u8, 30u8, 91u8, 32u8, 10u8, 200u8, 199u8, 195u8, 185u8, 37u8,
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
                221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8,
                176u8, 104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8, 99u8,
                196u8, 161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8, 179u8, 239u8,
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
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
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
            ::core::stringify!(RoleGranted),
            ::core::stringify!(log_named_int),
            ::core::stringify!(log_named_array_2),
            ::core::stringify!(log_array_2),
            ::core::stringify!(log),
            ::core::stringify!(log_named_decimal_int),
            ::core::stringify!(log_address),
            ::core::stringify!(log_array_1),
            ::core::stringify!(Approval),
            ::core::stringify!(log_named_address),
            ::core::stringify!(log_named_array_1),
            ::core::stringify!(log_named_bytes32),
            ::core::stringify!(log_named_uint),
            ::core::stringify!(log_named_bytes),
            ::core::stringify!(Transfer),
            ::core::stringify!(logs),
            ::core::stringify!(log_bytes32),
            ::core::stringify!(log_named_decimal_uint),
            ::core::stringify!(RoleRevoked),
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
            <RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <Approval as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <Transfer as alloy_sol_types::SolEvent>::SIGNATURE,
            <logs as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for TestnetSyndTokenTestEvents {
        const NAME: &'static str = "TestnetSyndTokenTestEvents";
        const COUNT: usize = 26usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Approval)
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
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Transfer)
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
    impl alloy_sol_types::private::IntoLogData for TestnetSyndTokenTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Transfer(inner) => {
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
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Transfer(inner) => {
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
    /**Creates a new wrapper around an on-chain [`TestnetSyndTokenTest`](self) contract instance.

See the [wrapper's documentation](`TestnetSyndTokenTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> TestnetSyndTokenTestInstance<P, N> {
        TestnetSyndTokenTestInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<TestnetSyndTokenTestInstance<P, N>>,
    > {
        TestnetSyndTokenTestInstance::<P, N>::deploy(__provider)
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
        TestnetSyndTokenTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`TestnetSyndTokenTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`TestnetSyndTokenTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TestnetSyndTokenTestInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for TestnetSyndTokenTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TestnetSyndTokenTestInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > TestnetSyndTokenTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`TestnetSyndTokenTest`](self) contract instance.

See the [wrapper's documentation](`TestnetSyndTokenTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<TestnetSyndTokenTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> TestnetSyndTokenTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TestnetSyndTokenTestInstance<P, N> {
            TestnetSyndTokenTestInstance {
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
    > TestnetSyndTokenTestInstance<P, N> {
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
        ///Creates a new call builder for the [`defaultAdmin`] function.
        pub fn defaultAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, defaultAdminCall, N> {
            self.call_builder(&defaultAdminCall)
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
        ///Creates a new call builder for the [`minter`] function.
        pub fn minter(&self) -> alloy_contract::SolCallBuilder<&P, minterCall, N> {
            self.call_builder(&minterCall)
        }
        ///Creates a new call builder for the [`setUp`] function.
        pub fn setUp(&self) -> alloy_contract::SolCallBuilder<&P, setUpCall, N> {
            self.call_builder(&setUpCall)
        }
        ///Creates a new call builder for the [`spender`] function.
        pub fn spender(&self) -> alloy_contract::SolCallBuilder<&P, spenderCall, N> {
            self.call_builder(&spenderCall)
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
        ///Creates a new call builder for the [`testFuzz_Mint_ValidAmounts`] function.
        pub fn testFuzz_Mint_ValidAmounts(
            &self,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, testFuzz_Mint_ValidAmountsCall, N> {
            self.call_builder(
                &testFuzz_Mint_ValidAmountsCall {
                    to,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`testFuzz_Transfer_ValidAmounts`] function.
        pub fn testFuzz_Transfer_ValidAmounts(
            &self,
            mintAmount: alloy::sol_types::private::primitives::aliases::U256,
            transferAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, testFuzz_Transfer_ValidAmountsCall, N> {
            self.call_builder(
                &testFuzz_Transfer_ValidAmountsCall {
                    mintAmount,
                    transferAmount,
                },
            )
        }
        ///Creates a new call builder for the [`test_Approve_Success`] function.
        pub fn test_Approve_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Approve_SuccessCall, N> {
            self.call_builder(&test_Approve_SuccessCall)
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
        ///Creates a new call builder for the [`test_Delegate_Success`] function.
        pub fn test_Delegate_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Delegate_SuccessCall, N> {
            self.call_builder(&test_Delegate_SuccessCall)
        }
        ///Creates a new call builder for the [`test_GetVotingPower_WithTokens`] function.
        pub fn test_GetVotingPower_WithTokens(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_GetVotingPower_WithTokensCall, N> {
            self.call_builder(&test_GetVotingPower_WithTokensCall)
        }
        ///Creates a new call builder for the [`test_GetVotingPower_WithoutTokens`] function.
        pub fn test_GetVotingPower_WithoutTokens(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_GetVotingPower_WithoutTokensCall,
            N,
        > {
            self.call_builder(&test_GetVotingPower_WithoutTokensCall)
        }
        ///Creates a new call builder for the [`test_GrantMinterRole_Success`] function.
        pub fn test_GrantMinterRole_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_GrantMinterRole_SuccessCall, N> {
            self.call_builder(&test_GrantMinterRole_SuccessCall)
        }
        ///Creates a new call builder for the [`test_Invariant_TotalSupplyMatchesBalances`] function.
        pub fn test_Invariant_TotalSupplyMatchesBalances(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_Invariant_TotalSupplyMatchesBalancesCall,
            N,
        > {
            self.call_builder(&test_Invariant_TotalSupplyMatchesBalancesCall)
        }
        ///Creates a new call builder for the [`test_Mint_MultipleMints`] function.
        pub fn test_Mint_MultipleMints(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Mint_MultipleMintsCall, N> {
            self.call_builder(&test_Mint_MultipleMintsCall)
        }
        ///Creates a new call builder for the [`test_Mint_Success`] function.
        pub fn test_Mint_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Mint_SuccessCall, N> {
            self.call_builder(&test_Mint_SuccessCall)
        }
        ///Creates a new call builder for the [`test_Permit_Success`] function.
        pub fn test_Permit_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Permit_SuccessCall, N> {
            self.call_builder(&test_Permit_SuccessCall)
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
        ///Creates a new call builder for the [`test_RevertWhen_Constructor_ZeroMinter`] function.
        pub fn test_RevertWhen_Constructor_ZeroMinter(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_Constructor_ZeroMinterCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_Constructor_ZeroMinterCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_GrantRole_NotAdmin`] function.
        pub fn test_RevertWhen_GrantRole_NotAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_GrantRole_NotAdminCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_GrantRole_NotAdminCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_Mint_NotMinter`] function.
        pub fn test_RevertWhen_Mint_NotMinter(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_RevertWhen_Mint_NotMinterCall, N> {
            self.call_builder(&test_RevertWhen_Mint_NotMinterCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_Mint_ZeroAddress`] function.
        pub fn test_RevertWhen_Mint_ZeroAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_Mint_ZeroAddressCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_Mint_ZeroAddressCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_Mint_ZeroAmount`] function.
        pub fn test_RevertWhen_Mint_ZeroAmount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_RevertWhen_Mint_ZeroAmountCall, N> {
            self.call_builder(&test_RevertWhen_Mint_ZeroAmountCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_Permit_ExpiredDeadline`] function.
        pub fn test_RevertWhen_Permit_ExpiredDeadline(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_Permit_ExpiredDeadlineCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_Permit_ExpiredDeadlineCall)
        }
        ///Creates a new call builder for the [`test_RevokeMinterRole_Success`] function.
        pub fn test_RevokeMinterRole_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_RevokeMinterRole_SuccessCall, N> {
            self.call_builder(&test_RevokeMinterRole_SuccessCall)
        }
        ///Creates a new call builder for the [`test_TransferFrom_Success`] function.
        pub fn test_TransferFrom_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_TransferFrom_SuccessCall, N> {
            self.call_builder(&test_TransferFrom_SuccessCall)
        }
        ///Creates a new call builder for the [`test_Transfer_Success`] function.
        pub fn test_Transfer_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Transfer_SuccessCall, N> {
            self.call_builder(&test_Transfer_SuccessCall)
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
    > TestnetSyndTokenTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<&P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<&P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<&P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<&P, Transfer, N> {
            self.event_filter::<Transfer>()
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
