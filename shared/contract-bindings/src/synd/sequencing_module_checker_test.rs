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

interface SequencingModuleCheckerTest {
    event RequirementModuleUpdated(address indexed newModule);
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
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function manager() external view returns (address);
    function masterModule() external view returns (address);
    function mockModule() external view returns (address);
    function nonAdmin() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testCannotInitializeTwice() external;
    function testFuzzDataSizeLimit(uint256 dataSize) external;
    function testFuzzDataSizeRevert(uint256 dataSize) external;
    function testFuzzIsAllowedWithVariousAddresses(address proposer, address originator) external;
    function testInitialization() external;
    function testInitializeWithDifferentModule() external;
    function testIsAllowedWithAddressOne() external;
    function testIsAllowedWithEmptyData() external;
    function testIsAllowedWithLargeData() external;
    function testIsAllowedWithMaxSizeData() external;
    function testIsAllowedWithMockModuleAllow() external;
    function testIsAllowedWithMockModuleDeny() external;
    function testIsAllowedWithOversizedData() external;
    function testIsAllowedWithRealModule() external;
    function testIsAllowedWithZeroAddress() external;
    function testMaxDataSizeConstant() external;
    function testOnlyOwnerFunctions() external;
    function testOwnershipTransfer() external;
    function testPermissionRequirementModuleGetter() external;
    function testRenounceOwnership() external;
    function testStorageLocation() external;
    function testUpdateRequirementModule() external;
    function testUpdateRequirementModuleNonAdmin() external;
    function testUpdateRequirementModuleToAddressOne() external;
    function testUpdateRequirementModuleToZeroAddress() external;
    function testWithRequireAndModule() external;
    function testWithRequireOrModule() external;
    function user1() external view returns (address);
    function user2() external view returns (address);
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
        "internalType": "contract SequencingModuleCheckerMock"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "masterModule",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract RequireAndModule"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "mockModule",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockPermissionModule"
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
    "name": "testCannotInitializeTwice",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testFuzzDataSizeLimit",
    "inputs": [
      {
        "name": "dataSize",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testFuzzDataSizeRevert",
    "inputs": [
      {
        "name": "dataSize",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testFuzzIsAllowedWithVariousAddresses",
    "inputs": [
      {
        "name": "proposer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "originator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testInitialization",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testInitializeWithDifferentModule",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithAddressOne",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithEmptyData",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithLargeData",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithMaxSizeData",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithMockModuleAllow",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithMockModuleDeny",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithOversizedData",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithRealModule",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedWithZeroAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testMaxDataSizeConstant",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testOnlyOwnerFunctions",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testOwnershipTransfer",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testPermissionRequirementModuleGetter",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testRenounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testStorageLocation",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateRequirementModule",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateRequirementModuleNonAdmin",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateRequirementModuleToAddressOne",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateRequirementModuleToZeroAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testWithRequireAndModule",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testWithRequireOrModule",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "user1",
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
    "name": "user2",
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
    "name": "RequirementModuleUpdated",
    "inputs": [
      {
        "name": "newModule",
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
pub mod SequencingModuleCheckerTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234602f57600160ff19600c541617600c55600160ff19601f541617601f5561697d90816100348239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c90816308887aea14613014575080630a9254e414612d555780631436ee7f14612b39578063192c28af14612a3a5780631ed7831c146129bc5780632abc50bf146128615780632ade38801461266d5780632ecdbf56146124b357806331c956a4146123c65780633e5e3c23146123485780633f7286f4146122ca57806342a85af5146121dd5780634359a74e146121b657806344768da3146120b1578063481c6a75146120875780634c58ef7b14611ec657806351d9d3d514611cad5780635387476614611b60578063575fc5d5146119ec5780635e5e5f6914611811578063626d0766146115ba578063629fe631146114af57806366d9a9a0146113725780637004eac41461125d578063716e2aa814610b9357806382eddc221461109057806385226c8114611006578063916a17c614610f5c5780639d4aa91c14610e80578063a109f39a14610d6d578063a6b3e93d14610bbf578063ac1717b014610b98578063ae7ed49f14610b93578063b0464fdc14610ae9578063b1a63516146109ca578063b5508aa914610940578063b9edb1af14610919578063ba414fa6146108f4578063bbb9bf8514610770578063c12e468714610749578063cdc4a3be14610723578063d11187dd1461054e578063d48ca5841461040e578063e20c9f7114610380578063f09d1d441461026f578063f851a440146102485763fa7626d414610223575f80fd5b34610245578060031936011261024557602060ff601f54166040519015158152f35b80fd5b503461024557806003193601126102455760206001600160a01b0360225416604051908152f35b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d578180916004604051809481937f715018a60000000000000000000000000000000000000000000000000000000083525af1801561035d57610368575b5050600460206001600160a01b03601f5460081c16604051928380927f8da5cb5b0000000000000000000000000000000000000000000000000000000082525afa801561035d5761032b91839161032e575b506139f1565b80f35b610350915060203d602011610356575b6103488183613488565b8101906134ab565b5f610325565b503d61033e565b6040513d84823e3d90fd5b8161037291613488565b61024557805f6102d3565b50fd5b503461024557806003193601126102455760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106103ef576103eb856103df81870382613488565b60405191829182613177565b0390f35b82546001600160a01b03168452602090930192600192830192016103c8565b503461024557806003193601126102455780600460206001600160a01b03601f5460081c16604051928380927f2407f0b60000000000000000000000000000000000000000000000000000000082525afa90811561035d578291610515575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d57604051907f7c84c69b00000000000000000000000000000000000000000000000000000000825260048201527f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d50060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576105045750f35b8161050e91613488565b6102455780f35b9150506020813d602011610546575b8161053160209383613488565b81010312610542578190515f61046d565b5f80fd5b3d9150610524565b5034610245578060031936011261024557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d57604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d5761070e575b506001600160a01b0360235416604051907f118cdaa7000000000000000000000000000000000000000000000000000000006020830152602482015260248152610626604482613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d578161068191604051809381927ff28dceb30000000000000000000000000000000000000000000000000000000083526020600484015260248301906131b9565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d576106f9575b506001600160a01b03601f5460081c166001600160a01b0360215416813b156106f557829160248392604051948593849263d4f0eb4d60e01b845260048401525af1801561035d576105045750f35b5050fd5b8161070391613488565b61024557805f6106a6565b8161071891613488565b61024557805f6105db565b503461024557806003193601126102455760206001600160a01b03815416604051908152f35b503461024557806003193601126102455760206001600160a01b0360235416604051908152f35b5034610245576040600319360112610245576004356001600160a01b03811681036108f057602435906001600160a01b03821682036108ec57826001600160a01b03601f5460081c16803b156108f05781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d576108d7575b505061088d91602091604051916108478361083986820160609060208152600460208201527f746573740000000000000000000000000000000000000000000000000000000060408201520190565b03601f198101855284613488565b6001600160a01b03601f5460081c1691604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b03915afa801561035d5761032b9183916108a8575b50613afb565b6108ca915060203d6020116108d0575b6108c28183613488565b8101906134ca565b5f6108a2565b503d6108b8565b816108e191613488565b6108ec57825f6107ea565b8280fd5b5080fd5b5034610245578060031936011261024557602061090f613918565b6040519015158152f35b503461024557806003193601126102455760206001600160a01b0360255416604051908152f35b503461024557806003193601126102455760195461095d81613559565b9161096b6040519384613488565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b8383106109ad57604051806103eb8782613358565b6001602081926109bc85613571565b815201920192019190610998565b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d57610ad4575b505061088d6020604051610a7681610a6884820160609060208152600960208201527f746573742064617461000000000000000000000000000000000000000000000060408201520190565b03601f198101835282613488565b6001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b81610ade91613488565b61024557805f610a1c565b5034610245578060031936011261024557601c54610b0681613559565b91610b146040519384613488565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610b5657604051806103eb87826133d5565b60026020600192604051610b698161346c565b6001600160a01b038654168152610b81858701613674565b83820152815201920192019190610b41565b613233565b503461024557806003193601126102455760206001600160a01b0360245416604051908152f35b5034610245578060031936011261024557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561024557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152818180610c3960048201905f60606080840193600181528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d57610d58575b50604051817f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b98180a26001600160a01b03601f5460081c16803b156106f5578160248185809463d4f0eb4d60e01b83528160048401525af1801561035d57610d43575b5050600460206001600160a01b03601f5460081c16604051928380927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa90811561035d5761032b916001600160a01b03918491610d24575b50166139f1565b610d3d915060203d602011610356576103488183613488565b5f610d1d565b81610d4d91613488565b61024557805f610cc1565b81610d6291613488565b61024557805f610c5e565b50346102455780600319360112610245576001600160a01b03601f5460081c16604051907f8da5cb5b000000000000000000000000000000000000000000000000000000008252602082600481845afa908115610e7557610de76020926004948691610e58575b506001600160a01b036022541690613a7a565b604051928380927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa801561035d5761032b918391610e39575b506001600160a01b0380602054169116613a7a565b610e52915060203d602011610356576103488183613488565b5f610e24565b610e6f9150843d8611610356576103488183613488565b5f610dd4565b6040513d85823e3d90fd5b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d57610f47575b505061088d6020604051610ee68282613488565b8381526001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b81610f5191613488565b61024557805f610ed2565b5034610245578060031936011261024557601d54610f7981613559565b91610f876040519384613488565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310610fc957604051806103eb87826133d5565b60026020600192604051610fdc8161346c565b6001600160a01b038654168152610ff4858701613674565b83820152815201920192019190610fb4565b5034610245578060031936011261024557601a5461102381613559565b916110316040519384613488565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b83831061107357604051806103eb8782613358565b60016020819261108285613571565b81520192019201919061105e565b5034610245578060031936011261024557806001600160a01b03601f5460081c166001600160a01b0360215416813b156106f557829160248392604051948593849263d4f0eb4d60e01b845260048401525af1801561035d57611248575b506001600160a01b0360215416803b1561037d578180916024604051809481937f2dcfafd70000000000000000000000000000000000000000000000000000000083528160048401525af1801561035d57611233575b5061118f6020604051610a7681610a6884820160609060208152600960208201527f746573742064617461000000000000000000000000000000000000000000000060408201520190565b03915afa90811561035d578291611214575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d57604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576105045750f35b61122d915060203d6020116108d0576108c28183613488565b5f6111a1565b8161123d91613488565b61024557805f611144565b8161125291613488565b61024557805f6110ee565b5034610245578060031936011261024557806001600160a01b03601f5460081c166001600160a01b0360215416813b156106f557829160248392604051948593849263d4f0eb4d60e01b845260048401525af1801561035d5761135d575b506001600160a01b0360215416803b1561037d578180916024604051809481937f2dcfafd7000000000000000000000000000000000000000000000000000000008352600160048401525af1801561035d57610ad457505061088d6020604051610a7681610a6884820160609060208152600960208201527f746573742064617461000000000000000000000000000000000000000000000060408201520190565b8161136791613488565b61024557805f6112bb565b5034610245578060031936011261024557601b5461138f81613559565b61139c6040519182613488565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b83831061147457868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061140957505050500390f35b91936020611464827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc060019597998495030186528851908361145483516040845260408401906131b9565b92015190848184039101526131de565b96019201920185949391926113fa565b600260206001926040516114878161346c565b61149086613571565b815261149d858701613674565b838201528152019201920191906113cc565b503461024557806003193601126102455780600460206001600160a01b03601f5460081c16604051928380927fe8eb1dc30000000000000000000000000000000000000000000000000000000082525afa90811561035d578291611588575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d57604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015262030d4060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576105045750f35b90506020813d6020116115b2575b816115a360209383613488565b8101031261037d57515f61150e565b3d9150611596565b5034610245578060031936011261024557604051610a9f8082019082821067ffffffffffffffff8311176117e457908291614e4f8339039082f080156117d7576040516102bc8082019082821067ffffffffffffffff8311176117aa576020918391614b93833985815203019083f090811561035d576001600160a01b031690826001600160a01b038060245416921691833b156108f0576040517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201529083166024820152818160448183885af1801561035d57611795575b5050604051917f8da5cb5b000000000000000000000000000000000000000000000000000000008352602083600481845afa90811561178a57611700602092600495879161176d575b506001600160a01b036024541690613a7a565b604051938480927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa918215610e755761032b926001600160a01b0391859161174e575b5016613a7a565b611767915060203d602011610356576103488183613488565b5f611747565b6117849150843d8611610356576103488183613488565b5f6116ed565b6040513d86823e3d90fd5b8161179f91613488565b6108ec57825f6116a4565b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50604051903d90823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346102455780600319360112610245576001600160a01b036022541660405190611025908183019183831067ffffffffffffffff8411176117aa57918391602093613b6e8439815203019082f080156117d7576001600160a01b0316737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057816040517f491cc7c20000000000000000000000000000000000000000000000000000000081528181806118d760048201905f60606080840193600181528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d576119d7575b50604051827f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b98380a26001600160a01b03601f5460081c16803b156108ec578160248185809463d4f0eb4d60e01b83528860048401525af1801561035d576119c2575b505060049060206001600160a01b03601f5460081c16604051938480927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa918215610e755761032b926001600160a01b0391859161174e575016613a7a565b816119cc91613488565b6108f057815f61195f565b816119e191613488565b6108f057815f6118fc565b5034610245578060031936011261024557806001600160a01b03601f5460081c166040517f8da5cb5b000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610e7557611a5f918491611b4757506001600160a01b036022541690613a7a565b6001600160a01b0360245416813b156106f55782916024839260405194859384927ff2fde38b00000000000000000000000000000000000000000000000000000000845260048401525af1801561035d57611b32575b5050600460206001600160a01b03601f5460081c16604051928380927f8da5cb5b0000000000000000000000000000000000000000000000000000000082525afa801561035d5761032b918391611b1957506001600160a01b036024541690613a7a565b611784915060203d602011610356576103488183613488565b81611b3c91613488565b61024557805f611ab5565b610e6f915060203d602011610356576103488183613488565b5034610245578060031936011261024557806001600160a01b03601f5460081c166040517f5b3cd6e2000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610e7557611bd5918491610e3957506001600160a01b0380602054169116613a7a565b6001600160a01b0360215416813b156106f557829160248392604051948593849263d4f0eb4d60e01b845260048401525af1801561035d57611c98575b5050600460206001600160a01b03601f5460081c16604051928380927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa801561035d5761032b918391611c79575b506001600160a01b0380602154169116613a7a565b611c92915060203d602011610356576103488183613488565b5f611c64565b81611ca291613488565b61024557805f611c12565b50346102455780600319360112610245576001600160a01b036022541690604051611025928382019382851067ffffffffffffffff8611176117e45783946020928492613b6e8439815203019082f080156117d7576001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106f557604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610e75578391611eb1575b50506001600160a01b0360235416604051907f118cdaa7000000000000000000000000000000000000000000000000000000006020830152602482015260248152611dcc604482613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106f55782611e2791604051809381927ff28dceb30000000000000000000000000000000000000000000000000000000083526020600484015260248301906131b9565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610e75578391611e9c575b50506001600160a01b03601f5460081c1690813b156106f5576001600160a01b036024849283604051958694859363d4f0eb4d60e01b85521660048401525af1801561035d576105045750f35b81611ea691613488565b61037d57815f611e4f565b81611ebb91613488565b61037d57815f611d80565b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b83528160048401525af1801561035d57612072575b5050604051611f5f81610a686020820160609060208152600960208201527f746573742064617461000000000000000000000000000000000000000000000060408201520190565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057816040517ff4844814000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d5761205d575b5050602061202f916001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b03915afa801561035d57612041575080f35b6120599060203d6020116108d0576108c28183613488565b5080f35b8161206791613488565b6108f057815f611fc9565b8161207c91613488565b61024557805f611f17565b503461024557806003193601126102455760206001600160a01b03601f5460081c16604051908152f35b503461024557602060031936011261024557600435737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057816040517f4c63e56200000000000000000000000000000000000000000000000000000000815262030d4083111560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576121a1575b506001600160a01b03601f5460081c16803b156108f05781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d5761218c575b50506020610a7661088d92613528565b8161219691613488565b6108f057815f61217c565b816121ab91613488565b6108f057815f61213b565b503461024557806003193601126102455760206001600160a01b0360215416604051908152f35b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d576122b5575b505061088d602062030d60601f196040519161224b8184613488565b62030d3f83520136838301376001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b816122bf91613488565b61024557805f61222f565b503461024557806003193601126102455760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110612329576103eb856103df81870382613488565b82546001600160a01b0316845260209093019260019283019201612312565b503461024557806003193601126102455760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b8181106123a7576103eb856103df81870382613488565b82546001600160a01b0316845260209093019260019283019201612390565b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d5761249e575b505061088d602062030d60601f19604051916124348184613488565b62030d4083520136838301376001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b816124a891613488565b61024557805f612418565b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d57612658575b5050604051601f1962030d8061251b8184613488565b62030d41835201366020830137816040517f4634691b00000000000000000000000000000000000000000000000000000000602082015262030d41602482015262030d40604482015260448152612573606482613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057816125ce91604051809381927ff28dceb30000000000000000000000000000000000000000000000000000000083526020600484015260248301906131b9565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d5761205d575050602061202f916001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b8161266291613488565b61024557805f612505565b5034610245578060031936011261024557601e5461268a81613559565b6126976040519182613488565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106127d85786858760405192839260208401906020855251809152604084019160408260051b8601019392815b8383106127035786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b82811061278f575050505050602080600192970193019301909286959492936126f6565b90919293946020806127cb837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0876001960301895289516131b9565b970195019392910161276b565b6040516127e48161346c565b6001600160a01b03835416815260018301805461280081613559565b9161280e6040519384613488565b8183528a526020808b20908b9084015b8382106128445750505050600192826020928360029501528152019201920191906126c7565b60016020819261285386613571565b81520193019101909161281e565b5034610245576020600319360112610245576004358162030d408211806129b0575b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057604051907f4c63e562000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d5761299b575b506001600160a01b03601f5460081c16803b156108f05781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d57612986575b5061293e82613528565b91604051907f4634691b000000000000000000000000000000000000000000000000000000006020830152602482015262030d40604482015260448152612573606482613488565b8161299091613488565b6108f057815f612934565b816129a591613488565b6108f057815f6128f3565b50620f42408210612883565b503461024557806003193601126102455760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110612a1b576103eb856103df81870382613488565b82546001600160a01b0316845260209093019260019283019201612a04565b50346102455780600319360112610245576001600160a01b036022541690604051611025928382019382851067ffffffffffffffff8611176117e45783946020928492613b6e8439815203019082f080156117d7576001600160a01b03601f5460081c1690813b156106f5576001600160a01b036024849283604051958694859363d4f0eb4d60e01b85521660048401525af1801561035d57612b24575b505061088d6020604051610a7681610a6884820160609060208152600460208201527f746573740000000000000000000000000000000000000000000000000000000060408201520190565b81612b2e91613488565b61024557805f612ad8565b5034610245578060031936011261024557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561024557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152818180612bb360048201905f60606080840193600181528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d57612d40575b5060405160017f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b98380a26001600160a01b03601f5460081c16803b156106f5578160248185809463d4f0eb4d60e01b8352600160048401525af1801561035d57612d2b575b50600460206001600160a01b03601f5460081c16604051928380927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa90811561035d578291612d0c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d576001600160a01b03604051917f515361f6000000000000000000000000000000000000000000000000000000008352166004820152600160248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576105045750f35b612d25915060203d602011610356576103488183613488565b5f612c8b565b81612d3591613488565b61024557805f612c3d565b81612d4a91613488565b61024557805f612bd8565b5034610245578060031936011261024557307fffffffffffffffffffffffff000000000000000000000000000000000000000060225416176022556104567fffffffffffffffffffffffff000000000000000000000000000000000000000060235416176023556101237fffffffffffffffffffffffff000000000000000000000000000000000000000060245416176024556107897fffffffffffffffffffffffff000000000000000000000000000000000000000060255416176025556040516110258082019082821067ffffffffffffffff8311176117e4576020918391613b6e833930815203019082f080156117d7576001600160a01b03167fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556040516102bc8082019082821067ffffffffffffffff8311176117e4576020918391614b9383396001815203019082f080156117d7576001600160a01b03167fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215560405190610a9f918281019281841067ffffffffffffffff851117612fe75782938291614e4f8339039082f080156117d7576001600160a01b03907fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b1691161780601f5560081c166001600160a01b03602254166001600160a01b0360205416823b15612fe2576040517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b039283166004820152911660248201529082908290818381604481015b03925af1801561035d576105045750f35b505050fd5b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b905034610542575f600319360112610542577ff92ee8a90000000000000000000000000000000000000000000000000000000060208201526004815261305b602482613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610542575f6130b691604051809381927ff28dceb30000000000000000000000000000000000000000000000000000000083526020600484015260248301906131b9565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561316c57613159575b50806001600160a01b03601f5460081c166001600160a01b03602254166001600160a01b0360205416823b15612fe2576040517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b03928316600482015291166024820152908290829081838160448101612fd1565b61316591505f90613488565b5f5f6130db565b6040513d5f823e3d90fd5b60206040818301928281528451809452019201905f5b81811061319a5750505090565b82516001600160a01b031684526020938401939092019160010161318d565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b8181106131fb5750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016131ee565b34610542575f5f600319360112610542576001600160a01b03602254166040519061108f80830183811067ffffffffffffffff82111761332b5760209284926158ee843981520301905ff0801561316c576001600160a01b03601f5460081c1690813b15610542576001600160a01b0360245f9283604051958694859363d4f0eb4d60e01b85521660048401525af1801561316c57613318575b5061088d6020604051610a7681610a6884820160609060208152600460208201527f746573740000000000000000000000000000000000000000000000000000000060408201520190565b61332491505f90613488565b5f5f6132cd565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061338a57505050505090565b90919293946020806133c6837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0866001960301875289516131b9565b9701930193019193929061337b565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061340757505050505090565b909192939460208061345d837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906131de565b970193019301919392906133f8565b6040810190811067ffffffffffffffff82111761332b57604052565b90601f601f19910116810190811067ffffffffffffffff82111761332b57604052565b9081602091031261054257516001600160a01b03811681036105425790565b90816020910312610542575180151581036105425790565b6001600160a01b0361350994938160609416835216602082015281604082015201906131b9565b90565b67ffffffffffffffff811161332b57601f01601f191660200190565b906135328261350c565b61353f6040519182613488565b828152601f1961354f829461350c565b0190602036910137565b67ffffffffffffffff811161332b5760051b60200190565b90604051915f8154908160011c926001831692831561366a575b60208510841461363d5784875286939081156135fd57506001146135b9575b506135b792500383613488565b565b90505f9291925260205f20905f915b8183106135e15750509060206135b7928201015f6135aa565b60209193508060019154838589010152019101909184926135c8565b602093506135b79592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f6135aa565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f169361358b565b90604051918281549182825260208201905f5260205f20925f905b80600783011061388b576135b7945491818110613855575b81811061381f575b8181106137e9575b8181106137b3575b81811061377d575b818110613747575b818110613712575b106136e5575b500383613488565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f6136dd565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b1681520193016136d7565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b1681520193016136cf565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b1681520193016136c7565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b1681520193016136bf565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b1681520193016136b7565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b1681520193016136af565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b1681520193016136a7565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e082015201940192018592939161368f565b60085460ff1680156139275790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561316c575f916139bf575b50151590565b90506020813d6020116139e9575b816139da60209383613488565b8101031261054257515f6139b9565b3d91506139cd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610542576001600160a01b03604051917f515361f60000000000000000000000000000000000000000000000000000000083521660048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561316c57613a705750565b5f6135b791613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610542576040517f515361f60000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152911660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561316c57613a705750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561054257604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561316c57613a70575056fe60803460b857601f61102538819003918201601f19168301916001600160401b0383118484101760bc5780849260209460405283398101031260b857516001600160a01b0381169081900360b857801560a5575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3610f5490816100d18239f35b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304f386f4146107a4578063052eefd1146106235780631b42c71114610407578063715018a61461038b5780637a3979dc146101905780638da5cb5b1461015e578063a26b4a88146101435763f2fde38b14610071575f80fd5b3461013f57602060031936011261013f5773ffffffffffffffffffffffffffffffffffffffff61009f6108c2565b6100a76109d4565b1680156101135773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461013f575f60031936011261013f57602060405160288152f35b3461013f575f60031936011261013f57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013f57606060031936011261013f576101a96108c2565b60243573ffffffffffffffffffffffffffffffffffffffff8116810361013f5760443567ffffffffffffffff811161013f573660238201121561013f5780600401359067ffffffffffffffff821161013f576024810190602483369201011161013f5760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b73ffffffffffffffffffffffffffffffffffffffff81168015610380576040517f7a3979dc00000000000000000000000000000000000000000000000000000000815290602090829081806102c889898c8e6004860161096b565b03915afa908115610375575f9161033b575b50156102ff576102e990610d0a565b9061026d5750505050505b602060405160018152f35b6103378386936040519485947f79a132500000000000000000000000000000000000000000000000000000000086526004860161096b565b0390fd5b90506020813d821161036d575b81610355602093836108e5565b8101031261013f5751801515810361013f57866102da565b3d9150610348565b6040513d5f823e3d90fd5b5050505050506102f4565b3461013f575f60031936011261013f576103a36109d4565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461013f575f60031936011261013f5760015461042381610953565b61043060405191826108e5565b81815261043c82610953565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe060208201920136833760015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b84821080610604575b156105fa5782518210156105cd578073ffffffffffffffffffffffffffffffffffffffff61050b921660208460051b86010152610d0a565b901561056f57907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811461054257600101906104ca565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b50509091505b604051918291602083019060208452518091526040830191905f5b81811061059e575050500390f35b825173ffffffffffffffffffffffffffffffffffffffff16845285945060209384019390920191600101610590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5050909150610575565b5073ffffffffffffffffffffffffffffffffffffffff811615156104d3565b3461013f57604060031936011261013f5761063c6108c2565b60243590811515820361013f576106516109d4565b73ffffffffffffffffffffffffffffffffffffffff811691821561077c5761067882610a20565b610754576028600154101561072c571561071e5761069590610e6b565b156106c0577f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e75f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f41646472657373206e6f742061646465640000000000000000000000000000006044820152fd5b61072790610d6b565b610695565b7f13d867a2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa2d86a1e000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe6c4247b000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461013f57602060031936011261013f576107bd6108c2565b6107c56109d4565b73ffffffffffffffffffffffffffffffffffffffff811690811561077c576107ec81610a20565b1561089a5773ffffffffffffffffffffffffffffffffffffffff6108108392610bf5565b160361083c577fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be5f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f41646472657373206e6f742072656d6f766564000000000000000000000000006044820152fd5b7f3d0f293d000000000000000000000000000000000000000000000000000000005f5260045ffd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361013f57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761092657604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff81116109265760051b60200190565b92938060809573ffffffffffffffffffffffffffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe09581601f9616885216602087015260606040870152816060870152868601375f8582860101520116010190565b73ffffffffffffffffffffffffffffffffffffffff5f541633036109f457565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff16805f52600260205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610ae3575b15610add5760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff1603610ad957600190565b5f90565b50600190565b50805f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610a6a565b60010173ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610bab575b15610ba4575f805260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2054169116145f14610ad957600190565b5050600190565b5073ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610b64565b73ffffffffffffffffffffffffffffffffffffffff811680158015610cf8575b610cf2575f90815260026020818152604080842084805280835281852080546001808852848820805473ffffffffffffffffffffffffffffffffffffffff908116808b52898952878b208b80528952878b208054929095167fffffffffffffffffffffffff00000000000000000000000000000000000000009283168117909555938a52978752858920828a5287529490972080548716909117905580548516905590915280549091169055547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116105425760015590565b50505f90565b50610d04826001610b18565b15610c15565b610d15816001610b18565b610d2057505f905f90565b73ffffffffffffffffffffffffffffffffffffffff165f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f205416908115159190565b610d76816001610b18565b1580610e5a575b610d8657505f90565b7f6ee3efecae883df2d7ccda22610b4ca771a299e707cb0d65c4ec97dc4e6668ad805473ffffffffffffffffffffffffffffffffffffffff9283165f818152600260208181526040808420600180865281845282862080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811690915589548116881790995598909616808552928252808420978452968152868320805487169094179093558180529290915292909220805490911690911790555b6001546001810180911161054257600155600190565b50610e665f6001610b18565b610d7d565b610e76816001610b18565b1580610f43575b610e8657505f90565b7f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d805473ffffffffffffffffffffffffffffffffffffffff9283165f81815260026020818152604080842084805280835281852080547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091558854811687179098559790951680845291815284832083805281528483208054871690941790935560018252949091522080549091169091179055610e44565b50610f4f5f6001610b18565b610e7d56608034606057601f6102bc38819003918201601f19168301916001600160401b038311848410176064578084926020946040528339810103126060575180151580910360605760ff80195f54169116175f5560405161024390816100798239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c9081632dcfafd7146101925781637a3979dc14610080575063a57b02a61461003d575f80fd5b3461007c575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c57602060ff5f54166040519015158152f35b5f80fd5b3461007c5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c576100b76101fd565b506100c0610220565b5060443567ffffffffffffffff811161007c573660238201121561007c57806004013567ffffffffffffffff8111610165577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f81601f8401160116830183811067ffffffffffffffff82111761016557604052808352366024828401011161007c575f928160246020940184830137010152602060ff5f54166040519015158152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b3461007c5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c5760043580151580910361007c5760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff005f54169116175f555f80f35b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361007c57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361007c575660808060405234601557610a85908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c9081632407f0b61461081657508063485cc9551461055b5780635b3cd6e214610509578063715018a61461044d5780637a3979dc146101e25780638da5cb5b14610190578063d4f0eb4d146100cb578063e8eb1dc3146100ae5763f2fde38b1461007f575f80fd5b346100aa5760206003193601126100aa576100a861009b61084e565b6100a36109c2565b6108d5565b005b5f80fd5b346100aa575f6003193601126100aa57602060405162030d408152f35b346100aa5760206003193601126100aa5773ffffffffffffffffffffffffffffffffffffffff6100f961084e565b6101016109c2565b16807fffffffffffffffffffffffff00000000000000000000000000000000000000007f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d5005416177f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d500557f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b95f80a2005b346100aa575f6003193601126100aa57602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b346100aa5760606003193601126100aa576101fb61084e565b610203610871565b6044359067ffffffffffffffff82116100aa57366023830112156100aa5781600401359167ffffffffffffffff8311610420576040519261026c60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8401160185610894565b808452602084019136602483830101116100aa57815f92602460209301853785010152825162030d4081116103ee575073ffffffffffffffffffffffffffffffffffffffff7f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d500541690600182149384156102ee575b6020856040519015158152f35b6020945073ffffffffffffffffffffffffffffffffffffffff957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f88946084946040519a8b998a9889977f7a3979dc000000000000000000000000000000000000000000000000000000008952166004880152166024860152606060448601525180918160648701528686015e5f858286010152011681010301915afa80156103e3575f906103a7575b60209150828080806102e1565b506020813d6020116103db575b816103c160209383610894565b810103126100aa575180151581036100aa5760209061039a565b3d91506103b4565b6040513d5f823e3d90fd5b7f4634691b000000000000000000000000000000000000000000000000000000005f5260045262030d4060245260445ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b346100aa575f6003193601126100aa576104656109c2565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346100aa575f6003193601126100aa57602073ffffffffffffffffffffffffffffffffffffffff7f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d5005416604051908152f35b346100aa5760406003193601126100aa5761057461084e565b61057c610871565b907ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549160ff8360401c16159267ffffffffffffffff81168015908161080e575b6001149081610804575b1590816107fb575b506107d35773ffffffffffffffffffffffffffffffffffffffff92818560017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000006106559516177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005561077e575b50610645610a2e565b61064d610a2e565b6100a3610a2e565b1690817fffffffffffffffffffffffff00000000000000000000000000000000000000007f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d5005416177f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d50055604051917f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b95f80a26106ed57005b60207fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2917fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005560018152a1005b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00558561063c565b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b905015856105cf565b303b1591506105c7565b8591506105bd565b346100aa575f6003193601126100aa57807f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d50060209252f35b6004359073ffffffffffffffffffffffffffffffffffffffff821682036100aa57565b6024359073ffffffffffffffffffffffffffffffffffffffff821682036100aa57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761042057604052565b73ffffffffffffffffffffffffffffffffffffffff1680156109965773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054163303610a0257565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c1615610a5d57565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd60803460b857601f61108f38819003918201601f19168301916001600160401b0383118484101760bc5780849260209460405283398101031260b857516001600160a01b0381169081900360b857801560a5575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3610fbe90816100d18239f35b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304f386f41461063c578063052eefd1146104bb5780631b42c7111461029f578063715018a6146102235780637a3979dc146101905780638da5cb5b1461015e578063a26b4a88146101435763f2fde38b14610071575f80fd5b3461013f57602060031936011261013f5773ffffffffffffffffffffffffffffffffffffffff61009f61075a565b6100a7610a3e565b1680156101135773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461013f575f60031936011261013f57602060405160288152f35b3461013f575f60031936011261013f57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013f57606060031936011261013f576101a961075a565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361013f576044359067ffffffffffffffff821161013f573660238301121561013f5781600401359067ffffffffffffffff821161013f57366024838501011161013f576020936024610219940191610841565b6040519015158152f35b3461013f575f60031936011261013f5761023b610a3e565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461013f575f60031936011261013f576001546102bb816107eb565b6102c8604051918261077d565b8181526102d4826107eb565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe060208201920136833760015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b8482108061049c575b15610492578251821015610465578073ffffffffffffffffffffffffffffffffffffffff6103a3921660208460051b86010152610d74565b901561040757907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81146103da5760010190610362565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b50509091505b604051918291602083019060208452518091526040830191905f5b818110610436575050500390f35b825173ffffffffffffffffffffffffffffffffffffffff16845285945060209384019390920191600101610428565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b505090915061040d565b5073ffffffffffffffffffffffffffffffffffffffff8116151561036b565b3461013f57604060031936011261013f576104d461075a565b60243590811515820361013f576104e9610a3e565b73ffffffffffffffffffffffffffffffffffffffff81169182156106145761051082610a8a565b6105ec57602860015410156105c457156105b65761052d90610ed5565b15610558577f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e75f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f41646472657373206e6f742061646465640000000000000000000000000000006044820152fd5b6105bf90610dd5565b61052d565b7f13d867a2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa2d86a1e000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe6c4247b000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461013f57602060031936011261013f5761065561075a565b61065d610a3e565b73ffffffffffffffffffffffffffffffffffffffff81169081156106145761068481610a8a565b156107325773ffffffffffffffffffffffffffffffffffffffff6106a88392610c5f565b16036106d4577fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be5f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f41646472657373206e6f742072656d6f766564000000000000000000000000006044820152fd5b7f3d0f293d000000000000000000000000000000000000000000000000000000005f5260045ffd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361013f57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176107be57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff81116107be5760051b60200190565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b60015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d549394909373ffffffffffffffffffffffffffffffffffffffff169182156109cb57915b73ffffffffffffffffffffffffffffffffffffffff81168015610a1b57602060405180927f7a3979dc00000000000000000000000000000000000000000000000000000000825273ffffffffffffffffffffffffffffffffffffffff8916600483015273ffffffffffffffffffffffffffffffffffffffff87166024830152606060448301528180610944606482018d8c610803565b03915afa908115610a10575f916109d6575b506109cb5761096490610d74565b906108ae575050506109c79073ffffffffffffffffffffffffffffffffffffffff935b6040519485947f0200da48000000000000000000000000000000000000000000000000000000008652166004850152604060248501526044840191610803565b0390fd5b509350505050600190565b90506020813d8211610a08575b816109f06020938361077d565b8101031261013f5751801515810361013f575f610956565b3d91506109e3565b6040513d5f823e3d90fd5b505050506109c79073ffffffffffffffffffffffffffffffffffffffff93610987565b73ffffffffffffffffffffffffffffffffffffffff5f54163303610a5e57565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff16805f52600260205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610b4d575b15610b475760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff1603610b4357600190565b5f90565b50600190565b50805f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610ad4565b60010173ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610c15575b15610c0e575f805260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2054169116145f14610b4357600190565b5050600190565b5073ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610bce565b73ffffffffffffffffffffffffffffffffffffffff811680158015610d62575b610d5c575f90815260026020818152604080842084805280835281852080546001808852848820805473ffffffffffffffffffffffffffffffffffffffff908116808b52898952878b208b80528952878b208054929095167fffffffffffffffffffffffff00000000000000000000000000000000000000009283168117909555938a52978752858920828a5287529490972080548716909117905580548516905590915280549091169055547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116103da5760015590565b50505f90565b50610d6e826001610b82565b15610c7f565b610d7f816001610b82565b610d8a57505f905f90565b73ffffffffffffffffffffffffffffffffffffffff165f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f205416908115159190565b610de0816001610b82565b1580610ec4575b610df057505f90565b7f6ee3efecae883df2d7ccda22610b4ca771a299e707cb0d65c4ec97dc4e6668ad805473ffffffffffffffffffffffffffffffffffffffff9283165f818152600260208181526040808420600180865281845282862080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811690915589548116881790995598909616808552928252808420978452968152868320805487169094179093558180529290915292909220805490911690911790555b600154600181018091116103da57600155600190565b50610ed05f6001610b82565b610de7565b610ee0816001610b82565b1580610fad575b610ef057505f90565b7f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d805473ffffffffffffffffffffffffffffffffffffffff9283165f81815260026020818152604080842084805280835281852080547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091558854811687179098559790951680845291815284832083805281528483208054871690941790935560018252949091522080549091169091179055610eae565b50610fb95f6001610b82565b610ee756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`/W`\x01`\xFF\x19`\x0CT\x16\x17`\x0CU`\x01`\xFF\x19`\x1FT\x16\x17`\x1FUai}\x90\x81a\x004\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x08\x88z\xEA\x14a0\x14WP\x80c\n\x92T\xE4\x14a-UW\x80c\x146\xEE\x7F\x14a+9W\x80c\x19,(\xAF\x14a*:W\x80c\x1E\xD7\x83\x1C\x14a)\xBCW\x80c*\xBCP\xBF\x14a(aW\x80c*\xDE8\x80\x14a&mW\x80c.\xCD\xBFV\x14a$\xB3W\x80c1\xC9V\xA4\x14a#\xC6W\x80c>^<#\x14a#HW\x80c?r\x86\xF4\x14a\"\xCAW\x80cB\xA8Z\xF5\x14a!\xDDW\x80cCY\xA7N\x14a!\xB6W\x80cDv\x8D\xA3\x14a \xB1W\x80cH\x1Cju\x14a \x87W\x80cLX\xEF{\x14a\x1E\xC6W\x80cQ\xD9\xD3\xD5\x14a\x1C\xADW\x80cS\x87Gf\x14a\x1B`W\x80cW_\xC5\xD5\x14a\x19\xECW\x80c^^_i\x14a\x18\x11W\x80cbm\x07f\x14a\x15\xBAW\x80cb\x9F\xE61\x14a\x14\xAFW\x80cf\xD9\xA9\xA0\x14a\x13rW\x80cp\x04\xEA\xC4\x14a\x12]W\x80cqn*\xA8\x14a\x0B\x93W\x80c\x82\xED\xDC\"\x14a\x10\x90W\x80c\x85\"l\x81\x14a\x10\x06W\x80c\x91j\x17\xC6\x14a\x0F\\W\x80c\x9DJ\xA9\x1C\x14a\x0E\x80W\x80c\xA1\t\xF3\x9A\x14a\rmW\x80c\xA6\xB3\xE9=\x14a\x0B\xBFW\x80c\xAC\x17\x17\xB0\x14a\x0B\x98W\x80c\xAE~\xD4\x9F\x14a\x0B\x93W\x80c\xB0FO\xDC\x14a\n\xE9W\x80c\xB1\xA65\x16\x14a\t\xCAW\x80c\xB5P\x8A\xA9\x14a\t@W\x80c\xB9\xED\xB1\xAF\x14a\t\x19W\x80c\xBAAO\xA6\x14a\x08\xF4W\x80c\xBB\xB9\xBF\x85\x14a\x07pW\x80c\xC1.F\x87\x14a\x07IW\x80c\xCD\xC4\xA3\xBE\x14a\x07#W\x80c\xD1\x11\x87\xDD\x14a\x05NW\x80c\xD4\x8C\xA5\x84\x14a\x04\x0EW\x80c\xE2\x0C\x9Fq\x14a\x03\x80W\x80c\xF0\x9D\x1DD\x14a\x02oW\x80c\xF8Q\xA4@\x14a\x02HWc\xFAv&\xD4\x14a\x02#W_\x80\xFD[4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7FqP\x18\xA6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x03]Wa\x03hW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x03.W[Pa9\xF1V[\x80\xF3[a\x03P\x91P` =` \x11a\x03VW[a\x03H\x81\x83a4\x88V[\x81\x01\x90a4\xABV[_a\x03%V[P=a\x03>V[`@Q=\x84\x82>=\x90\xFD[\x81a\x03r\x91a4\x88V[a\x02EW\x80_a\x02\xD3V[P\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x03\xEFWa\x03\xEB\x85a\x03\xDF\x81\x87\x03\x82a4\x88V[`@Q\x91\x82\x91\x82a1wV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x03\xC8V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F$\x07\xF0\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03]W\x82\x91a\x05\x15W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`@Q\x90\x7F|\x84\xC6\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa\x05\x04WP\xF3[\x81a\x05\x0E\x91a4\x88V[a\x02EW\x80\xF3[\x91PP` \x81=` \x11a\x05FW[\x81a\x051` \x93\x83a4\x88V[\x81\x01\x03\x12a\x05BW\x81\x90Q_a\x04mV[_\x80\xFD[=\x91Pa\x05$V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa\x07\x0EW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra\x06&`D\x82a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W\x81a\x06\x81\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a1\xB9V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa\x06\xF9W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c\xD4\xF0\xEBM`\xE0\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x05\x04WP\xF3[PP\xFD[\x81a\x07\x03\x91a4\x88V[a\x02EW\x80_a\x06\xA6V[\x81a\x07\x18\x91a4\x88V[a\x02EW\x80_a\x05\xDBV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x02EW`@`\x03\x196\x01\x12a\x02EW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x08\xF0W`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x08\xECW\x82`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x08\xF0W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x08\xD7W[PPa\x08\x8D\x91` \x91`@Q\x91a\x08G\x83a\x089\x86\x82\x01``\x90` \x81R`\x04` \x82\x01R\x7Ftest\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x85R\x84a4\x88V[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x03\x91Z\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x08\xA8W[Pa:\xFBV[a\x08\xCA\x91P` =` \x11a\x08\xD0W[a\x08\xC2\x81\x83a4\x88V[\x81\x01\x90a4\xCAV[_a\x08\xA2V[P=a\x08\xB8V[\x81a\x08\xE1\x91a4\x88V[a\x08\xECW\x82_a\x07\xEAV[\x82\x80\xFD[P\x80\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` a\t\x0Fa9\x18V[`@Q\x90\x15\x15\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x19Ta\t]\x81a5YV[\x91a\tk`@Q\x93\x84a4\x88V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\t\xADW`@Q\x80a\x03\xEB\x87\x82a3XV[`\x01` \x81\x92a\t\xBC\x85a5qV[\x81R\x01\x92\x01\x92\x01\x91\x90a\t\x98V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\n\xD4W[PPa\x08\x8D` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\t` \x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a4\x88V[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a\n\xDE\x91a4\x88V[a\x02EW\x80_a\n\x1CV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1CTa\x0B\x06\x81a5YV[\x91a\x0B\x14`@Q\x93\x84a4\x88V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x0BVW`@Q\x80a\x03\xEB\x87\x82a3\xD5V[`\x02` `\x01\x92`@Qa\x0Bi\x81a4lV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x0B\x81\x85\x87\x01a6tV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0BAV[a23V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02EW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\x0C9`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa\rXW[P`@Q\x81\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x81\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06\xF5W\x81`$\x81\x85\x80\x94c\xD4\xF0\xEBM`\xE0\x1B\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\rCW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03]Wa\x03+\x91`\x01`\x01`\xA0\x1B\x03\x91\x84\x91a\r$W[P\x16a9\xF1V[a\r=\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a\r\x1DV[\x81a\rM\x91a4\x88V[a\x02EW\x80_a\x0C\xC1V[\x81a\rb\x91a4\x88V[a\x02EW\x80_a\x0C^V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x90\x81\x15a\x0EuWa\r\xE7` \x92`\x04\x94\x86\x91a\x0EXW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90a:zV[`@Q\x92\x83\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x0E9W[P`\x01`\x01`\xA0\x1B\x03\x80` T\x16\x91\x16a:zV[a\x0ER\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a\x0E$V[a\x0Eo\x91P\x84=\x86\x11a\x03VWa\x03H\x81\x83a4\x88V[_a\r\xD4V[`@Q=\x85\x82>=\x90\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x0FGW[PPa\x08\x8D` `@Qa\x0E\xE6\x82\x82a4\x88V[\x83\x81R`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a\x0FQ\x91a4\x88V[a\x02EW\x80_a\x0E\xD2V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1DTa\x0Fy\x81a5YV[\x91a\x0F\x87`@Q\x93\x84a4\x88V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x0F\xC9W`@Q\x80a\x03\xEB\x87\x82a3\xD5V[`\x02` `\x01\x92`@Qa\x0F\xDC\x81a4lV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x0F\xF4\x85\x87\x01a6tV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0F\xB4V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1ATa\x10#\x81a5YV[\x91a\x101`@Q\x93\x84a4\x88V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x10sW`@Q\x80a\x03\xEB\x87\x82a3XV[`\x01` \x81\x92a\x10\x82\x85a5qV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x10^V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c\xD4\xF0\xEBM`\xE0\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x12HW[P`\x01`\x01`\xA0\x1B\x03`!T\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x123W[Pa\x11\x8F` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\t` \x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x03\x91Z\xFA\x90\x81\x15a\x03]W\x82\x91a\x12\x14W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa\x05\x04WP\xF3[a\x12-\x91P` =` \x11a\x08\xD0Wa\x08\xC2\x81\x83a4\x88V[_a\x11\xA1V[\x81a\x12=\x91a4\x88V[a\x02EW\x80_a\x11DV[\x81a\x12R\x91a4\x88V[a\x02EW\x80_a\x10\xEEV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c\xD4\xF0\xEBM`\xE0\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x13]W[P`\x01`\x01`\xA0\x1B\x03`!T\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\n\xD4WPPa\x08\x8D` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\t` \x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x81a\x13g\x91a4\x88V[a\x02EW\x80_a\x12\xBBV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1BTa\x13\x8F\x81a5YV[a\x13\x9C`@Q\x91\x82a4\x88V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x14tW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x14\tWPPPP\x03\x90\xF3[\x91\x93` a\x14d\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x14T\x83Q`@\x84R`@\x84\x01\x90a1\xB9V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra1\xDEV[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x13\xFAV[`\x02` `\x01\x92`@Qa\x14\x87\x81a4lV[a\x14\x90\x86a5qV[\x81Ra\x14\x9D\x85\x87\x01a6tV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x13\xCCV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xE8\xEB\x1D\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03]W\x82\x91a\x15\x88W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rb\x03\r@`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa\x05\x04WP\xF3[\x90P` \x81=` \x11a\x15\xB2W[\x81a\x15\xA3` \x93\x83a4\x88V[\x81\x01\x03\x12a\x03}WQ_a\x15\x0EV[=\x91Pa\x15\x96V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Qa\n\x9F\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x17\xE4W\x90\x82\x91aNO\x839\x03\x90\x82\xF0\x80\x15a\x17\xD7W`@Qa\x02\xBC\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x17\xAAW` \x91\x83\x91aK\x93\x839\x85\x81R\x03\x01\x90\x83\xF0\x90\x81\x15a\x03]W`\x01`\x01`\xA0\x1B\x03\x16\x90\x82`\x01`\x01`\xA0\x1B\x03\x80`$T\x16\x92\x16\x91\x83;\x15a\x08\xF0W`@Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R\x81\x81`D\x81\x83\x88Z\xF1\x80\x15a\x03]Wa\x17\x95W[PP`@Q\x91\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` \x83`\x04\x81\x84Z\xFA\x90\x81\x15a\x17\x8AWa\x17\0` \x92`\x04\x95\x87\x91a\x17mW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x90a:zV[`@Q\x93\x84\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x0EuWa\x03+\x92`\x01`\x01`\xA0\x1B\x03\x91\x85\x91a\x17NW[P\x16a:zV[a\x17g\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a\x17GV[a\x17\x84\x91P\x84=\x86\x11a\x03VWa\x03H\x81\x83a4\x88V[_a\x16\xEDV[`@Q=\x86\x82>=\x90\xFD[\x81a\x17\x9F\x91a4\x88V[a\x08\xECW\x82_a\x16\xA4V[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90a\x10%\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x17\xAAW\x91\x83\x91` \x93a;n\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W\x81`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\x18\xD7`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa\x19\xD7W[P`@Q\x82\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x83\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x08\xECW\x81`$\x81\x85\x80\x94c\xD4\xF0\xEBM`\xE0\x1B\x83R\x88`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x19\xC2W[PP`\x04\x90` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x0EuWa\x03+\x92`\x01`\x01`\xA0\x1B\x03\x91\x85\x91a\x17NWP\x16a:zV[\x81a\x19\xCC\x91a4\x88V[a\x08\xF0W\x81_a\x19_V[\x81a\x19\xE1\x91a4\x88V[a\x08\xF0W\x81_a\x18\xFCV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x0EuWa\x1A_\x91\x84\x91a\x1BGWP`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90a:zV[`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x1B2W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x1B\x19WP`\x01`\x01`\xA0\x1B\x03`$T\x16\x90a:zV[a\x17\x84\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[\x81a\x1B<\x91a4\x88V[a\x02EW\x80_a\x1A\xB5V[a\x0Eo\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x0EuWa\x1B\xD5\x91\x84\x91a\x0E9WP`\x01`\x01`\xA0\x1B\x03\x80` T\x16\x91\x16a:zV[`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c\xD4\xF0\xEBM`\xE0\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x1C\x98W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x1CyW[P`\x01`\x01`\xA0\x1B\x03\x80`!T\x16\x91\x16a:zV[a\x1C\x92\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a\x1CdV[\x81a\x1C\xA2\x91a4\x88V[a\x02EW\x80_a\x1C\x12V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Qa\x10%\x92\x83\x82\x01\x93\x82\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x17\xE4W\x83\x94` \x92\x84\x92a;n\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\xF5W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0EuW\x83\x91a\x1E\xB1W[PP`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra\x1D\xCC`D\x82a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\xF5W\x82a\x1E'\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a1\xB9V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0EuW\x83\x91a\x1E\x9CW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x06\xF5W`\x01`\x01`\xA0\x1B\x03`$\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\xD4\xF0\xEBM`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x05\x04WP\xF3[\x81a\x1E\xA6\x91a4\x88V[a\x03}W\x81_a\x1EOV[\x81a\x1E\xBB\x91a4\x88V[a\x03}W\x81_a\x1D\x80V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa rW[PP`@Qa\x1F_\x81a\nh` \x82\x01``\x90` \x81R`\t` \x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W\x81`@Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa ]W[PP` a /\x91`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x03\x91Z\xFA\x80\x15a\x03]Wa AWP\x80\xF3[a Y\x90` =` \x11a\x08\xD0Wa\x08\xC2\x81\x83a4\x88V[P\x80\xF3[\x81a g\x91a4\x88V[a\x08\xF0W\x81_a\x1F\xC9V[\x81a |\x91a4\x88V[a\x02EW\x80_a\x1F\x17V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x02EW` `\x03\x196\x01\x12a\x02EW`\x045sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W\x81`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rb\x03\r@\x83\x11\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa!\xA1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x08\xF0W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa!\x8CW[PP` a\nva\x08\x8D\x92a5(V[\x81a!\x96\x91a4\x88V[a\x08\xF0W\x81_a!|V[\x81a!\xAB\x91a4\x88V[a\x08\xF0W\x81_a!;V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\"\xB5W[PPa\x08\x8D` b\x03\r``\x1F\x19`@Q\x91a\"K\x81\x84a4\x88V[b\x03\r?\x83R\x016\x83\x83\x017`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a\"\xBF\x91a4\x88V[a\x02EW\x80_a\"/V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a#)Wa\x03\xEB\x85a\x03\xDF\x81\x87\x03\x82a4\x88V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a#\x12V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a#\xA7Wa\x03\xEB\x85a\x03\xDF\x81\x87\x03\x82a4\x88V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a#\x90V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa$\x9EW[PPa\x08\x8D` b\x03\r``\x1F\x19`@Q\x91a$4\x81\x84a4\x88V[b\x03\r@\x83R\x016\x83\x83\x017`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a$\xA8\x91a4\x88V[a\x02EW\x80_a$\x18V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa&XW[PP`@Q`\x1F\x19b\x03\r\x80a%\x1B\x81\x84a4\x88V[b\x03\rA\x83R\x016` \x83\x017\x81`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Rb\x03\rA`$\x82\x01Rb\x03\r@`D\x82\x01R`D\x81Ra%s`d\x82a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W\x81a%\xCE\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a1\xB9V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa ]WPP` a /\x91`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a&b\x91a4\x88V[a\x02EW\x80_a%\x05V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1ETa&\x8A\x81a5YV[a&\x97`@Q\x91\x82a4\x88V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a'\xD8W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a'\x03W\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a'\x8FWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a&\xF6V[\x90\x91\x92\x93\x94` \x80a'\xCB\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa1\xB9V[\x97\x01\x95\x01\x93\x92\x91\x01a'kV[`@Qa'\xE4\x81a4lV[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta(\0\x81a5YV[\x91a(\x0E`@Q\x93\x84a4\x88V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a(DWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a&\xC7V[`\x01` \x81\x92a(S\x86a5qV[\x81R\x01\x93\x01\x91\x01\x90\x91a(\x1EV[P4a\x02EW` `\x03\x196\x01\x12a\x02EW`\x045\x81b\x03\r@\x82\x11\x80a)\xB0W[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W`@Q\x90\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa)\x9BW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x08\xF0W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa)\x86W[Pa)>\x82a5(V[\x91`@Q\x90\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01Rb\x03\r@`D\x82\x01R`D\x81Ra%s`d\x82a4\x88V[\x81a)\x90\x91a4\x88V[a\x08\xF0W\x81_a)4V[\x81a)\xA5\x91a4\x88V[a\x08\xF0W\x81_a(\xF3V[Pb\x0FB@\x82\x10a(\x83V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a*\x1BWa\x03\xEB\x85a\x03\xDF\x81\x87\x03\x82a4\x88V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a*\x04V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Qa\x10%\x92\x83\x82\x01\x93\x82\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x17\xE4W\x83\x94` \x92\x84\x92a;n\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x06\xF5W`\x01`\x01`\xA0\x1B\x03`$\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\xD4\xF0\xEBM`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa+$W[PPa\x08\x8D` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\x04` \x82\x01R\x7Ftest\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x81a+.\x91a4\x88V[a\x02EW\x80_a*\xD8V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02EW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a+\xB3`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa-@W[P`@Q`\x01\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x83\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06\xF5W\x81`$\x81\x85\x80\x94c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa-+W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03]W\x82\x91a-\x0CW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`\x01`\x01`\xA0\x1B\x03`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01R`\x01`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa\x05\x04WP\xF3[a-%\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a,\x8BV[\x81a-5\x91a4\x88V[a\x02EW\x80_a,=V[\x81a-J\x91a4\x88V[a\x02EW\x80_a+\xD8V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"Ua\x04V\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`#T\x16\x17`#Ua\x01#\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$T\x16\x17`$Ua\x07\x89\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`%T\x16\x17`%U`@Qa\x10%\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x17\xE4W` \x91\x83\x91a;n\x8390\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`@Qa\x02\xBC\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x17\xE4W` \x91\x83\x91aK\x93\x839`\x01\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`@Q\x90a\n\x9F\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a/\xE7W\x82\x93\x82\x91aNO\x839\x03\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17\x80`\x1FU`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x82;\x15a/\xE2W`@Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R\x90\x82\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x03]Wa\x05\x04WP\xF3[PPP\xFD[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x90P4a\x05BW_`\x03\x196\x01\x12a\x05BW\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x04\x81Ra0[`$\x82a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05BW_a0\xB6\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a1\xB9V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a1lWa1YW[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x82;\x15a/\xE2W`@Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R\x90\x82\x90\x82\x90\x81\x83\x81`D\x81\x01a/\xD1V[a1e\x91P_\x90a4\x88V[__a0\xDBV[`@Q=_\x82>=\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a1\x9AWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a1\x8DV[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a1\xFBWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a1\xEEV[4a\x05BW__`\x03\x196\x01\x12a\x05BW`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90a\x10\x8F\x80\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3+W` \x92\x84\x92aX\xEE\x849\x81R\x03\x01\x90_\xF0\x80\x15a1lW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x05BW`\x01`\x01`\xA0\x1B\x03`$_\x92\x83`@Q\x95\x86\x94\x85\x93c\xD4\xF0\xEBM`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a1lWa3\x18W[Pa\x08\x8D` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\x04` \x82\x01R\x7Ftest\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a3$\x91P_\x90a4\x88V[__a2\xCDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a3\x8AWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a3\xC6\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa1\xB9V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a3{V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a4\x07WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a4]\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a1\xDEV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a3\xF8V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3+W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3+W`@RV[\x90\x81` \x91\x03\x12a\x05BWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x05BW\x90V[\x90\x81` \x91\x03\x12a\x05BWQ\x80\x15\x15\x81\x03a\x05BW\x90V[`\x01`\x01`\xA0\x1B\x03a5\t\x94\x93\x81``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a1\xB9V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a3+W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90a52\x82a5\x0CV[a5?`@Q\x91\x82a4\x88V[\x82\x81R`\x1F\x19a5O\x82\x94a5\x0CV[\x01\x90` 6\x91\x017V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a3+W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a6jW[` \x85\x10\x84\x14a6=W\x84\x87R\x86\x93\x90\x81\x15a5\xFDWP`\x01\x14a5\xB9W[Pa5\xB7\x92P\x03\x83a4\x88V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a5\xE1WPP\x90` a5\xB7\x92\x82\x01\x01_a5\xAAV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a5\xC8V[` \x93Pa5\xB7\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a5\xAAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a5\x8BV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a8\x8BWa5\xB7\x94T\x91\x81\x81\x10a8UW[\x81\x81\x10a8\x1FW[\x81\x81\x10a7\xE9W[\x81\x81\x10a7\xB3W[\x81\x81\x10a7}W[\x81\x81\x10a7GW[\x81\x81\x10a7\x12W[\x10a6\xE5W[P\x03\x83a4\x88V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a6\xDDV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a6\xD7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a6\xCFV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a6\xC7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a6\xBFV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a6\xB7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a6\xAFV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a6\xA7V[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a6\x8FV[`\x08T`\xFF\x16\x80\x15a9'W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a1lW_\x91a9\xBFW[P\x15\x15\x90V[\x90P` \x81=` \x11a9\xE9W[\x81a9\xDA` \x93\x83a4\x88V[\x81\x01\x03\x12a\x05BWQ_a9\xB9V[=\x91Pa9\xCDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05BW`\x01`\x01`\xA0\x1B\x03`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a1lWa:pWPV[_a5\xB7\x91a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05BW`@Q\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a1lWa:pWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05BW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a1lWa:pWPV\xFE`\x804`\xB8W`\x1Fa\x10%8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`\xBCW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`\xB8WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`\xB8W\x80\x15`\xA5W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x0FT\x90\x81a\0\xD1\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xF3\x86\xF4\x14a\x07\xA4W\x80c\x05.\xEF\xD1\x14a\x06#W\x80c\x1BB\xC7\x11\x14a\x04\x07W\x80cqP\x18\xA6\x14a\x03\x8BW\x80cz9y\xDC\x14a\x01\x90W\x80c\x8D\xA5\xCB[\x14a\x01^W\x80c\xA2kJ\x88\x14a\x01CWc\xF2\xFD\xE3\x8B\x14a\0qW_\x80\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\x9Fa\x08\xC2V[a\0\xA7a\t\xD4V[\x16\x80\x15a\x01\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01?W_`\x03\x196\x01\x12a\x01?W` `@Q`(\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01?W```\x03\x196\x01\x12a\x01?Wa\x01\xA9a\x08\xC2V[`$5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01?W`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01?W6`#\x82\x01\x12\x15a\x01?W\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W`$\x81\x01\x90`$\x836\x92\x01\x01\x11a\x01?W`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15a\x03\x80W`@Q\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90` \x90\x82\x90\x81\x80a\x02\xC8\x89\x89\x8C\x8E`\x04\x86\x01a\tkV[\x03\x91Z\xFA\x90\x81\x15a\x03uW_\x91a\x03;W[P\x15a\x02\xFFWa\x02\xE9\x90a\r\nV[\x90a\x02mWPPPPP[` `@Q`\x01\x81R\xF3[a\x037\x83\x86\x93`@Q\x94\x85\x94\x7Fy\xA12P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01a\tkV[\x03\x90\xFD[\x90P` \x81=\x82\x11a\x03mW[\x81a\x03U` \x93\x83a\x08\xE5V[\x81\x01\x03\x12a\x01?WQ\x80\x15\x15\x81\x03a\x01?W\x86a\x02\xDAV[=\x91Pa\x03HV[`@Q=_\x82>=\x90\xFD[PPPPPPa\x02\xF4V[4a\x01?W_`\x03\x196\x01\x12a\x01?Wa\x03\xA3a\t\xD4V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01?W_`\x03\x196\x01\x12a\x01?W`\x01Ta\x04#\x81a\tSV[a\x040`@Q\x91\x82a\x08\xE5V[\x81\x81Ra\x04<\x82a\tSV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x82\x01\x92\x016\x837`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x84\x82\x10\x80a\x06\x04W[\x15a\x05\xFAW\x82Q\x82\x10\x15a\x05\xCDW\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x0B\x92\x16` \x84`\x05\x1B\x86\x01\x01Ra\r\nV[\x90\x15a\x05oW\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x05BW`\x01\x01\x90a\x04\xCAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[PP\x90\x91P[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x05\x9EWPPP\x03\x90\xF3[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x05\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[PP\x90\x91Pa\x05uV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x04\xD3V[4a\x01?W`@`\x03\x196\x01\x12a\x01?Wa\x06<a\x08\xC2V[`$5\x90\x81\x15\x15\x82\x03a\x01?Wa\x06Qa\t\xD4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x07|Wa\x06x\x82a\n V[a\x07TW`(`\x01T\x10\x15a\x07,W\x15a\x07\x1EWa\x06\x95\x90a\x0EkV[\x15a\x06\xC0W\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FAddress not added\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[a\x07'\x90a\rkV[a\x06\x95V[\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Wa\x07\xBDa\x08\xC2V[a\x07\xC5a\t\xD4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x07|Wa\x07\xEC\x81a\n V[\x15a\x08\x9AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x08\x10\x83\x92a\x0B\xF5V[\x16\x03a\x08<W\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FAddress not removed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t&W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\t&W`\x05\x1B` \x01\x90V[\x92\x93\x80`\x80\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x95\x81`\x1F\x96\x16\x88R\x16` \x87\x01R```@\x87\x01R\x81``\x87\x01R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\t\xF4WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80_R`\x02` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\n\xE3W[\x15a\n\xDDW`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\n\xD9W`\x01\x90V[_\x90V[P`\x01\x90V[P\x80_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\njV[`\x01\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0B\xABW[\x15a\x0B\xA4W_\x80R` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ T\x16\x91\x16\x14_\x14a\n\xD9W`\x01\x90V[PP`\x01\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x0BdV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x80\x15a\x0C\xF8W[a\x0C\xF2W_\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T`\x01\x80\x88R\x84\x88 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x8BR\x89\x89R\x87\x8B \x8B\x80R\x89R\x87\x8B \x80T\x92\x90\x95\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x95U\x93\x8AR\x97\x87R\x85\x89 \x82\x8AR\x87R\x94\x90\x97 \x80T\x87\x16\x90\x91\x17\x90U\x80T\x85\x16\x90U\x90\x91R\x80T\x90\x91\x16\x90UT\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x05BW`\x01U\x90V[PP_\x90V[Pa\r\x04\x82`\x01a\x0B\x18V[\x15a\x0C\x15V[a\r\x15\x81`\x01a\x0B\x18V[a\r WP_\x90_\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90\x81\x15\x15\x91\x90V[a\rv\x81`\x01a\x0B\x18V[\x15\x80a\x0EZW[a\r\x86WP_\x90V[\x7Fn\xE3\xEF\xEC\xAE\x88=\xF2\xD7\xCC\xDA\"a\x0BL\xA7q\xA2\x99\xE7\x07\xCB\re\xC4\xEC\x97\xDCNfh\xAD\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 `\x01\x80\x86R\x81\x84R\x82\x86 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x89T\x81\x16\x88\x17\x90\x99U\x98\x90\x96\x16\x80\x85R\x92\x82R\x80\x84 \x97\x84R\x96\x81R\x86\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U\x81\x80R\x92\x90\x91R\x92\x90\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U[`\x01T`\x01\x81\x01\x80\x91\x11a\x05BW`\x01U`\x01\x90V[Pa\x0Ef_`\x01a\x0B\x18V[a\r}V[a\x0Ev\x81`\x01a\x0B\x18V[\x15\x80a\x0FCW[a\x0E\x86WP_\x90V[\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9D\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x88T\x81\x16\x87\x17\x90\x98U\x97\x90\x95\x16\x80\x84R\x91\x81R\x84\x83 \x83\x80R\x81R\x84\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U`\x01\x82R\x94\x90\x91R \x80T\x90\x91\x16\x90\x91\x17\x90Ua\x0EDV[Pa\x0FO_`\x01a\x0B\x18V[a\x0E}V`\x804``W`\x1Fa\x02\xBC8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`dW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12``WQ\x80\x15\x15\x80\x91\x03``W`\xFF\x80\x19_T\x16\x91\x16\x17_U`@Qa\x02C\x90\x81a\0y\x829\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c-\xCF\xAF\xD7\x14a\x01\x92W\x81cz9y\xDC\x14a\0\x80WPc\xA5{\x02\xA6\x14a\0=W_\x80\xFD[4a\0|W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W` `\xFF_T\x16`@Q\x90\x15\x15\x81R\xF3[_\x80\xFD[4a\0|W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|Wa\0\xB7a\x01\xFDV[Pa\0\xC0a\x02 V[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0|W6`#\x82\x01\x12\x15a\0|W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01eW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x81`\x1F\x84\x01\x16\x01\x16\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01eW`@R\x80\x83R6`$\x82\x84\x01\x01\x11a\0|W_\x92\x81`$` \x94\x01\x84\x83\x017\x01\x01R` `\xFF_T\x16`@Q\x90\x15\x15\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[4a\0|W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W`\x045\x80\x15\x15\x80\x91\x03a\0|W`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0_T\x16\x91\x16\x17_U_\x80\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0|WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0|WV`\x80\x80`@R4`\x15Wa\n\x85\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c$\x07\xF0\xB6\x14a\x08\x16WP\x80cH\\\xC9U\x14a\x05[W\x80c[<\xD6\xE2\x14a\x05\tW\x80cqP\x18\xA6\x14a\x04MW\x80cz9y\xDC\x14a\x01\xE2W\x80c\x8D\xA5\xCB[\x14a\x01\x90W\x80c\xD4\xF0\xEBM\x14a\0\xCBW\x80c\xE8\xEB\x1D\xC3\x14a\0\xAEWc\xF2\xFD\xE3\x8B\x14a\0\x7FW_\x80\xFD[4a\0\xAAW` `\x03\x196\x01\x12a\0\xAAWa\0\xA8a\0\x9Ba\x08NV[a\0\xA3a\t\xC2V[a\x08\xD5V[\0[_\x80\xFD[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAW` `@Qb\x03\r@\x81R\xF3[4a\0\xAAW` `\x03\x196\x01\x12a\0\xAAWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xF9a\x08NV[a\x01\x01a\t\xC2V[\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0T\x16\x17\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0U\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9_\x80\xA2\0[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\0\xAAW```\x03\x196\x01\x12a\0\xAAWa\x01\xFBa\x08NV[a\x02\x03a\x08qV[`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\xAAW6`#\x83\x01\x12\x15a\0\xAAW\x81`\x04\x015\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04 W`@Q\x92a\x02l` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01\x85a\x08\x94V[\x80\x84R` \x84\x01\x916`$\x83\x83\x01\x01\x11a\0\xAAW\x81_\x92`$` \x93\x01\x857\x85\x01\x01R\x82Qb\x03\r@\x81\x11a\x03\xEEWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0T\x16\x90`\x01\x82\x14\x93\x84\x15a\x02\xEEW[` \x85`@Q\x90\x15\x15\x81R\xF3[` \x94Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x88\x94`\x84\x94`@Q\x9A\x8B\x99\x8A\x98\x89\x97\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R\x16`\x04\x88\x01R\x16`$\x86\x01R```D\x86\x01RQ\x80\x91\x81`d\x87\x01R\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x81\x01\x03\x01\x91Z\xFA\x80\x15a\x03\xE3W_\x90a\x03\xA7W[` \x91P\x82\x80\x80\x80a\x02\xE1V[P` \x81=` \x11a\x03\xDBW[\x81a\x03\xC1` \x93\x83a\x08\x94V[\x81\x01\x03\x12a\0\xAAWQ\x80\x15\x15\x81\x03a\0\xAAW` \x90a\x03\x9AV[=\x91Pa\x03\xB4V[`@Q=_\x82>=\x90\xFD[\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04Rb\x03\r@`$R`D_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAWa\x04ea\t\xC2V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0T\x16`@Q\x90\x81R\xF3[4a\0\xAAW`@`\x03\x196\x01\x12a\0\xAAWa\x05ta\x08NV[a\x05|a\x08qV[\x90\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x91`\xFF\x83`@\x1C\x16\x15\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x90\x81a\x08\x0EW[`\x01\x14\x90\x81a\x08\x04W[\x15\x90\x81a\x07\xFBW[Pa\x07\xD3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x81\x85`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x06U\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x07~W[Pa\x06Ea\n.V[a\x06Ma\n.V[a\0\xA3a\n.V[\x16\x90\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0T\x16\x17\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0U`@Q\x91\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9_\x80\xA2a\x06\xEDW\0[` \x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U`\x01\x81R\xA1\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x85a\x06<V[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P\x15\x85a\x05\xCFV[0;\x15\x91Pa\x05\xC7V[\x85\x91Pa\x05\xBDV[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAW\x80\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0` \x92R\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xAAWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xAAWV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04 W`@RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\t\x96Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a\n\x02WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\n]WV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD`\x804`\xB8W`\x1Fa\x10\x8F8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`\xBCW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`\xB8WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`\xB8W\x80\x15`\xA5W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x0F\xBE\x90\x81a\0\xD1\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xF3\x86\xF4\x14a\x06<W\x80c\x05.\xEF\xD1\x14a\x04\xBBW\x80c\x1BB\xC7\x11\x14a\x02\x9FW\x80cqP\x18\xA6\x14a\x02#W\x80cz9y\xDC\x14a\x01\x90W\x80c\x8D\xA5\xCB[\x14a\x01^W\x80c\xA2kJ\x88\x14a\x01CWc\xF2\xFD\xE3\x8B\x14a\0qW_\x80\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\x9Fa\x07ZV[a\0\xA7a\n>V[\x16\x80\x15a\x01\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01?W_`\x03\x196\x01\x12a\x01?W` `@Q`(\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01?W```\x03\x196\x01\x12a\x01?Wa\x01\xA9a\x07ZV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?W`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W6`#\x83\x01\x12\x15a\x01?W\x81`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W6`$\x83\x85\x01\x01\x11a\x01?W` \x93`$a\x02\x19\x94\x01\x91a\x08AV[`@Q\x90\x15\x15\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?Wa\x02;a\n>V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01?W_`\x03\x196\x01\x12a\x01?W`\x01Ta\x02\xBB\x81a\x07\xEBV[a\x02\xC8`@Q\x91\x82a\x07}V[\x81\x81Ra\x02\xD4\x82a\x07\xEBV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x82\x01\x92\x016\x837`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x84\x82\x10\x80a\x04\x9CW[\x15a\x04\x92W\x82Q\x82\x10\x15a\x04eW\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\xA3\x92\x16` \x84`\x05\x1B\x86\x01\x01Ra\rtV[\x90\x15a\x04\x07W\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x03\xDAW`\x01\x01\x90a\x03bV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[PP\x90\x91P[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x046WPPP\x03\x90\xF3[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04(V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[PP\x90\x91Pa\x04\rV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x03kV[4a\x01?W`@`\x03\x196\x01\x12a\x01?Wa\x04\xD4a\x07ZV[`$5\x90\x81\x15\x15\x82\x03a\x01?Wa\x04\xE9a\n>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x06\x14Wa\x05\x10\x82a\n\x8AV[a\x05\xECW`(`\x01T\x10\x15a\x05\xC4W\x15a\x05\xB6Wa\x05-\x90a\x0E\xD5V[\x15a\x05XW\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FAddress not added\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[a\x05\xBF\x90a\r\xD5V[a\x05-V[\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Wa\x06Ua\x07ZV[a\x06]a\n>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x06\x14Wa\x06\x84\x81a\n\x8AV[\x15a\x072Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\xA8\x83\x92a\x0C_V[\x16\x03a\x06\xD4W\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FAddress not removed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xBEW`\x05\x1B` \x01\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DT\x93\x94\x90\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x82\x15a\t\xCBW\x91[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15a\n\x1BW` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16`\x04\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`$\x83\x01R```D\x83\x01R\x81\x80a\tD`d\x82\x01\x8D\x8Ca\x08\x03V[\x03\x91Z\xFA\x90\x81\x15a\n\x10W_\x91a\t\xD6W[Pa\t\xCBWa\td\x90a\rtV[\x90a\x08\xAEWPPPa\t\xC7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93[`@Q\x94\x85\x94\x7F\x02\0\xDAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x16`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a\x08\x03V[\x03\x90\xFD[P\x93PPPP`\x01\x90V[\x90P` \x81=\x82\x11a\n\x08W[\x81a\t\xF0` \x93\x83a\x07}V[\x81\x01\x03\x12a\x01?WQ\x80\x15\x15\x81\x03a\x01?W_a\tVV[=\x91Pa\t\xE3V[`@Q=_\x82>=\x90\xFD[PPPPa\t\xC7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a\t\x87V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\n^WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80_R`\x02` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0BMW[\x15a\x0BGW`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0BCW`\x01\x90V[_\x90V[P`\x01\x90V[P\x80_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\n\xD4V[`\x01\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0C\x15W[\x15a\x0C\x0EW_\x80R` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ T\x16\x91\x16\x14_\x14a\x0BCW`\x01\x90V[PP`\x01\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x0B\xCEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x80\x15a\rbW[a\r\\W_\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T`\x01\x80\x88R\x84\x88 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x8BR\x89\x89R\x87\x8B \x8B\x80R\x89R\x87\x8B \x80T\x92\x90\x95\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x95U\x93\x8AR\x97\x87R\x85\x89 \x82\x8AR\x87R\x94\x90\x97 \x80T\x87\x16\x90\x91\x17\x90U\x80T\x85\x16\x90U\x90\x91R\x80T\x90\x91\x16\x90UT\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x03\xDAW`\x01U\x90V[PP_\x90V[Pa\rn\x82`\x01a\x0B\x82V[\x15a\x0C\x7FV[a\r\x7F\x81`\x01a\x0B\x82V[a\r\x8AWP_\x90_\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90\x81\x15\x15\x91\x90V[a\r\xE0\x81`\x01a\x0B\x82V[\x15\x80a\x0E\xC4W[a\r\xF0WP_\x90V[\x7Fn\xE3\xEF\xEC\xAE\x88=\xF2\xD7\xCC\xDA\"a\x0BL\xA7q\xA2\x99\xE7\x07\xCB\re\xC4\xEC\x97\xDCNfh\xAD\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 `\x01\x80\x86R\x81\x84R\x82\x86 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x89T\x81\x16\x88\x17\x90\x99U\x98\x90\x96\x16\x80\x85R\x92\x82R\x80\x84 \x97\x84R\x96\x81R\x86\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U\x81\x80R\x92\x90\x91R\x92\x90\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U[`\x01T`\x01\x81\x01\x80\x91\x11a\x03\xDAW`\x01U`\x01\x90V[Pa\x0E\xD0_`\x01a\x0B\x82V[a\r\xE7V[a\x0E\xE0\x81`\x01a\x0B\x82V[\x15\x80a\x0F\xADW[a\x0E\xF0WP_\x90V[\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9D\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x88T\x81\x16\x87\x17\x90\x98U\x97\x90\x95\x16\x80\x84R\x91\x81R\x84\x83 \x83\x80R\x81R\x84\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U`\x01\x82R\x94\x90\x91R \x80T\x90\x91\x16\x90\x91\x17\x90Ua\x0E\xAEV[Pa\x0F\xB9_`\x01a\x0B\x82V[a\x0E\xE7V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c90816308887aea14613014575080630a9254e414612d555780631436ee7f14612b39578063192c28af14612a3a5780631ed7831c146129bc5780632abc50bf146128615780632ade38801461266d5780632ecdbf56146124b357806331c956a4146123c65780633e5e3c23146123485780633f7286f4146122ca57806342a85af5146121dd5780634359a74e146121b657806344768da3146120b1578063481c6a75146120875780634c58ef7b14611ec657806351d9d3d514611cad5780635387476614611b60578063575fc5d5146119ec5780635e5e5f6914611811578063626d0766146115ba578063629fe631146114af57806366d9a9a0146113725780637004eac41461125d578063716e2aa814610b9357806382eddc221461109057806385226c8114611006578063916a17c614610f5c5780639d4aa91c14610e80578063a109f39a14610d6d578063a6b3e93d14610bbf578063ac1717b014610b98578063ae7ed49f14610b93578063b0464fdc14610ae9578063b1a63516146109ca578063b5508aa914610940578063b9edb1af14610919578063ba414fa6146108f4578063bbb9bf8514610770578063c12e468714610749578063cdc4a3be14610723578063d11187dd1461054e578063d48ca5841461040e578063e20c9f7114610380578063f09d1d441461026f578063f851a440146102485763fa7626d414610223575f80fd5b34610245578060031936011261024557602060ff601f54166040519015158152f35b80fd5b503461024557806003193601126102455760206001600160a01b0360225416604051908152f35b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d578180916004604051809481937f715018a60000000000000000000000000000000000000000000000000000000083525af1801561035d57610368575b5050600460206001600160a01b03601f5460081c16604051928380927f8da5cb5b0000000000000000000000000000000000000000000000000000000082525afa801561035d5761032b91839161032e575b506139f1565b80f35b610350915060203d602011610356575b6103488183613488565b8101906134ab565b5f610325565b503d61033e565b6040513d84823e3d90fd5b8161037291613488565b61024557805f6102d3565b50fd5b503461024557806003193601126102455760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106103ef576103eb856103df81870382613488565b60405191829182613177565b0390f35b82546001600160a01b03168452602090930192600192830192016103c8565b503461024557806003193601126102455780600460206001600160a01b03601f5460081c16604051928380927f2407f0b60000000000000000000000000000000000000000000000000000000082525afa90811561035d578291610515575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d57604051907f7c84c69b00000000000000000000000000000000000000000000000000000000825260048201527f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d50060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576105045750f35b8161050e91613488565b6102455780f35b9150506020813d602011610546575b8161053160209383613488565b81010312610542578190515f61046d565b5f80fd5b3d9150610524565b5034610245578060031936011261024557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d57604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d5761070e575b506001600160a01b0360235416604051907f118cdaa7000000000000000000000000000000000000000000000000000000006020830152602482015260248152610626604482613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d578161068191604051809381927ff28dceb30000000000000000000000000000000000000000000000000000000083526020600484015260248301906131b9565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d576106f9575b506001600160a01b03601f5460081c166001600160a01b0360215416813b156106f557829160248392604051948593849263d4f0eb4d60e01b845260048401525af1801561035d576105045750f35b5050fd5b8161070391613488565b61024557805f6106a6565b8161071891613488565b61024557805f6105db565b503461024557806003193601126102455760206001600160a01b03815416604051908152f35b503461024557806003193601126102455760206001600160a01b0360235416604051908152f35b5034610245576040600319360112610245576004356001600160a01b03811681036108f057602435906001600160a01b03821682036108ec57826001600160a01b03601f5460081c16803b156108f05781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d576108d7575b505061088d91602091604051916108478361083986820160609060208152600460208201527f746573740000000000000000000000000000000000000000000000000000000060408201520190565b03601f198101855284613488565b6001600160a01b03601f5460081c1691604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b03915afa801561035d5761032b9183916108a8575b50613afb565b6108ca915060203d6020116108d0575b6108c28183613488565b8101906134ca565b5f6108a2565b503d6108b8565b816108e191613488565b6108ec57825f6107ea565b8280fd5b5080fd5b5034610245578060031936011261024557602061090f613918565b6040519015158152f35b503461024557806003193601126102455760206001600160a01b0360255416604051908152f35b503461024557806003193601126102455760195461095d81613559565b9161096b6040519384613488565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b8383106109ad57604051806103eb8782613358565b6001602081926109bc85613571565b815201920192019190610998565b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d57610ad4575b505061088d6020604051610a7681610a6884820160609060208152600960208201527f746573742064617461000000000000000000000000000000000000000000000060408201520190565b03601f198101835282613488565b6001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b81610ade91613488565b61024557805f610a1c565b5034610245578060031936011261024557601c54610b0681613559565b91610b146040519384613488565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610b5657604051806103eb87826133d5565b60026020600192604051610b698161346c565b6001600160a01b038654168152610b81858701613674565b83820152815201920192019190610b41565b613233565b503461024557806003193601126102455760206001600160a01b0360245416604051908152f35b5034610245578060031936011261024557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561024557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152818180610c3960048201905f60606080840193600181528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d57610d58575b50604051817f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b98180a26001600160a01b03601f5460081c16803b156106f5578160248185809463d4f0eb4d60e01b83528160048401525af1801561035d57610d43575b5050600460206001600160a01b03601f5460081c16604051928380927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa90811561035d5761032b916001600160a01b03918491610d24575b50166139f1565b610d3d915060203d602011610356576103488183613488565b5f610d1d565b81610d4d91613488565b61024557805f610cc1565b81610d6291613488565b61024557805f610c5e565b50346102455780600319360112610245576001600160a01b03601f5460081c16604051907f8da5cb5b000000000000000000000000000000000000000000000000000000008252602082600481845afa908115610e7557610de76020926004948691610e58575b506001600160a01b036022541690613a7a565b604051928380927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa801561035d5761032b918391610e39575b506001600160a01b0380602054169116613a7a565b610e52915060203d602011610356576103488183613488565b5f610e24565b610e6f9150843d8611610356576103488183613488565b5f610dd4565b6040513d85823e3d90fd5b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d57610f47575b505061088d6020604051610ee68282613488565b8381526001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b81610f5191613488565b61024557805f610ed2565b5034610245578060031936011261024557601d54610f7981613559565b91610f876040519384613488565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310610fc957604051806103eb87826133d5565b60026020600192604051610fdc8161346c565b6001600160a01b038654168152610ff4858701613674565b83820152815201920192019190610fb4565b5034610245578060031936011261024557601a5461102381613559565b916110316040519384613488565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b83831061107357604051806103eb8782613358565b60016020819261108285613571565b81520192019201919061105e565b5034610245578060031936011261024557806001600160a01b03601f5460081c166001600160a01b0360215416813b156106f557829160248392604051948593849263d4f0eb4d60e01b845260048401525af1801561035d57611248575b506001600160a01b0360215416803b1561037d578180916024604051809481937f2dcfafd70000000000000000000000000000000000000000000000000000000083528160048401525af1801561035d57611233575b5061118f6020604051610a7681610a6884820160609060208152600960208201527f746573742064617461000000000000000000000000000000000000000000000060408201520190565b03915afa90811561035d578291611214575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d57604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576105045750f35b61122d915060203d6020116108d0576108c28183613488565b5f6111a1565b8161123d91613488565b61024557805f611144565b8161125291613488565b61024557805f6110ee565b5034610245578060031936011261024557806001600160a01b03601f5460081c166001600160a01b0360215416813b156106f557829160248392604051948593849263d4f0eb4d60e01b845260048401525af1801561035d5761135d575b506001600160a01b0360215416803b1561037d578180916024604051809481937f2dcfafd7000000000000000000000000000000000000000000000000000000008352600160048401525af1801561035d57610ad457505061088d6020604051610a7681610a6884820160609060208152600960208201527f746573742064617461000000000000000000000000000000000000000000000060408201520190565b8161136791613488565b61024557805f6112bb565b5034610245578060031936011261024557601b5461138f81613559565b61139c6040519182613488565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b83831061147457868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061140957505050500390f35b91936020611464827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc060019597998495030186528851908361145483516040845260408401906131b9565b92015190848184039101526131de565b96019201920185949391926113fa565b600260206001926040516114878161346c565b61149086613571565b815261149d858701613674565b838201528152019201920191906113cc565b503461024557806003193601126102455780600460206001600160a01b03601f5460081c16604051928380927fe8eb1dc30000000000000000000000000000000000000000000000000000000082525afa90811561035d578291611588575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d57604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015262030d4060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576105045750f35b90506020813d6020116115b2575b816115a360209383613488565b8101031261037d57515f61150e565b3d9150611596565b5034610245578060031936011261024557604051610a9f8082019082821067ffffffffffffffff8311176117e457908291614e4f8339039082f080156117d7576040516102bc8082019082821067ffffffffffffffff8311176117aa576020918391614b93833985815203019083f090811561035d576001600160a01b031690826001600160a01b038060245416921691833b156108f0576040517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201529083166024820152818160448183885af1801561035d57611795575b5050604051917f8da5cb5b000000000000000000000000000000000000000000000000000000008352602083600481845afa90811561178a57611700602092600495879161176d575b506001600160a01b036024541690613a7a565b604051938480927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa918215610e755761032b926001600160a01b0391859161174e575b5016613a7a565b611767915060203d602011610356576103488183613488565b5f611747565b6117849150843d8611610356576103488183613488565b5f6116ed565b6040513d86823e3d90fd5b8161179f91613488565b6108ec57825f6116a4565b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50604051903d90823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346102455780600319360112610245576001600160a01b036022541660405190611025908183019183831067ffffffffffffffff8411176117aa57918391602093613b6e8439815203019082f080156117d7576001600160a01b0316737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057816040517f491cc7c20000000000000000000000000000000000000000000000000000000081528181806118d760048201905f60606080840193600181528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d576119d7575b50604051827f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b98380a26001600160a01b03601f5460081c16803b156108ec578160248185809463d4f0eb4d60e01b83528860048401525af1801561035d576119c2575b505060049060206001600160a01b03601f5460081c16604051938480927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa918215610e755761032b926001600160a01b0391859161174e575016613a7a565b816119cc91613488565b6108f057815f61195f565b816119e191613488565b6108f057815f6118fc565b5034610245578060031936011261024557806001600160a01b03601f5460081c166040517f8da5cb5b000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610e7557611a5f918491611b4757506001600160a01b036022541690613a7a565b6001600160a01b0360245416813b156106f55782916024839260405194859384927ff2fde38b00000000000000000000000000000000000000000000000000000000845260048401525af1801561035d57611b32575b5050600460206001600160a01b03601f5460081c16604051928380927f8da5cb5b0000000000000000000000000000000000000000000000000000000082525afa801561035d5761032b918391611b1957506001600160a01b036024541690613a7a565b611784915060203d602011610356576103488183613488565b81611b3c91613488565b61024557805f611ab5565b610e6f915060203d602011610356576103488183613488565b5034610245578060031936011261024557806001600160a01b03601f5460081c166040517f5b3cd6e2000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610e7557611bd5918491610e3957506001600160a01b0380602054169116613a7a565b6001600160a01b0360215416813b156106f557829160248392604051948593849263d4f0eb4d60e01b845260048401525af1801561035d57611c98575b5050600460206001600160a01b03601f5460081c16604051928380927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa801561035d5761032b918391611c79575b506001600160a01b0380602154169116613a7a565b611c92915060203d602011610356576103488183613488565b5f611c64565b81611ca291613488565b61024557805f611c12565b50346102455780600319360112610245576001600160a01b036022541690604051611025928382019382851067ffffffffffffffff8611176117e45783946020928492613b6e8439815203019082f080156117d7576001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106f557604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610e75578391611eb1575b50506001600160a01b0360235416604051907f118cdaa7000000000000000000000000000000000000000000000000000000006020830152602482015260248152611dcc604482613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106f55782611e2791604051809381927ff28dceb30000000000000000000000000000000000000000000000000000000083526020600484015260248301906131b9565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610e75578391611e9c575b50506001600160a01b03601f5460081c1690813b156106f5576001600160a01b036024849283604051958694859363d4f0eb4d60e01b85521660048401525af1801561035d576105045750f35b81611ea691613488565b61037d57815f611e4f565b81611ebb91613488565b61037d57815f611d80565b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b83528160048401525af1801561035d57612072575b5050604051611f5f81610a686020820160609060208152600960208201527f746573742064617461000000000000000000000000000000000000000000000060408201520190565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057816040517ff4844814000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d5761205d575b5050602061202f916001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b03915afa801561035d57612041575080f35b6120599060203d6020116108d0576108c28183613488565b5080f35b8161206791613488565b6108f057815f611fc9565b8161207c91613488565b61024557805f611f17565b503461024557806003193601126102455760206001600160a01b03601f5460081c16604051908152f35b503461024557602060031936011261024557600435737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057816040517f4c63e56200000000000000000000000000000000000000000000000000000000815262030d4083111560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576121a1575b506001600160a01b03601f5460081c16803b156108f05781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d5761218c575b50506020610a7661088d92613528565b8161219691613488565b6108f057815f61217c565b816121ab91613488565b6108f057815f61213b565b503461024557806003193601126102455760206001600160a01b0360215416604051908152f35b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d576122b5575b505061088d602062030d60601f196040519161224b8184613488565b62030d3f83520136838301376001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b816122bf91613488565b61024557805f61222f565b503461024557806003193601126102455760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110612329576103eb856103df81870382613488565b82546001600160a01b0316845260209093019260019283019201612312565b503461024557806003193601126102455760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b8181106123a7576103eb856103df81870382613488565b82546001600160a01b0316845260209093019260019283019201612390565b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d5761249e575b505061088d602062030d60601f19604051916124348184613488565b62030d4083520136838301376001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b816124a891613488565b61024557805f612418565b5034610245578060031936011261024557806001600160a01b03601f5460081c16803b1561037d5781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d57612658575b5050604051601f1962030d8061251b8184613488565b62030d41835201366020830137816040517f4634691b00000000000000000000000000000000000000000000000000000000602082015262030d41602482015262030d40604482015260448152612573606482613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057816125ce91604051809381927ff28dceb30000000000000000000000000000000000000000000000000000000083526020600484015260248301906131b9565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d5761205d575050602061202f916001600160a01b03601f5460081c166001600160a01b03602454166001600160a01b036025541690604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016134e2565b8161266291613488565b61024557805f612505565b5034610245578060031936011261024557601e5461268a81613559565b6126976040519182613488565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106127d85786858760405192839260208401906020855251809152604084019160408260051b8601019392815b8383106127035786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b82811061278f575050505050602080600192970193019301909286959492936126f6565b90919293946020806127cb837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0876001960301895289516131b9565b970195019392910161276b565b6040516127e48161346c565b6001600160a01b03835416815260018301805461280081613559565b9161280e6040519384613488565b8183528a526020808b20908b9084015b8382106128445750505050600192826020928360029501528152019201920191906126c7565b60016020819261285386613571565b81520193019101909161281e565b5034610245576020600319360112610245576004358162030d408211806129b0575b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108f057604051907f4c63e562000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d5761299b575b506001600160a01b03601f5460081c16803b156108f05781809160246040518094819363d4f0eb4d60e01b8352600160048401525af1801561035d57612986575b5061293e82613528565b91604051907f4634691b000000000000000000000000000000000000000000000000000000006020830152602482015262030d40604482015260448152612573606482613488565b8161299091613488565b6108f057815f612934565b816129a591613488565b6108f057815f6128f3565b50620f42408210612883565b503461024557806003193601126102455760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110612a1b576103eb856103df81870382613488565b82546001600160a01b0316845260209093019260019283019201612a04565b50346102455780600319360112610245576001600160a01b036022541690604051611025928382019382851067ffffffffffffffff8611176117e45783946020928492613b6e8439815203019082f080156117d7576001600160a01b03601f5460081c1690813b156106f5576001600160a01b036024849283604051958694859363d4f0eb4d60e01b85521660048401525af1801561035d57612b24575b505061088d6020604051610a7681610a6884820160609060208152600460208201527f746573740000000000000000000000000000000000000000000000000000000060408201520190565b81612b2e91613488565b61024557805f612ad8565b5034610245578060031936011261024557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561024557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152818180612bb360048201905f60606080840193600181528260208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561035d57612d40575b5060405160017f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b98380a26001600160a01b03601f5460081c16803b156106f5578160248185809463d4f0eb4d60e01b8352600160048401525af1801561035d57612d2b575b50600460206001600160a01b03601f5460081c16604051928380927f5b3cd6e20000000000000000000000000000000000000000000000000000000082525afa90811561035d578291612d0c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561037d576001600160a01b03604051917f515361f6000000000000000000000000000000000000000000000000000000008352166004820152600160248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561035d576105045750f35b612d25915060203d602011610356576103488183613488565b5f612c8b565b81612d3591613488565b61024557805f612c3d565b81612d4a91613488565b61024557805f612bd8565b5034610245578060031936011261024557307fffffffffffffffffffffffff000000000000000000000000000000000000000060225416176022556104567fffffffffffffffffffffffff000000000000000000000000000000000000000060235416176023556101237fffffffffffffffffffffffff000000000000000000000000000000000000000060245416176024556107897fffffffffffffffffffffffff000000000000000000000000000000000000000060255416176025556040516110258082019082821067ffffffffffffffff8311176117e4576020918391613b6e833930815203019082f080156117d7576001600160a01b03167fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556040516102bc8082019082821067ffffffffffffffff8311176117e4576020918391614b9383396001815203019082f080156117d7576001600160a01b03167fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215560405190610a9f918281019281841067ffffffffffffffff851117612fe75782938291614e4f8339039082f080156117d7576001600160a01b03907fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b1691161780601f5560081c166001600160a01b03602254166001600160a01b0360205416823b15612fe2576040517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b039283166004820152911660248201529082908290818381604481015b03925af1801561035d576105045750f35b505050fd5b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b905034610542575f600319360112610542577ff92ee8a90000000000000000000000000000000000000000000000000000000060208201526004815261305b602482613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610542575f6130b691604051809381927ff28dceb30000000000000000000000000000000000000000000000000000000083526020600484015260248301906131b9565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561316c57613159575b50806001600160a01b03601f5460081c166001600160a01b03602254166001600160a01b0360205416823b15612fe2576040517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b03928316600482015291166024820152908290829081838160448101612fd1565b61316591505f90613488565b5f5f6130db565b6040513d5f823e3d90fd5b60206040818301928281528451809452019201905f5b81811061319a5750505090565b82516001600160a01b031684526020938401939092019160010161318d565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b8181106131fb5750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016131ee565b34610542575f5f600319360112610542576001600160a01b03602254166040519061108f80830183811067ffffffffffffffff82111761332b5760209284926158ee843981520301905ff0801561316c576001600160a01b03601f5460081c1690813b15610542576001600160a01b0360245f9283604051958694859363d4f0eb4d60e01b85521660048401525af1801561316c57613318575b5061088d6020604051610a7681610a6884820160609060208152600460208201527f746573740000000000000000000000000000000000000000000000000000000060408201520190565b61332491505f90613488565b5f5f6132cd565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061338a57505050505090565b90919293946020806133c6837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0866001960301875289516131b9565b9701930193019193929061337b565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061340757505050505090565b909192939460208061345d837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906131de565b970193019301919392906133f8565b6040810190811067ffffffffffffffff82111761332b57604052565b90601f601f19910116810190811067ffffffffffffffff82111761332b57604052565b9081602091031261054257516001600160a01b03811681036105425790565b90816020910312610542575180151581036105425790565b6001600160a01b0361350994938160609416835216602082015281604082015201906131b9565b90565b67ffffffffffffffff811161332b57601f01601f191660200190565b906135328261350c565b61353f6040519182613488565b828152601f1961354f829461350c565b0190602036910137565b67ffffffffffffffff811161332b5760051b60200190565b90604051915f8154908160011c926001831692831561366a575b60208510841461363d5784875286939081156135fd57506001146135b9575b506135b792500383613488565b565b90505f9291925260205f20905f915b8183106135e15750509060206135b7928201015f6135aa565b60209193508060019154838589010152019101909184926135c8565b602093506135b79592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f6135aa565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f169361358b565b90604051918281549182825260208201905f5260205f20925f905b80600783011061388b576135b7945491818110613855575b81811061381f575b8181106137e9575b8181106137b3575b81811061377d575b818110613747575b818110613712575b106136e5575b500383613488565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f6136dd565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b1681520193016136d7565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b1681520193016136cf565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b1681520193016136c7565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b1681520193016136bf565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b1681520193016136b7565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b1681520193016136af565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b1681520193016136a7565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e082015201940192018592939161368f565b60085460ff1680156139275790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561316c575f916139bf575b50151590565b90506020813d6020116139e9575b816139da60209383613488565b8101031261054257515f6139b9565b3d91506139cd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610542576001600160a01b03604051917f515361f60000000000000000000000000000000000000000000000000000000083521660048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561316c57613a705750565b5f6135b791613488565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610542576040517f515361f60000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152911660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561316c57613a705750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561054257604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561316c57613a70575056fe60803460b857601f61102538819003918201601f19168301916001600160401b0383118484101760bc5780849260209460405283398101031260b857516001600160a01b0381169081900360b857801560a5575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3610f5490816100d18239f35b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304f386f4146107a4578063052eefd1146106235780631b42c71114610407578063715018a61461038b5780637a3979dc146101905780638da5cb5b1461015e578063a26b4a88146101435763f2fde38b14610071575f80fd5b3461013f57602060031936011261013f5773ffffffffffffffffffffffffffffffffffffffff61009f6108c2565b6100a76109d4565b1680156101135773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461013f575f60031936011261013f57602060405160288152f35b3461013f575f60031936011261013f57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013f57606060031936011261013f576101a96108c2565b60243573ffffffffffffffffffffffffffffffffffffffff8116810361013f5760443567ffffffffffffffff811161013f573660238201121561013f5780600401359067ffffffffffffffff821161013f576024810190602483369201011161013f5760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b73ffffffffffffffffffffffffffffffffffffffff81168015610380576040517f7a3979dc00000000000000000000000000000000000000000000000000000000815290602090829081806102c889898c8e6004860161096b565b03915afa908115610375575f9161033b575b50156102ff576102e990610d0a565b9061026d5750505050505b602060405160018152f35b6103378386936040519485947f79a132500000000000000000000000000000000000000000000000000000000086526004860161096b565b0390fd5b90506020813d821161036d575b81610355602093836108e5565b8101031261013f5751801515810361013f57866102da565b3d9150610348565b6040513d5f823e3d90fd5b5050505050506102f4565b3461013f575f60031936011261013f576103a36109d4565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461013f575f60031936011261013f5760015461042381610953565b61043060405191826108e5565b81815261043c82610953565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe060208201920136833760015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b84821080610604575b156105fa5782518210156105cd578073ffffffffffffffffffffffffffffffffffffffff61050b921660208460051b86010152610d0a565b901561056f57907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811461054257600101906104ca565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b50509091505b604051918291602083019060208452518091526040830191905f5b81811061059e575050500390f35b825173ffffffffffffffffffffffffffffffffffffffff16845285945060209384019390920191600101610590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5050909150610575565b5073ffffffffffffffffffffffffffffffffffffffff811615156104d3565b3461013f57604060031936011261013f5761063c6108c2565b60243590811515820361013f576106516109d4565b73ffffffffffffffffffffffffffffffffffffffff811691821561077c5761067882610a20565b610754576028600154101561072c571561071e5761069590610e6b565b156106c0577f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e75f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f41646472657373206e6f742061646465640000000000000000000000000000006044820152fd5b61072790610d6b565b610695565b7f13d867a2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa2d86a1e000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe6c4247b000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461013f57602060031936011261013f576107bd6108c2565b6107c56109d4565b73ffffffffffffffffffffffffffffffffffffffff811690811561077c576107ec81610a20565b1561089a5773ffffffffffffffffffffffffffffffffffffffff6108108392610bf5565b160361083c577fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be5f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f41646472657373206e6f742072656d6f766564000000000000000000000000006044820152fd5b7f3d0f293d000000000000000000000000000000000000000000000000000000005f5260045ffd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361013f57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761092657604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff81116109265760051b60200190565b92938060809573ffffffffffffffffffffffffffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe09581601f9616885216602087015260606040870152816060870152868601375f8582860101520116010190565b73ffffffffffffffffffffffffffffffffffffffff5f541633036109f457565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff16805f52600260205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610ae3575b15610add5760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff1603610ad957600190565b5f90565b50600190565b50805f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610a6a565b60010173ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610bab575b15610ba4575f805260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2054169116145f14610ad957600190565b5050600190565b5073ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610b64565b73ffffffffffffffffffffffffffffffffffffffff811680158015610cf8575b610cf2575f90815260026020818152604080842084805280835281852080546001808852848820805473ffffffffffffffffffffffffffffffffffffffff908116808b52898952878b208b80528952878b208054929095167fffffffffffffffffffffffff00000000000000000000000000000000000000009283168117909555938a52978752858920828a5287529490972080548716909117905580548516905590915280549091169055547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116105425760015590565b50505f90565b50610d04826001610b18565b15610c15565b610d15816001610b18565b610d2057505f905f90565b73ffffffffffffffffffffffffffffffffffffffff165f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f205416908115159190565b610d76816001610b18565b1580610e5a575b610d8657505f90565b7f6ee3efecae883df2d7ccda22610b4ca771a299e707cb0d65c4ec97dc4e6668ad805473ffffffffffffffffffffffffffffffffffffffff9283165f818152600260208181526040808420600180865281845282862080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811690915589548116881790995598909616808552928252808420978452968152868320805487169094179093558180529290915292909220805490911690911790555b6001546001810180911161054257600155600190565b50610e665f6001610b18565b610d7d565b610e76816001610b18565b1580610f43575b610e8657505f90565b7f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d805473ffffffffffffffffffffffffffffffffffffffff9283165f81815260026020818152604080842084805280835281852080547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091558854811687179098559790951680845291815284832083805281528483208054871690941790935560018252949091522080549091169091179055610e44565b50610f4f5f6001610b18565b610e7d56608034606057601f6102bc38819003918201601f19168301916001600160401b038311848410176064578084926020946040528339810103126060575180151580910360605760ff80195f54169116175f5560405161024390816100798239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c9081632dcfafd7146101925781637a3979dc14610080575063a57b02a61461003d575f80fd5b3461007c575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c57602060ff5f54166040519015158152f35b5f80fd5b3461007c5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c576100b76101fd565b506100c0610220565b5060443567ffffffffffffffff811161007c573660238201121561007c57806004013567ffffffffffffffff8111610165577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f81601f8401160116830183811067ffffffffffffffff82111761016557604052808352366024828401011161007c575f928160246020940184830137010152602060ff5f54166040519015158152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b3461007c5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c5760043580151580910361007c5760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff005f54169116175f555f80f35b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361007c57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361007c575660808060405234601557610a85908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c9081632407f0b61461081657508063485cc9551461055b5780635b3cd6e214610509578063715018a61461044d5780637a3979dc146101e25780638da5cb5b14610190578063d4f0eb4d146100cb578063e8eb1dc3146100ae5763f2fde38b1461007f575f80fd5b346100aa5760206003193601126100aa576100a861009b61084e565b6100a36109c2565b6108d5565b005b5f80fd5b346100aa575f6003193601126100aa57602060405162030d408152f35b346100aa5760206003193601126100aa5773ffffffffffffffffffffffffffffffffffffffff6100f961084e565b6101016109c2565b16807fffffffffffffffffffffffff00000000000000000000000000000000000000007f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d5005416177f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d500557f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b95f80a2005b346100aa575f6003193601126100aa57602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b346100aa5760606003193601126100aa576101fb61084e565b610203610871565b6044359067ffffffffffffffff82116100aa57366023830112156100aa5781600401359167ffffffffffffffff8311610420576040519261026c60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8401160185610894565b808452602084019136602483830101116100aa57815f92602460209301853785010152825162030d4081116103ee575073ffffffffffffffffffffffffffffffffffffffff7f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d500541690600182149384156102ee575b6020856040519015158152f35b6020945073ffffffffffffffffffffffffffffffffffffffff957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f88946084946040519a8b998a9889977f7a3979dc000000000000000000000000000000000000000000000000000000008952166004880152166024860152606060448601525180918160648701528686015e5f858286010152011681010301915afa80156103e3575f906103a7575b60209150828080806102e1565b506020813d6020116103db575b816103c160209383610894565b810103126100aa575180151581036100aa5760209061039a565b3d91506103b4565b6040513d5f823e3d90fd5b7f4634691b000000000000000000000000000000000000000000000000000000005f5260045262030d4060245260445ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b346100aa575f6003193601126100aa576104656109c2565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346100aa575f6003193601126100aa57602073ffffffffffffffffffffffffffffffffffffffff7f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d5005416604051908152f35b346100aa5760406003193601126100aa5761057461084e565b61057c610871565b907ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549160ff8360401c16159267ffffffffffffffff81168015908161080e575b6001149081610804575b1590816107fb575b506107d35773ffffffffffffffffffffffffffffffffffffffff92818560017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000006106559516177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005561077e575b50610645610a2e565b61064d610a2e565b6100a3610a2e565b1690817fffffffffffffffffffffffff00000000000000000000000000000000000000007f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d5005416177f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d50055604051917f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b95f80a26106ed57005b60207fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2917fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005560018152a1005b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00558561063c565b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b905015856105cf565b303b1591506105c7565b8591506105bd565b346100aa575f6003193601126100aa57807f5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d50060209252f35b6004359073ffffffffffffffffffffffffffffffffffffffff821682036100aa57565b6024359073ffffffffffffffffffffffffffffffffffffffff821682036100aa57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761042057604052565b73ffffffffffffffffffffffffffffffffffffffff1680156109965773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054163303610a0257565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c1615610a5d57565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd60803460b857601f61108f38819003918201601f19168301916001600160401b0383118484101760bc5780849260209460405283398101031260b857516001600160a01b0381169081900360b857801560a5575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3610fbe90816100d18239f35b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304f386f41461063c578063052eefd1146104bb5780631b42c7111461029f578063715018a6146102235780637a3979dc146101905780638da5cb5b1461015e578063a26b4a88146101435763f2fde38b14610071575f80fd5b3461013f57602060031936011261013f5773ffffffffffffffffffffffffffffffffffffffff61009f61075a565b6100a7610a3e565b1680156101135773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461013f575f60031936011261013f57602060405160288152f35b3461013f575f60031936011261013f57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013f57606060031936011261013f576101a961075a565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361013f576044359067ffffffffffffffff821161013f573660238301121561013f5781600401359067ffffffffffffffff821161013f57366024838501011161013f576020936024610219940191610841565b6040519015158152f35b3461013f575f60031936011261013f5761023b610a3e565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461013f575f60031936011261013f576001546102bb816107eb565b6102c8604051918261077d565b8181526102d4826107eb565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe060208201920136833760015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b8482108061049c575b15610492578251821015610465578073ffffffffffffffffffffffffffffffffffffffff6103a3921660208460051b86010152610d74565b901561040757907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81146103da5760010190610362565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b50509091505b604051918291602083019060208452518091526040830191905f5b818110610436575050500390f35b825173ffffffffffffffffffffffffffffffffffffffff16845285945060209384019390920191600101610428565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b505090915061040d565b5073ffffffffffffffffffffffffffffffffffffffff8116151561036b565b3461013f57604060031936011261013f576104d461075a565b60243590811515820361013f576104e9610a3e565b73ffffffffffffffffffffffffffffffffffffffff81169182156106145761051082610a8a565b6105ec57602860015410156105c457156105b65761052d90610ed5565b15610558577f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e75f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f41646472657373206e6f742061646465640000000000000000000000000000006044820152fd5b6105bf90610dd5565b61052d565b7f13d867a2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa2d86a1e000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe6c4247b000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461013f57602060031936011261013f5761065561075a565b61065d610a3e565b73ffffffffffffffffffffffffffffffffffffffff81169081156106145761068481610a8a565b156107325773ffffffffffffffffffffffffffffffffffffffff6106a88392610c5f565b16036106d4577fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be5f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f41646472657373206e6f742072656d6f766564000000000000000000000000006044820152fd5b7f3d0f293d000000000000000000000000000000000000000000000000000000005f5260045ffd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361013f57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176107be57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff81116107be5760051b60200190565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b60015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d549394909373ffffffffffffffffffffffffffffffffffffffff169182156109cb57915b73ffffffffffffffffffffffffffffffffffffffff81168015610a1b57602060405180927f7a3979dc00000000000000000000000000000000000000000000000000000000825273ffffffffffffffffffffffffffffffffffffffff8916600483015273ffffffffffffffffffffffffffffffffffffffff87166024830152606060448301528180610944606482018d8c610803565b03915afa908115610a10575f916109d6575b506109cb5761096490610d74565b906108ae575050506109c79073ffffffffffffffffffffffffffffffffffffffff935b6040519485947f0200da48000000000000000000000000000000000000000000000000000000008652166004850152604060248501526044840191610803565b0390fd5b509350505050600190565b90506020813d8211610a08575b816109f06020938361077d565b8101031261013f5751801515810361013f575f610956565b3d91506109e3565b6040513d5f823e3d90fd5b505050506109c79073ffffffffffffffffffffffffffffffffffffffff93610987565b73ffffffffffffffffffffffffffffffffffffffff5f54163303610a5e57565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff16805f52600260205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610b4d575b15610b475760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff1603610b4357600190565b5f90565b50600190565b50805f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610ad4565b60010173ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610c15575b15610c0e575f805260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2054169116145f14610b4357600190565b5050600190565b5073ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610bce565b73ffffffffffffffffffffffffffffffffffffffff811680158015610d62575b610d5c575f90815260026020818152604080842084805280835281852080546001808852848820805473ffffffffffffffffffffffffffffffffffffffff908116808b52898952878b208b80528952878b208054929095167fffffffffffffffffffffffff00000000000000000000000000000000000000009283168117909555938a52978752858920828a5287529490972080548716909117905580548516905590915280549091169055547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116103da5760015590565b50505f90565b50610d6e826001610b82565b15610c7f565b610d7f816001610b82565b610d8a57505f905f90565b73ffffffffffffffffffffffffffffffffffffffff165f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f205416908115159190565b610de0816001610b82565b1580610ec4575b610df057505f90565b7f6ee3efecae883df2d7ccda22610b4ca771a299e707cb0d65c4ec97dc4e6668ad805473ffffffffffffffffffffffffffffffffffffffff9283165f818152600260208181526040808420600180865281845282862080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811690915589548116881790995598909616808552928252808420978452968152868320805487169094179093558180529290915292909220805490911690911790555b600154600181018091116103da57600155600190565b50610ed05f6001610b82565b610de7565b610ee0816001610b82565b1580610fad575b610ef057505f90565b7f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d805473ffffffffffffffffffffffffffffffffffffffff9283165f81815260026020818152604080842084805280835281852080547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091558854811687179098559790951680845291815284832083805281528483208054871690941790935560018252949091522080549091169091179055610eae565b50610fb95f6001610b82565b610ee756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x08\x88z\xEA\x14a0\x14WP\x80c\n\x92T\xE4\x14a-UW\x80c\x146\xEE\x7F\x14a+9W\x80c\x19,(\xAF\x14a*:W\x80c\x1E\xD7\x83\x1C\x14a)\xBCW\x80c*\xBCP\xBF\x14a(aW\x80c*\xDE8\x80\x14a&mW\x80c.\xCD\xBFV\x14a$\xB3W\x80c1\xC9V\xA4\x14a#\xC6W\x80c>^<#\x14a#HW\x80c?r\x86\xF4\x14a\"\xCAW\x80cB\xA8Z\xF5\x14a!\xDDW\x80cCY\xA7N\x14a!\xB6W\x80cDv\x8D\xA3\x14a \xB1W\x80cH\x1Cju\x14a \x87W\x80cLX\xEF{\x14a\x1E\xC6W\x80cQ\xD9\xD3\xD5\x14a\x1C\xADW\x80cS\x87Gf\x14a\x1B`W\x80cW_\xC5\xD5\x14a\x19\xECW\x80c^^_i\x14a\x18\x11W\x80cbm\x07f\x14a\x15\xBAW\x80cb\x9F\xE61\x14a\x14\xAFW\x80cf\xD9\xA9\xA0\x14a\x13rW\x80cp\x04\xEA\xC4\x14a\x12]W\x80cqn*\xA8\x14a\x0B\x93W\x80c\x82\xED\xDC\"\x14a\x10\x90W\x80c\x85\"l\x81\x14a\x10\x06W\x80c\x91j\x17\xC6\x14a\x0F\\W\x80c\x9DJ\xA9\x1C\x14a\x0E\x80W\x80c\xA1\t\xF3\x9A\x14a\rmW\x80c\xA6\xB3\xE9=\x14a\x0B\xBFW\x80c\xAC\x17\x17\xB0\x14a\x0B\x98W\x80c\xAE~\xD4\x9F\x14a\x0B\x93W\x80c\xB0FO\xDC\x14a\n\xE9W\x80c\xB1\xA65\x16\x14a\t\xCAW\x80c\xB5P\x8A\xA9\x14a\t@W\x80c\xB9\xED\xB1\xAF\x14a\t\x19W\x80c\xBAAO\xA6\x14a\x08\xF4W\x80c\xBB\xB9\xBF\x85\x14a\x07pW\x80c\xC1.F\x87\x14a\x07IW\x80c\xCD\xC4\xA3\xBE\x14a\x07#W\x80c\xD1\x11\x87\xDD\x14a\x05NW\x80c\xD4\x8C\xA5\x84\x14a\x04\x0EW\x80c\xE2\x0C\x9Fq\x14a\x03\x80W\x80c\xF0\x9D\x1DD\x14a\x02oW\x80c\xF8Q\xA4@\x14a\x02HWc\xFAv&\xD4\x14a\x02#W_\x80\xFD[4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7FqP\x18\xA6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x03]Wa\x03hW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x03.W[Pa9\xF1V[\x80\xF3[a\x03P\x91P` =` \x11a\x03VW[a\x03H\x81\x83a4\x88V[\x81\x01\x90a4\xABV[_a\x03%V[P=a\x03>V[`@Q=\x84\x82>=\x90\xFD[\x81a\x03r\x91a4\x88V[a\x02EW\x80_a\x02\xD3V[P\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x03\xEFWa\x03\xEB\x85a\x03\xDF\x81\x87\x03\x82a4\x88V[`@Q\x91\x82\x91\x82a1wV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x03\xC8V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F$\x07\xF0\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03]W\x82\x91a\x05\x15W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`@Q\x90\x7F|\x84\xC6\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa\x05\x04WP\xF3[\x81a\x05\x0E\x91a4\x88V[a\x02EW\x80\xF3[\x91PP` \x81=` \x11a\x05FW[\x81a\x051` \x93\x83a4\x88V[\x81\x01\x03\x12a\x05BW\x81\x90Q_a\x04mV[_\x80\xFD[=\x91Pa\x05$V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa\x07\x0EW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra\x06&`D\x82a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W\x81a\x06\x81\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a1\xB9V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa\x06\xF9W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c\xD4\xF0\xEBM`\xE0\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x05\x04WP\xF3[PP\xFD[\x81a\x07\x03\x91a4\x88V[a\x02EW\x80_a\x06\xA6V[\x81a\x07\x18\x91a4\x88V[a\x02EW\x80_a\x05\xDBV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x02EW`@`\x03\x196\x01\x12a\x02EW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x08\xF0W`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x08\xECW\x82`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x08\xF0W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x08\xD7W[PPa\x08\x8D\x91` \x91`@Q\x91a\x08G\x83a\x089\x86\x82\x01``\x90` \x81R`\x04` \x82\x01R\x7Ftest\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x85R\x84a4\x88V[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x03\x91Z\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x08\xA8W[Pa:\xFBV[a\x08\xCA\x91P` =` \x11a\x08\xD0W[a\x08\xC2\x81\x83a4\x88V[\x81\x01\x90a4\xCAV[_a\x08\xA2V[P=a\x08\xB8V[\x81a\x08\xE1\x91a4\x88V[a\x08\xECW\x82_a\x07\xEAV[\x82\x80\xFD[P\x80\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` a\t\x0Fa9\x18V[`@Q\x90\x15\x15\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x19Ta\t]\x81a5YV[\x91a\tk`@Q\x93\x84a4\x88V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\t\xADW`@Q\x80a\x03\xEB\x87\x82a3XV[`\x01` \x81\x92a\t\xBC\x85a5qV[\x81R\x01\x92\x01\x92\x01\x91\x90a\t\x98V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\n\xD4W[PPa\x08\x8D` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\t` \x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a4\x88V[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a\n\xDE\x91a4\x88V[a\x02EW\x80_a\n\x1CV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1CTa\x0B\x06\x81a5YV[\x91a\x0B\x14`@Q\x93\x84a4\x88V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x0BVW`@Q\x80a\x03\xEB\x87\x82a3\xD5V[`\x02` `\x01\x92`@Qa\x0Bi\x81a4lV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x0B\x81\x85\x87\x01a6tV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0BAV[a23V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02EW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\x0C9`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa\rXW[P`@Q\x81\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x81\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06\xF5W\x81`$\x81\x85\x80\x94c\xD4\xF0\xEBM`\xE0\x1B\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\rCW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03]Wa\x03+\x91`\x01`\x01`\xA0\x1B\x03\x91\x84\x91a\r$W[P\x16a9\xF1V[a\r=\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a\r\x1DV[\x81a\rM\x91a4\x88V[a\x02EW\x80_a\x0C\xC1V[\x81a\rb\x91a4\x88V[a\x02EW\x80_a\x0C^V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x90\x81\x15a\x0EuWa\r\xE7` \x92`\x04\x94\x86\x91a\x0EXW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90a:zV[`@Q\x92\x83\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x0E9W[P`\x01`\x01`\xA0\x1B\x03\x80` T\x16\x91\x16a:zV[a\x0ER\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a\x0E$V[a\x0Eo\x91P\x84=\x86\x11a\x03VWa\x03H\x81\x83a4\x88V[_a\r\xD4V[`@Q=\x85\x82>=\x90\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x0FGW[PPa\x08\x8D` `@Qa\x0E\xE6\x82\x82a4\x88V[\x83\x81R`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a\x0FQ\x91a4\x88V[a\x02EW\x80_a\x0E\xD2V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1DTa\x0Fy\x81a5YV[\x91a\x0F\x87`@Q\x93\x84a4\x88V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x0F\xC9W`@Q\x80a\x03\xEB\x87\x82a3\xD5V[`\x02` `\x01\x92`@Qa\x0F\xDC\x81a4lV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x0F\xF4\x85\x87\x01a6tV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0F\xB4V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1ATa\x10#\x81a5YV[\x91a\x101`@Q\x93\x84a4\x88V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x10sW`@Q\x80a\x03\xEB\x87\x82a3XV[`\x01` \x81\x92a\x10\x82\x85a5qV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x10^V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c\xD4\xF0\xEBM`\xE0\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x12HW[P`\x01`\x01`\xA0\x1B\x03`!T\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x123W[Pa\x11\x8F` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\t` \x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x03\x91Z\xFA\x90\x81\x15a\x03]W\x82\x91a\x12\x14W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa\x05\x04WP\xF3[a\x12-\x91P` =` \x11a\x08\xD0Wa\x08\xC2\x81\x83a4\x88V[_a\x11\xA1V[\x81a\x12=\x91a4\x88V[a\x02EW\x80_a\x11DV[\x81a\x12R\x91a4\x88V[a\x02EW\x80_a\x10\xEEV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c\xD4\xF0\xEBM`\xE0\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x13]W[P`\x01`\x01`\xA0\x1B\x03`!T\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\n\xD4WPPa\x08\x8D` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\t` \x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x81a\x13g\x91a4\x88V[a\x02EW\x80_a\x12\xBBV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1BTa\x13\x8F\x81a5YV[a\x13\x9C`@Q\x91\x82a4\x88V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x14tW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x14\tWPPPP\x03\x90\xF3[\x91\x93` a\x14d\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x14T\x83Q`@\x84R`@\x84\x01\x90a1\xB9V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra1\xDEV[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x13\xFAV[`\x02` `\x01\x92`@Qa\x14\x87\x81a4lV[a\x14\x90\x86a5qV[\x81Ra\x14\x9D\x85\x87\x01a6tV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x13\xCCV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xE8\xEB\x1D\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03]W\x82\x91a\x15\x88W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rb\x03\r@`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa\x05\x04WP\xF3[\x90P` \x81=` \x11a\x15\xB2W[\x81a\x15\xA3` \x93\x83a4\x88V[\x81\x01\x03\x12a\x03}WQ_a\x15\x0EV[=\x91Pa\x15\x96V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Qa\n\x9F\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x17\xE4W\x90\x82\x91aNO\x839\x03\x90\x82\xF0\x80\x15a\x17\xD7W`@Qa\x02\xBC\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x17\xAAW` \x91\x83\x91aK\x93\x839\x85\x81R\x03\x01\x90\x83\xF0\x90\x81\x15a\x03]W`\x01`\x01`\xA0\x1B\x03\x16\x90\x82`\x01`\x01`\xA0\x1B\x03\x80`$T\x16\x92\x16\x91\x83;\x15a\x08\xF0W`@Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R\x81\x81`D\x81\x83\x88Z\xF1\x80\x15a\x03]Wa\x17\x95W[PP`@Q\x91\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` \x83`\x04\x81\x84Z\xFA\x90\x81\x15a\x17\x8AWa\x17\0` \x92`\x04\x95\x87\x91a\x17mW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x90a:zV[`@Q\x93\x84\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x0EuWa\x03+\x92`\x01`\x01`\xA0\x1B\x03\x91\x85\x91a\x17NW[P\x16a:zV[a\x17g\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a\x17GV[a\x17\x84\x91P\x84=\x86\x11a\x03VWa\x03H\x81\x83a4\x88V[_a\x16\xEDV[`@Q=\x86\x82>=\x90\xFD[\x81a\x17\x9F\x91a4\x88V[a\x08\xECW\x82_a\x16\xA4V[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90a\x10%\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x17\xAAW\x91\x83\x91` \x93a;n\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W\x81`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\x18\xD7`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa\x19\xD7W[P`@Q\x82\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x83\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x08\xECW\x81`$\x81\x85\x80\x94c\xD4\xF0\xEBM`\xE0\x1B\x83R\x88`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x19\xC2W[PP`\x04\x90` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x0EuWa\x03+\x92`\x01`\x01`\xA0\x1B\x03\x91\x85\x91a\x17NWP\x16a:zV[\x81a\x19\xCC\x91a4\x88V[a\x08\xF0W\x81_a\x19_V[\x81a\x19\xE1\x91a4\x88V[a\x08\xF0W\x81_a\x18\xFCV[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x0EuWa\x1A_\x91\x84\x91a\x1BGWP`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90a:zV[`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x1B2W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x1B\x19WP`\x01`\x01`\xA0\x1B\x03`$T\x16\x90a:zV[a\x17\x84\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[\x81a\x1B<\x91a4\x88V[a\x02EW\x80_a\x1A\xB5V[a\x0Eo\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x0EuWa\x1B\xD5\x91\x84\x91a\x0E9WP`\x01`\x01`\xA0\x1B\x03\x80` T\x16\x91\x16a:zV[`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x06\xF5W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c\xD4\xF0\xEBM`\xE0\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x1C\x98W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03]Wa\x03+\x91\x83\x91a\x1CyW[P`\x01`\x01`\xA0\x1B\x03\x80`!T\x16\x91\x16a:zV[a\x1C\x92\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a\x1CdV[\x81a\x1C\xA2\x91a4\x88V[a\x02EW\x80_a\x1C\x12V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Qa\x10%\x92\x83\x82\x01\x93\x82\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x17\xE4W\x83\x94` \x92\x84\x92a;n\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\xF5W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0EuW\x83\x91a\x1E\xB1W[PP`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra\x1D\xCC`D\x82a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\xF5W\x82a\x1E'\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a1\xB9V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0EuW\x83\x91a\x1E\x9CW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x06\xF5W`\x01`\x01`\xA0\x1B\x03`$\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\xD4\xF0\xEBM`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\x05\x04WP\xF3[\x81a\x1E\xA6\x91a4\x88V[a\x03}W\x81_a\x1EOV[\x81a\x1E\xBB\x91a4\x88V[a\x03}W\x81_a\x1D\x80V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa rW[PP`@Qa\x1F_\x81a\nh` \x82\x01``\x90` \x81R`\t` \x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W\x81`@Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa ]W[PP` a /\x91`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x03\x91Z\xFA\x80\x15a\x03]Wa AWP\x80\xF3[a Y\x90` =` \x11a\x08\xD0Wa\x08\xC2\x81\x83a4\x88V[P\x80\xF3[\x81a g\x91a4\x88V[a\x08\xF0W\x81_a\x1F\xC9V[\x81a |\x91a4\x88V[a\x02EW\x80_a\x1F\x17V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x02EW` `\x03\x196\x01\x12a\x02EW`\x045sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W\x81`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rb\x03\r@\x83\x11\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa!\xA1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x08\xF0W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa!\x8CW[PP` a\nva\x08\x8D\x92a5(V[\x81a!\x96\x91a4\x88V[a\x08\xF0W\x81_a!|V[\x81a!\xAB\x91a4\x88V[a\x08\xF0W\x81_a!;V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa\"\xB5W[PPa\x08\x8D` b\x03\r``\x1F\x19`@Q\x91a\"K\x81\x84a4\x88V[b\x03\r?\x83R\x016\x83\x83\x017`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a\"\xBF\x91a4\x88V[a\x02EW\x80_a\"/V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a#)Wa\x03\xEB\x85a\x03\xDF\x81\x87\x03\x82a4\x88V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a#\x12V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a#\xA7Wa\x03\xEB\x85a\x03\xDF\x81\x87\x03\x82a4\x88V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a#\x90V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa$\x9EW[PPa\x08\x8D` b\x03\r``\x1F\x19`@Q\x91a$4\x81\x84a4\x88V[b\x03\r@\x83R\x016\x83\x83\x017`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a$\xA8\x91a4\x88V[a\x02EW\x80_a$\x18V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x03}W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa&XW[PP`@Q`\x1F\x19b\x03\r\x80a%\x1B\x81\x84a4\x88V[b\x03\rA\x83R\x016` \x83\x017\x81`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Rb\x03\rA`$\x82\x01Rb\x03\r@`D\x82\x01R`D\x81Ra%s`d\x82a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W\x81a%\xCE\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a1\xB9V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa ]WPP` a /\x91`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a4\xE2V[\x81a&b\x91a4\x88V[a\x02EW\x80_a%\x05V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x1ETa&\x8A\x81a5YV[a&\x97`@Q\x91\x82a4\x88V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a'\xD8W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a'\x03W\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a'\x8FWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a&\xF6V[\x90\x91\x92\x93\x94` \x80a'\xCB\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa1\xB9V[\x97\x01\x95\x01\x93\x92\x91\x01a'kV[`@Qa'\xE4\x81a4lV[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta(\0\x81a5YV[\x91a(\x0E`@Q\x93\x84a4\x88V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a(DWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a&\xC7V[`\x01` \x81\x92a(S\x86a5qV[\x81R\x01\x93\x01\x91\x01\x90\x91a(\x1EV[P4a\x02EW` `\x03\x196\x01\x12a\x02EW`\x045\x81b\x03\r@\x82\x11\x80a)\xB0W[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xF0W`@Q\x90\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa)\x9BW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x08\xF0W\x81\x80\x91`$`@Q\x80\x94\x81\x93c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa)\x86W[Pa)>\x82a5(V[\x91`@Q\x90\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01Rb\x03\r@`D\x82\x01R`D\x81Ra%s`d\x82a4\x88V[\x81a)\x90\x91a4\x88V[a\x08\xF0W\x81_a)4V[\x81a)\xA5\x91a4\x88V[a\x08\xF0W\x81_a(\xF3V[Pb\x0FB@\x82\x10a(\x83V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a*\x1BWa\x03\xEB\x85a\x03\xDF\x81\x87\x03\x82a4\x88V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a*\x04V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Qa\x10%\x92\x83\x82\x01\x93\x82\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x17\xE4W\x83\x94` \x92\x84\x92a;n\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x06\xF5W`\x01`\x01`\xA0\x1B\x03`$\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\xD4\xF0\xEBM`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa+$W[PPa\x08\x8D` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\x04` \x82\x01R\x7Ftest\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[\x81a+.\x91a4\x88V[a\x02EW\x80_a*\xD8V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02EW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a+\xB3`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03]Wa-@W[P`@Q`\x01\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x83\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06\xF5W\x81`$\x81\x85\x80\x94c\xD4\xF0\xEBM`\xE0\x1B\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x03]Wa-+W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F[<\xD6\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03]W\x82\x91a-\x0CW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03}W`\x01`\x01`\xA0\x1B\x03`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01R`\x01`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03]Wa\x05\x04WP\xF3[a-%\x91P` =` \x11a\x03VWa\x03H\x81\x83a4\x88V[_a,\x8BV[\x81a-5\x91a4\x88V[a\x02EW\x80_a,=V[\x81a-J\x91a4\x88V[a\x02EW\x80_a+\xD8V[P4a\x02EW\x80`\x03\x196\x01\x12a\x02EW0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"Ua\x04V\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`#T\x16\x17`#Ua\x01#\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$T\x16\x17`$Ua\x07\x89\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`%T\x16\x17`%U`@Qa\x10%\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x17\xE4W` \x91\x83\x91a;n\x8390\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`@Qa\x02\xBC\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x17\xE4W` \x91\x83\x91aK\x93\x839`\x01\x81R\x03\x01\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`@Q\x90a\n\x9F\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a/\xE7W\x82\x93\x82\x91aNO\x839\x03\x90\x82\xF0\x80\x15a\x17\xD7W`\x01`\x01`\xA0\x1B\x03\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17\x80`\x1FU`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x82;\x15a/\xE2W`@Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R\x90\x82\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x03]Wa\x05\x04WP\xF3[PPP\xFD[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x90P4a\x05BW_`\x03\x196\x01\x12a\x05BW\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x04\x81Ra0[`$\x82a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05BW_a0\xB6\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a1\xB9V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a1lWa1YW[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x82;\x15a/\xE2W`@Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R\x90\x82\x90\x82\x90\x81\x83\x81`D\x81\x01a/\xD1V[a1e\x91P_\x90a4\x88V[__a0\xDBV[`@Q=_\x82>=\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a1\x9AWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a1\x8DV[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a1\xFBWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a1\xEEV[4a\x05BW__`\x03\x196\x01\x12a\x05BW`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90a\x10\x8F\x80\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3+W` \x92\x84\x92aX\xEE\x849\x81R\x03\x01\x90_\xF0\x80\x15a1lW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x05BW`\x01`\x01`\xA0\x1B\x03`$_\x92\x83`@Q\x95\x86\x94\x85\x93c\xD4\xF0\xEBM`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a1lWa3\x18W[Pa\x08\x8D` `@Qa\nv\x81a\nh\x84\x82\x01``\x90` \x81R`\x04` \x82\x01R\x7Ftest\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x01\x90V[a3$\x91P_\x90a4\x88V[__a2\xCDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a3\x8AWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a3\xC6\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa1\xB9V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a3{V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a4\x07WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a4]\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a1\xDEV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a3\xF8V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3+W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3+W`@RV[\x90\x81` \x91\x03\x12a\x05BWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x05BW\x90V[\x90\x81` \x91\x03\x12a\x05BWQ\x80\x15\x15\x81\x03a\x05BW\x90V[`\x01`\x01`\xA0\x1B\x03a5\t\x94\x93\x81``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a1\xB9V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a3+W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90a52\x82a5\x0CV[a5?`@Q\x91\x82a4\x88V[\x82\x81R`\x1F\x19a5O\x82\x94a5\x0CV[\x01\x90` 6\x91\x017V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a3+W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a6jW[` \x85\x10\x84\x14a6=W\x84\x87R\x86\x93\x90\x81\x15a5\xFDWP`\x01\x14a5\xB9W[Pa5\xB7\x92P\x03\x83a4\x88V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a5\xE1WPP\x90` a5\xB7\x92\x82\x01\x01_a5\xAAV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a5\xC8V[` \x93Pa5\xB7\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a5\xAAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a5\x8BV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a8\x8BWa5\xB7\x94T\x91\x81\x81\x10a8UW[\x81\x81\x10a8\x1FW[\x81\x81\x10a7\xE9W[\x81\x81\x10a7\xB3W[\x81\x81\x10a7}W[\x81\x81\x10a7GW[\x81\x81\x10a7\x12W[\x10a6\xE5W[P\x03\x83a4\x88V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a6\xDDV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a6\xD7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a6\xCFV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a6\xC7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a6\xBFV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a6\xB7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a6\xAFV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a6\xA7V[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a6\x8FV[`\x08T`\xFF\x16\x80\x15a9'W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a1lW_\x91a9\xBFW[P\x15\x15\x90V[\x90P` \x81=` \x11a9\xE9W[\x81a9\xDA` \x93\x83a4\x88V[\x81\x01\x03\x12a\x05BWQ_a9\xB9V[=\x91Pa9\xCDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05BW`\x01`\x01`\xA0\x1B\x03`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a1lWa:pWPV[_a5\xB7\x91a4\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05BW`@Q\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a1lWa:pWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05BW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a1lWa:pWPV\xFE`\x804`\xB8W`\x1Fa\x10%8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`\xBCW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`\xB8WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`\xB8W\x80\x15`\xA5W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x0FT\x90\x81a\0\xD1\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xF3\x86\xF4\x14a\x07\xA4W\x80c\x05.\xEF\xD1\x14a\x06#W\x80c\x1BB\xC7\x11\x14a\x04\x07W\x80cqP\x18\xA6\x14a\x03\x8BW\x80cz9y\xDC\x14a\x01\x90W\x80c\x8D\xA5\xCB[\x14a\x01^W\x80c\xA2kJ\x88\x14a\x01CWc\xF2\xFD\xE3\x8B\x14a\0qW_\x80\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\x9Fa\x08\xC2V[a\0\xA7a\t\xD4V[\x16\x80\x15a\x01\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01?W_`\x03\x196\x01\x12a\x01?W` `@Q`(\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01?W```\x03\x196\x01\x12a\x01?Wa\x01\xA9a\x08\xC2V[`$5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01?W`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01?W6`#\x82\x01\x12\x15a\x01?W\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W`$\x81\x01\x90`$\x836\x92\x01\x01\x11a\x01?W`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15a\x03\x80W`@Q\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90` \x90\x82\x90\x81\x80a\x02\xC8\x89\x89\x8C\x8E`\x04\x86\x01a\tkV[\x03\x91Z\xFA\x90\x81\x15a\x03uW_\x91a\x03;W[P\x15a\x02\xFFWa\x02\xE9\x90a\r\nV[\x90a\x02mWPPPPP[` `@Q`\x01\x81R\xF3[a\x037\x83\x86\x93`@Q\x94\x85\x94\x7Fy\xA12P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01a\tkV[\x03\x90\xFD[\x90P` \x81=\x82\x11a\x03mW[\x81a\x03U` \x93\x83a\x08\xE5V[\x81\x01\x03\x12a\x01?WQ\x80\x15\x15\x81\x03a\x01?W\x86a\x02\xDAV[=\x91Pa\x03HV[`@Q=_\x82>=\x90\xFD[PPPPPPa\x02\xF4V[4a\x01?W_`\x03\x196\x01\x12a\x01?Wa\x03\xA3a\t\xD4V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01?W_`\x03\x196\x01\x12a\x01?W`\x01Ta\x04#\x81a\tSV[a\x040`@Q\x91\x82a\x08\xE5V[\x81\x81Ra\x04<\x82a\tSV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x82\x01\x92\x016\x837`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x84\x82\x10\x80a\x06\x04W[\x15a\x05\xFAW\x82Q\x82\x10\x15a\x05\xCDW\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x0B\x92\x16` \x84`\x05\x1B\x86\x01\x01Ra\r\nV[\x90\x15a\x05oW\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x05BW`\x01\x01\x90a\x04\xCAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[PP\x90\x91P[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x05\x9EWPPP\x03\x90\xF3[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x05\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[PP\x90\x91Pa\x05uV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x04\xD3V[4a\x01?W`@`\x03\x196\x01\x12a\x01?Wa\x06<a\x08\xC2V[`$5\x90\x81\x15\x15\x82\x03a\x01?Wa\x06Qa\t\xD4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x07|Wa\x06x\x82a\n V[a\x07TW`(`\x01T\x10\x15a\x07,W\x15a\x07\x1EWa\x06\x95\x90a\x0EkV[\x15a\x06\xC0W\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FAddress not added\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[a\x07'\x90a\rkV[a\x06\x95V[\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Wa\x07\xBDa\x08\xC2V[a\x07\xC5a\t\xD4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x07|Wa\x07\xEC\x81a\n V[\x15a\x08\x9AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x08\x10\x83\x92a\x0B\xF5V[\x16\x03a\x08<W\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FAddress not removed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t&W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\t&W`\x05\x1B` \x01\x90V[\x92\x93\x80`\x80\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x95\x81`\x1F\x96\x16\x88R\x16` \x87\x01R```@\x87\x01R\x81``\x87\x01R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\t\xF4WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80_R`\x02` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\n\xE3W[\x15a\n\xDDW`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\n\xD9W`\x01\x90V[_\x90V[P`\x01\x90V[P\x80_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\njV[`\x01\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0B\xABW[\x15a\x0B\xA4W_\x80R` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ T\x16\x91\x16\x14_\x14a\n\xD9W`\x01\x90V[PP`\x01\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x0BdV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x80\x15a\x0C\xF8W[a\x0C\xF2W_\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T`\x01\x80\x88R\x84\x88 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x8BR\x89\x89R\x87\x8B \x8B\x80R\x89R\x87\x8B \x80T\x92\x90\x95\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x95U\x93\x8AR\x97\x87R\x85\x89 \x82\x8AR\x87R\x94\x90\x97 \x80T\x87\x16\x90\x91\x17\x90U\x80T\x85\x16\x90U\x90\x91R\x80T\x90\x91\x16\x90UT\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x05BW`\x01U\x90V[PP_\x90V[Pa\r\x04\x82`\x01a\x0B\x18V[\x15a\x0C\x15V[a\r\x15\x81`\x01a\x0B\x18V[a\r WP_\x90_\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90\x81\x15\x15\x91\x90V[a\rv\x81`\x01a\x0B\x18V[\x15\x80a\x0EZW[a\r\x86WP_\x90V[\x7Fn\xE3\xEF\xEC\xAE\x88=\xF2\xD7\xCC\xDA\"a\x0BL\xA7q\xA2\x99\xE7\x07\xCB\re\xC4\xEC\x97\xDCNfh\xAD\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 `\x01\x80\x86R\x81\x84R\x82\x86 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x89T\x81\x16\x88\x17\x90\x99U\x98\x90\x96\x16\x80\x85R\x92\x82R\x80\x84 \x97\x84R\x96\x81R\x86\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U\x81\x80R\x92\x90\x91R\x92\x90\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U[`\x01T`\x01\x81\x01\x80\x91\x11a\x05BW`\x01U`\x01\x90V[Pa\x0Ef_`\x01a\x0B\x18V[a\r}V[a\x0Ev\x81`\x01a\x0B\x18V[\x15\x80a\x0FCW[a\x0E\x86WP_\x90V[\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9D\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x88T\x81\x16\x87\x17\x90\x98U\x97\x90\x95\x16\x80\x84R\x91\x81R\x84\x83 \x83\x80R\x81R\x84\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U`\x01\x82R\x94\x90\x91R \x80T\x90\x91\x16\x90\x91\x17\x90Ua\x0EDV[Pa\x0FO_`\x01a\x0B\x18V[a\x0E}V`\x804``W`\x1Fa\x02\xBC8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`dW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12``WQ\x80\x15\x15\x80\x91\x03``W`\xFF\x80\x19_T\x16\x91\x16\x17_U`@Qa\x02C\x90\x81a\0y\x829\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c-\xCF\xAF\xD7\x14a\x01\x92W\x81cz9y\xDC\x14a\0\x80WPc\xA5{\x02\xA6\x14a\0=W_\x80\xFD[4a\0|W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W` `\xFF_T\x16`@Q\x90\x15\x15\x81R\xF3[_\x80\xFD[4a\0|W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|Wa\0\xB7a\x01\xFDV[Pa\0\xC0a\x02 V[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0|W6`#\x82\x01\x12\x15a\0|W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01eW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x81`\x1F\x84\x01\x16\x01\x16\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01eW`@R\x80\x83R6`$\x82\x84\x01\x01\x11a\0|W_\x92\x81`$` \x94\x01\x84\x83\x017\x01\x01R` `\xFF_T\x16`@Q\x90\x15\x15\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[4a\0|W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W`\x045\x80\x15\x15\x80\x91\x03a\0|W`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0_T\x16\x91\x16\x17_U_\x80\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0|WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0|WV`\x80\x80`@R4`\x15Wa\n\x85\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c$\x07\xF0\xB6\x14a\x08\x16WP\x80cH\\\xC9U\x14a\x05[W\x80c[<\xD6\xE2\x14a\x05\tW\x80cqP\x18\xA6\x14a\x04MW\x80cz9y\xDC\x14a\x01\xE2W\x80c\x8D\xA5\xCB[\x14a\x01\x90W\x80c\xD4\xF0\xEBM\x14a\0\xCBW\x80c\xE8\xEB\x1D\xC3\x14a\0\xAEWc\xF2\xFD\xE3\x8B\x14a\0\x7FW_\x80\xFD[4a\0\xAAW` `\x03\x196\x01\x12a\0\xAAWa\0\xA8a\0\x9Ba\x08NV[a\0\xA3a\t\xC2V[a\x08\xD5V[\0[_\x80\xFD[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAW` `@Qb\x03\r@\x81R\xF3[4a\0\xAAW` `\x03\x196\x01\x12a\0\xAAWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xF9a\x08NV[a\x01\x01a\t\xC2V[\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0T\x16\x17\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0U\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9_\x80\xA2\0[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\0\xAAW```\x03\x196\x01\x12a\0\xAAWa\x01\xFBa\x08NV[a\x02\x03a\x08qV[`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\xAAW6`#\x83\x01\x12\x15a\0\xAAW\x81`\x04\x015\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04 W`@Q\x92a\x02l` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01\x85a\x08\x94V[\x80\x84R` \x84\x01\x916`$\x83\x83\x01\x01\x11a\0\xAAW\x81_\x92`$` \x93\x01\x857\x85\x01\x01R\x82Qb\x03\r@\x81\x11a\x03\xEEWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0T\x16\x90`\x01\x82\x14\x93\x84\x15a\x02\xEEW[` \x85`@Q\x90\x15\x15\x81R\xF3[` \x94Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x88\x94`\x84\x94`@Q\x9A\x8B\x99\x8A\x98\x89\x97\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R\x16`\x04\x88\x01R\x16`$\x86\x01R```D\x86\x01RQ\x80\x91\x81`d\x87\x01R\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x81\x01\x03\x01\x91Z\xFA\x80\x15a\x03\xE3W_\x90a\x03\xA7W[` \x91P\x82\x80\x80\x80a\x02\xE1V[P` \x81=` \x11a\x03\xDBW[\x81a\x03\xC1` \x93\x83a\x08\x94V[\x81\x01\x03\x12a\0\xAAWQ\x80\x15\x15\x81\x03a\0\xAAW` \x90a\x03\x9AV[=\x91Pa\x03\xB4V[`@Q=_\x82>=\x90\xFD[\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04Rb\x03\r@`$R`D_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAWa\x04ea\t\xC2V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0T\x16`@Q\x90\x81R\xF3[4a\0\xAAW`@`\x03\x196\x01\x12a\0\xAAWa\x05ta\x08NV[a\x05|a\x08qV[\x90\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x91`\xFF\x83`@\x1C\x16\x15\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x90\x81a\x08\x0EW[`\x01\x14\x90\x81a\x08\x04W[\x15\x90\x81a\x07\xFBW[Pa\x07\xD3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x81\x85`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x06U\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x07~W[Pa\x06Ea\n.V[a\x06Ma\n.V[a\0\xA3a\n.V[\x16\x90\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0T\x16\x17\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0U`@Q\x91\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9_\x80\xA2a\x06\xEDW\0[` \x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U`\x01\x81R\xA1\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x85a\x06<V[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P\x15\x85a\x05\xCFV[0;\x15\x91Pa\x05\xC7V[\x85\x91Pa\x05\xBDV[4a\0\xAAW_`\x03\x196\x01\x12a\0\xAAW\x80\x7F\\m\x17t\xBD\xD6\x9D\x8D\x16\x84|<\x97\xB5\x1E\xA742W\xB8\xF5\xAC\xE5\xDA\x9E%\xAB;\xAF\xD7\xD5\0` \x92R\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xAAWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xAAWV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04 W`@RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\t\x96Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a\n\x02WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\n]WV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD`\x804`\xB8W`\x1Fa\x10\x8F8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`\xBCW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`\xB8WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`\xB8W\x80\x15`\xA5W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x0F\xBE\x90\x81a\0\xD1\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xF3\x86\xF4\x14a\x06<W\x80c\x05.\xEF\xD1\x14a\x04\xBBW\x80c\x1BB\xC7\x11\x14a\x02\x9FW\x80cqP\x18\xA6\x14a\x02#W\x80cz9y\xDC\x14a\x01\x90W\x80c\x8D\xA5\xCB[\x14a\x01^W\x80c\xA2kJ\x88\x14a\x01CWc\xF2\xFD\xE3\x8B\x14a\0qW_\x80\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\x9Fa\x07ZV[a\0\xA7a\n>V[\x16\x80\x15a\x01\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01?W_`\x03\x196\x01\x12a\x01?W` `@Q`(\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01?W```\x03\x196\x01\x12a\x01?Wa\x01\xA9a\x07ZV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?W`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W6`#\x83\x01\x12\x15a\x01?W\x81`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W6`$\x83\x85\x01\x01\x11a\x01?W` \x93`$a\x02\x19\x94\x01\x91a\x08AV[`@Q\x90\x15\x15\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?Wa\x02;a\n>V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01?W_`\x03\x196\x01\x12a\x01?W`\x01Ta\x02\xBB\x81a\x07\xEBV[a\x02\xC8`@Q\x91\x82a\x07}V[\x81\x81Ra\x02\xD4\x82a\x07\xEBV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x82\x01\x92\x016\x837`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x84\x82\x10\x80a\x04\x9CW[\x15a\x04\x92W\x82Q\x82\x10\x15a\x04eW\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\xA3\x92\x16` \x84`\x05\x1B\x86\x01\x01Ra\rtV[\x90\x15a\x04\x07W\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x03\xDAW`\x01\x01\x90a\x03bV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[PP\x90\x91P[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x046WPPP\x03\x90\xF3[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04(V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[PP\x90\x91Pa\x04\rV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x03kV[4a\x01?W`@`\x03\x196\x01\x12a\x01?Wa\x04\xD4a\x07ZV[`$5\x90\x81\x15\x15\x82\x03a\x01?Wa\x04\xE9a\n>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x06\x14Wa\x05\x10\x82a\n\x8AV[a\x05\xECW`(`\x01T\x10\x15a\x05\xC4W\x15a\x05\xB6Wa\x05-\x90a\x0E\xD5V[\x15a\x05XW\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FAddress not added\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[a\x05\xBF\x90a\r\xD5V[a\x05-V[\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Wa\x06Ua\x07ZV[a\x06]a\n>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x06\x14Wa\x06\x84\x81a\n\x8AV[\x15a\x072Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\xA8\x83\x92a\x0C_V[\x16\x03a\x06\xD4W\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FAddress not removed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xBEW`\x05\x1B` \x01\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DT\x93\x94\x90\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x82\x15a\t\xCBW\x91[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15a\n\x1BW` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16`\x04\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`$\x83\x01R```D\x83\x01R\x81\x80a\tD`d\x82\x01\x8D\x8Ca\x08\x03V[\x03\x91Z\xFA\x90\x81\x15a\n\x10W_\x91a\t\xD6W[Pa\t\xCBWa\td\x90a\rtV[\x90a\x08\xAEWPPPa\t\xC7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93[`@Q\x94\x85\x94\x7F\x02\0\xDAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x16`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a\x08\x03V[\x03\x90\xFD[P\x93PPPP`\x01\x90V[\x90P` \x81=\x82\x11a\n\x08W[\x81a\t\xF0` \x93\x83a\x07}V[\x81\x01\x03\x12a\x01?WQ\x80\x15\x15\x81\x03a\x01?W_a\tVV[=\x91Pa\t\xE3V[`@Q=_\x82>=\x90\xFD[PPPPa\t\xC7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a\t\x87V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\n^WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80_R`\x02` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0BMW[\x15a\x0BGW`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0BCW`\x01\x90V[_\x90V[P`\x01\x90V[P\x80_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\n\xD4V[`\x01\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0C\x15W[\x15a\x0C\x0EW_\x80R` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ T\x16\x91\x16\x14_\x14a\x0BCW`\x01\x90V[PP`\x01\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x0B\xCEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x80\x15a\rbW[a\r\\W_\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T`\x01\x80\x88R\x84\x88 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x8BR\x89\x89R\x87\x8B \x8B\x80R\x89R\x87\x8B \x80T\x92\x90\x95\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x95U\x93\x8AR\x97\x87R\x85\x89 \x82\x8AR\x87R\x94\x90\x97 \x80T\x87\x16\x90\x91\x17\x90U\x80T\x85\x16\x90U\x90\x91R\x80T\x90\x91\x16\x90UT\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x03\xDAW`\x01U\x90V[PP_\x90V[Pa\rn\x82`\x01a\x0B\x82V[\x15a\x0C\x7FV[a\r\x7F\x81`\x01a\x0B\x82V[a\r\x8AWP_\x90_\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90\x81\x15\x15\x91\x90V[a\r\xE0\x81`\x01a\x0B\x82V[\x15\x80a\x0E\xC4W[a\r\xF0WP_\x90V[\x7Fn\xE3\xEF\xEC\xAE\x88=\xF2\xD7\xCC\xDA\"a\x0BL\xA7q\xA2\x99\xE7\x07\xCB\re\xC4\xEC\x97\xDCNfh\xAD\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 `\x01\x80\x86R\x81\x84R\x82\x86 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x89T\x81\x16\x88\x17\x90\x99U\x98\x90\x96\x16\x80\x85R\x92\x82R\x80\x84 \x97\x84R\x96\x81R\x86\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U\x81\x80R\x92\x90\x91R\x92\x90\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U[`\x01T`\x01\x81\x01\x80\x91\x11a\x03\xDAW`\x01U`\x01\x90V[Pa\x0E\xD0_`\x01a\x0B\x82V[a\r\xE7V[a\x0E\xE0\x81`\x01a\x0B\x82V[\x15\x80a\x0F\xADW[a\x0E\xF0WP_\x90V[\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9D\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x88T\x81\x16\x87\x17\x90\x98U\x97\x90\x95\x16\x80\x84R\x91\x81R\x84\x83 \x83\x80R\x81R\x84\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U`\x01\x82R\x94\x90\x91R \x80T\x90\x91\x16\x90\x91\x17\x90Ua\x0E\xAEV[Pa\x0F\xB9_`\x01a\x0B\x82V[a\x0E\xE7V",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RequirementModuleUpdated(address)` and selector `0x253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b9`.
```solidity
event RequirementModuleUpdated(address indexed newModule);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RequirementModuleUpdated {
        #[allow(missing_docs)]
        pub newModule: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RequirementModuleUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RequirementModuleUpdated(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                37u8, 53u8, 128u8, 248u8, 6u8, 116u8, 28u8, 17u8, 179u8, 212u8, 170u8,
                96u8, 217u8, 202u8, 204u8, 91u8, 239u8, 12u8, 235u8, 179u8, 87u8, 72u8,
                118u8, 127u8, 226u8, 63u8, 17u8, 145u8, 110u8, 47u8, 4u8, 185u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newModule: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.newModule.clone())
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
                    &self.newModule,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RequirementModuleUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RequirementModuleUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RequirementModuleUpdated,
            ) -> alloy_sol_types::private::LogData {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Function with signature `masterModule()` and selector `0xcdc4a3be`.
```solidity
function masterModule() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct masterModuleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`masterModule()`](masterModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct masterModuleReturn {
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
            impl ::core::convert::From<masterModuleCall> for UnderlyingRustTuple<'_> {
                fn from(value: masterModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for masterModuleCall {
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
            impl ::core::convert::From<masterModuleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: masterModuleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for masterModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for masterModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "masterModule()";
            const SELECTOR: [u8; 4] = [205u8, 196u8, 163u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: masterModuleReturn = r.into();
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
                        let r: masterModuleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `mockModule()` and selector `0x4359a74e`.
```solidity
function mockModule() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mockModuleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`mockModule()`](mockModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mockModuleReturn {
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
            impl ::core::convert::From<mockModuleCall> for UnderlyingRustTuple<'_> {
                fn from(value: mockModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mockModuleCall {
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
            impl ::core::convert::From<mockModuleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mockModuleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mockModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mockModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mockModule()";
            const SELECTOR: [u8; 4] = [67u8, 89u8, 167u8, 78u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: mockModuleReturn = r.into();
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
                        let r: mockModuleReturn = r.into();
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Function with signature `testCannotInitializeTwice()` and selector `0x08887aea`.
```solidity
function testCannotInitializeTwice() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotInitializeTwiceCall;
    ///Container type for the return parameters of the [`testCannotInitializeTwice()`](testCannotInitializeTwiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotInitializeTwiceReturn {}
    #[allow(
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
            impl ::core::convert::From<testCannotInitializeTwiceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotInitializeTwiceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotInitializeTwiceCall {
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
            impl ::core::convert::From<testCannotInitializeTwiceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotInitializeTwiceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotInitializeTwiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testCannotInitializeTwiceReturn {
            fn _tokenize(
                &self,
            ) -> <testCannotInitializeTwiceCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testCannotInitializeTwiceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testCannotInitializeTwiceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testCannotInitializeTwice()";
            const SELECTOR: [u8; 4] = [8u8, 136u8, 122u8, 234u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testCannotInitializeTwiceReturn::_tokenize(ret)
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
    /**Function with signature `testFuzzDataSizeLimit(uint256)` and selector `0x44768da3`.
```solidity
function testFuzzDataSizeLimit(uint256 dataSize) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzDataSizeLimitCall {
        #[allow(missing_docs)]
        pub dataSize: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzzDataSizeLimit(uint256)`](testFuzzDataSizeLimitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzDataSizeLimitReturn {}
    #[allow(
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
            impl ::core::convert::From<testFuzzDataSizeLimitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzDataSizeLimitCall) -> Self {
                    (value.dataSize,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzDataSizeLimitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataSize: tuple.0 }
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
            impl ::core::convert::From<testFuzzDataSizeLimitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzDataSizeLimitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzDataSizeLimitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzzDataSizeLimitReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzzDataSizeLimitCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzzDataSizeLimitCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzzDataSizeLimitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzzDataSizeLimit(uint256)";
            const SELECTOR: [u8; 4] = [68u8, 118u8, 141u8, 163u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.dataSize),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzzDataSizeLimitReturn::_tokenize(ret)
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
    /**Function with signature `testFuzzDataSizeRevert(uint256)` and selector `0x2abc50bf`.
```solidity
function testFuzzDataSizeRevert(uint256 dataSize) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzDataSizeRevertCall {
        #[allow(missing_docs)]
        pub dataSize: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzzDataSizeRevert(uint256)`](testFuzzDataSizeRevertCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzDataSizeRevertReturn {}
    #[allow(
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
            impl ::core::convert::From<testFuzzDataSizeRevertCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzDataSizeRevertCall) -> Self {
                    (value.dataSize,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzDataSizeRevertCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataSize: tuple.0 }
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
            impl ::core::convert::From<testFuzzDataSizeRevertReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzDataSizeRevertReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzDataSizeRevertReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzzDataSizeRevertReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzzDataSizeRevertCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzzDataSizeRevertCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzzDataSizeRevertReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzzDataSizeRevert(uint256)";
            const SELECTOR: [u8; 4] = [42u8, 188u8, 80u8, 191u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.dataSize),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzzDataSizeRevertReturn::_tokenize(ret)
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
    /**Function with signature `testFuzzIsAllowedWithVariousAddresses(address,address)` and selector `0xbbb9bf85`.
```solidity
function testFuzzIsAllowedWithVariousAddresses(address proposer, address originator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzIsAllowedWithVariousAddressesCall {
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub originator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`testFuzzIsAllowedWithVariousAddresses(address,address)`](testFuzzIsAllowedWithVariousAddressesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzIsAllowedWithVariousAddressesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<testFuzzIsAllowedWithVariousAddressesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzIsAllowedWithVariousAddressesCall) -> Self {
                    (value.proposer, value.originator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzIsAllowedWithVariousAddressesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposer: tuple.0,
                        originator: tuple.1,
                    }
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
            impl ::core::convert::From<testFuzzIsAllowedWithVariousAddressesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzIsAllowedWithVariousAddressesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzIsAllowedWithVariousAddressesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzzIsAllowedWithVariousAddressesReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzzIsAllowedWithVariousAddressesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzzIsAllowedWithVariousAddressesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzzIsAllowedWithVariousAddressesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzzIsAllowedWithVariousAddresses(address,address)";
            const SELECTOR: [u8; 4] = [187u8, 185u8, 191u8, 133u8];
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
                        &self.proposer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.originator,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzzIsAllowedWithVariousAddressesReturn::_tokenize(ret)
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
    /**Function with signature `testInitialization()` and selector `0xa109f39a`.
```solidity
function testInitialization() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInitializationCall;
    ///Container type for the return parameters of the [`testInitialization()`](testInitializationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInitializationReturn {}
    #[allow(
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
            impl ::core::convert::From<testInitializationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInitializationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInitializationCall {
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
            impl ::core::convert::From<testInitializationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInitializationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInitializationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testInitializationReturn {
            fn _tokenize(
                &self,
            ) -> <testInitializationCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testInitializationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testInitializationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testInitialization()";
            const SELECTOR: [u8; 4] = [161u8, 9u8, 243u8, 154u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testInitializationReturn::_tokenize(ret)
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
    /**Function with signature `testInitializeWithDifferentModule()` and selector `0x626d0766`.
```solidity
function testInitializeWithDifferentModule() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInitializeWithDifferentModuleCall;
    ///Container type for the return parameters of the [`testInitializeWithDifferentModule()`](testInitializeWithDifferentModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInitializeWithDifferentModuleReturn {}
    #[allow(
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
            impl ::core::convert::From<testInitializeWithDifferentModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInitializeWithDifferentModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInitializeWithDifferentModuleCall {
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
            impl ::core::convert::From<testInitializeWithDifferentModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInitializeWithDifferentModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInitializeWithDifferentModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testInitializeWithDifferentModuleReturn {
            fn _tokenize(
                &self,
            ) -> <testInitializeWithDifferentModuleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testInitializeWithDifferentModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testInitializeWithDifferentModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testInitializeWithDifferentModule()";
            const SELECTOR: [u8; 4] = [98u8, 109u8, 7u8, 102u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testInitializeWithDifferentModuleReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithAddressOne()` and selector `0xb1a63516`.
```solidity
function testIsAllowedWithAddressOne() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithAddressOneCall;
    ///Container type for the return parameters of the [`testIsAllowedWithAddressOne()`](testIsAllowedWithAddressOneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithAddressOneReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithAddressOneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithAddressOneCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithAddressOneCall {
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
            impl ::core::convert::From<testIsAllowedWithAddressOneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithAddressOneReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithAddressOneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithAddressOneReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithAddressOneCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithAddressOneCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithAddressOneReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithAddressOne()";
            const SELECTOR: [u8; 4] = [177u8, 166u8, 53u8, 22u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithAddressOneReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithEmptyData()` and selector `0x9d4aa91c`.
```solidity
function testIsAllowedWithEmptyData() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithEmptyDataCall;
    ///Container type for the return parameters of the [`testIsAllowedWithEmptyData()`](testIsAllowedWithEmptyDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithEmptyDataReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithEmptyDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithEmptyDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithEmptyDataCall {
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
            impl ::core::convert::From<testIsAllowedWithEmptyDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithEmptyDataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithEmptyDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithEmptyDataReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithEmptyDataCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithEmptyDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithEmptyDataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithEmptyData()";
            const SELECTOR: [u8; 4] = [157u8, 74u8, 169u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithEmptyDataReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithLargeData()` and selector `0x42a85af5`.
```solidity
function testIsAllowedWithLargeData() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithLargeDataCall;
    ///Container type for the return parameters of the [`testIsAllowedWithLargeData()`](testIsAllowedWithLargeDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithLargeDataReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithLargeDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithLargeDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithLargeDataCall {
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
            impl ::core::convert::From<testIsAllowedWithLargeDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithLargeDataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithLargeDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithLargeDataReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithLargeDataCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithLargeDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithLargeDataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithLargeData()";
            const SELECTOR: [u8; 4] = [66u8, 168u8, 90u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithLargeDataReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithMaxSizeData()` and selector `0x31c956a4`.
```solidity
function testIsAllowedWithMaxSizeData() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithMaxSizeDataCall;
    ///Container type for the return parameters of the [`testIsAllowedWithMaxSizeData()`](testIsAllowedWithMaxSizeDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithMaxSizeDataReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithMaxSizeDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithMaxSizeDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithMaxSizeDataCall {
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
            impl ::core::convert::From<testIsAllowedWithMaxSizeDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithMaxSizeDataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithMaxSizeDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithMaxSizeDataReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithMaxSizeDataCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithMaxSizeDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithMaxSizeDataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithMaxSizeData()";
            const SELECTOR: [u8; 4] = [49u8, 201u8, 86u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithMaxSizeDataReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithMockModuleAllow()` and selector `0x7004eac4`.
```solidity
function testIsAllowedWithMockModuleAllow() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithMockModuleAllowCall;
    ///Container type for the return parameters of the [`testIsAllowedWithMockModuleAllow()`](testIsAllowedWithMockModuleAllowCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithMockModuleAllowReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithMockModuleAllowCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithMockModuleAllowCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithMockModuleAllowCall {
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
            impl ::core::convert::From<testIsAllowedWithMockModuleAllowReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithMockModuleAllowReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithMockModuleAllowReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithMockModuleAllowReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithMockModuleAllowCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithMockModuleAllowCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithMockModuleAllowReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithMockModuleAllow()";
            const SELECTOR: [u8; 4] = [112u8, 4u8, 234u8, 196u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithMockModuleAllowReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithMockModuleDeny()` and selector `0x82eddc22`.
```solidity
function testIsAllowedWithMockModuleDeny() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithMockModuleDenyCall;
    ///Container type for the return parameters of the [`testIsAllowedWithMockModuleDeny()`](testIsAllowedWithMockModuleDenyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithMockModuleDenyReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithMockModuleDenyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithMockModuleDenyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithMockModuleDenyCall {
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
            impl ::core::convert::From<testIsAllowedWithMockModuleDenyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithMockModuleDenyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithMockModuleDenyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithMockModuleDenyReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithMockModuleDenyCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithMockModuleDenyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithMockModuleDenyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithMockModuleDeny()";
            const SELECTOR: [u8; 4] = [130u8, 237u8, 220u8, 34u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithMockModuleDenyReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithOversizedData()` and selector `0x2ecdbf56`.
```solidity
function testIsAllowedWithOversizedData() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithOversizedDataCall;
    ///Container type for the return parameters of the [`testIsAllowedWithOversizedData()`](testIsAllowedWithOversizedDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithOversizedDataReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithOversizedDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithOversizedDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithOversizedDataCall {
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
            impl ::core::convert::From<testIsAllowedWithOversizedDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithOversizedDataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithOversizedDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithOversizedDataReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithOversizedDataCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithOversizedDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithOversizedDataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithOversizedData()";
            const SELECTOR: [u8; 4] = [46u8, 205u8, 191u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithOversizedDataReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithRealModule()` and selector `0xae7ed49f`.
```solidity
function testIsAllowedWithRealModule() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithRealModuleCall;
    ///Container type for the return parameters of the [`testIsAllowedWithRealModule()`](testIsAllowedWithRealModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithRealModuleReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithRealModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithRealModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithRealModuleCall {
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
            impl ::core::convert::From<testIsAllowedWithRealModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithRealModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithRealModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithRealModuleReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithRealModuleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithRealModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithRealModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithRealModule()";
            const SELECTOR: [u8; 4] = [174u8, 126u8, 212u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithRealModuleReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedWithZeroAddress()` and selector `0x4c58ef7b`.
```solidity
function testIsAllowedWithZeroAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithZeroAddressCall;
    ///Container type for the return parameters of the [`testIsAllowedWithZeroAddress()`](testIsAllowedWithZeroAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedWithZeroAddressReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedWithZeroAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithZeroAddressCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithZeroAddressCall {
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
            impl ::core::convert::From<testIsAllowedWithZeroAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedWithZeroAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedWithZeroAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedWithZeroAddressReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedWithZeroAddressCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedWithZeroAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedWithZeroAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedWithZeroAddress()";
            const SELECTOR: [u8; 4] = [76u8, 88u8, 239u8, 123u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedWithZeroAddressReturn::_tokenize(ret)
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
    /**Function with signature `testMaxDataSizeConstant()` and selector `0x629fe631`.
```solidity
function testMaxDataSizeConstant() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMaxDataSizeConstantCall;
    ///Container type for the return parameters of the [`testMaxDataSizeConstant()`](testMaxDataSizeConstantCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMaxDataSizeConstantReturn {}
    #[allow(
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
            impl ::core::convert::From<testMaxDataSizeConstantCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMaxDataSizeConstantCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMaxDataSizeConstantCall {
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
            impl ::core::convert::From<testMaxDataSizeConstantReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMaxDataSizeConstantReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMaxDataSizeConstantReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testMaxDataSizeConstantReturn {
            fn _tokenize(
                &self,
            ) -> <testMaxDataSizeConstantCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testMaxDataSizeConstantCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testMaxDataSizeConstantReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testMaxDataSizeConstant()";
            const SELECTOR: [u8; 4] = [98u8, 159u8, 230u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testMaxDataSizeConstantReturn::_tokenize(ret)
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
    /**Function with signature `testOnlyOwnerFunctions()` and selector `0xd11187dd`.
```solidity
function testOnlyOwnerFunctions() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOnlyOwnerFunctionsCall;
    ///Container type for the return parameters of the [`testOnlyOwnerFunctions()`](testOnlyOwnerFunctionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOnlyOwnerFunctionsReturn {}
    #[allow(
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
            impl ::core::convert::From<testOnlyOwnerFunctionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOnlyOwnerFunctionsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOnlyOwnerFunctionsCall {
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
            impl ::core::convert::From<testOnlyOwnerFunctionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOnlyOwnerFunctionsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOnlyOwnerFunctionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testOnlyOwnerFunctionsReturn {
            fn _tokenize(
                &self,
            ) -> <testOnlyOwnerFunctionsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testOnlyOwnerFunctionsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testOnlyOwnerFunctionsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testOnlyOwnerFunctions()";
            const SELECTOR: [u8; 4] = [209u8, 17u8, 135u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testOnlyOwnerFunctionsReturn::_tokenize(ret)
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
    /**Function with signature `testOwnershipTransfer()` and selector `0x575fc5d5`.
```solidity
function testOwnershipTransfer() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOwnershipTransferCall;
    ///Container type for the return parameters of the [`testOwnershipTransfer()`](testOwnershipTransferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOwnershipTransferReturn {}
    #[allow(
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
            impl ::core::convert::From<testOwnershipTransferCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOwnershipTransferCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOwnershipTransferCall {
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
            impl ::core::convert::From<testOwnershipTransferReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOwnershipTransferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOwnershipTransferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testOwnershipTransferReturn {
            fn _tokenize(
                &self,
            ) -> <testOwnershipTransferCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testOwnershipTransferCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testOwnershipTransferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testOwnershipTransfer()";
            const SELECTOR: [u8; 4] = [87u8, 95u8, 197u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testOwnershipTransferReturn::_tokenize(ret)
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
    /**Function with signature `testPermissionRequirementModuleGetter()` and selector `0x53874766`.
```solidity
function testPermissionRequirementModuleGetter() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testPermissionRequirementModuleGetterCall;
    ///Container type for the return parameters of the [`testPermissionRequirementModuleGetter()`](testPermissionRequirementModuleGetterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testPermissionRequirementModuleGetterReturn {}
    #[allow(
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
            impl ::core::convert::From<testPermissionRequirementModuleGetterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testPermissionRequirementModuleGetterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testPermissionRequirementModuleGetterCall {
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
            impl ::core::convert::From<testPermissionRequirementModuleGetterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testPermissionRequirementModuleGetterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testPermissionRequirementModuleGetterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testPermissionRequirementModuleGetterReturn {
            fn _tokenize(
                &self,
            ) -> <testPermissionRequirementModuleGetterCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testPermissionRequirementModuleGetterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testPermissionRequirementModuleGetterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testPermissionRequirementModuleGetter()";
            const SELECTOR: [u8; 4] = [83u8, 135u8, 71u8, 102u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testPermissionRequirementModuleGetterReturn::_tokenize(ret)
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
    /**Function with signature `testRenounceOwnership()` and selector `0xf09d1d44`.
```solidity
function testRenounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRenounceOwnershipCall;
    ///Container type for the return parameters of the [`testRenounceOwnership()`](testRenounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRenounceOwnershipReturn {}
    #[allow(
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
            impl ::core::convert::From<testRenounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRenounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRenounceOwnershipCall {
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
            impl ::core::convert::From<testRenounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRenounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRenounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testRenounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <testRenounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testRenounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testRenounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testRenounceOwnership()";
            const SELECTOR: [u8; 4] = [240u8, 157u8, 29u8, 68u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testRenounceOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `testStorageLocation()` and selector `0xd48ca584`.
```solidity
function testStorageLocation() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testStorageLocationCall;
    ///Container type for the return parameters of the [`testStorageLocation()`](testStorageLocationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testStorageLocationReturn {}
    #[allow(
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
            impl ::core::convert::From<testStorageLocationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testStorageLocationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testStorageLocationCall {
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
            impl ::core::convert::From<testStorageLocationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testStorageLocationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testStorageLocationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testStorageLocationReturn {
            fn _tokenize(
                &self,
            ) -> <testStorageLocationCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testStorageLocationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testStorageLocationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testStorageLocation()";
            const SELECTOR: [u8; 4] = [212u8, 140u8, 165u8, 132u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testStorageLocationReturn::_tokenize(ret)
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
    /**Function with signature `testUpdateRequirementModule()` and selector `0x5e5e5f69`.
```solidity
function testUpdateRequirementModule() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateRequirementModuleCall;
    ///Container type for the return parameters of the [`testUpdateRequirementModule()`](testUpdateRequirementModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateRequirementModuleReturn {}
    #[allow(
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
            impl ::core::convert::From<testUpdateRequirementModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateRequirementModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateRequirementModuleCall {
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
            impl ::core::convert::From<testUpdateRequirementModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateRequirementModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateRequirementModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateRequirementModuleReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateRequirementModuleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testUpdateRequirementModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateRequirementModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateRequirementModule()";
            const SELECTOR: [u8; 4] = [94u8, 94u8, 95u8, 105u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testUpdateRequirementModuleReturn::_tokenize(ret)
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
    /**Function with signature `testUpdateRequirementModuleNonAdmin()` and selector `0x51d9d3d5`.
```solidity
function testUpdateRequirementModuleNonAdmin() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateRequirementModuleNonAdminCall;
    ///Container type for the return parameters of the [`testUpdateRequirementModuleNonAdmin()`](testUpdateRequirementModuleNonAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateRequirementModuleNonAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<testUpdateRequirementModuleNonAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateRequirementModuleNonAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateRequirementModuleNonAdminCall {
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
            impl ::core::convert::From<testUpdateRequirementModuleNonAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateRequirementModuleNonAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateRequirementModuleNonAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateRequirementModuleNonAdminReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateRequirementModuleNonAdminCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testUpdateRequirementModuleNonAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateRequirementModuleNonAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateRequirementModuleNonAdmin()";
            const SELECTOR: [u8; 4] = [81u8, 217u8, 211u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testUpdateRequirementModuleNonAdminReturn::_tokenize(ret)
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
    /**Function with signature `testUpdateRequirementModuleToAddressOne()` and selector `0x1436ee7f`.
```solidity
function testUpdateRequirementModuleToAddressOne() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateRequirementModuleToAddressOneCall;
    ///Container type for the return parameters of the [`testUpdateRequirementModuleToAddressOne()`](testUpdateRequirementModuleToAddressOneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateRequirementModuleToAddressOneReturn {}
    #[allow(
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
            impl ::core::convert::From<testUpdateRequirementModuleToAddressOneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateRequirementModuleToAddressOneCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateRequirementModuleToAddressOneCall {
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
            impl ::core::convert::From<testUpdateRequirementModuleToAddressOneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateRequirementModuleToAddressOneReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateRequirementModuleToAddressOneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateRequirementModuleToAddressOneReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateRequirementModuleToAddressOneCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testUpdateRequirementModuleToAddressOneCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateRequirementModuleToAddressOneReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateRequirementModuleToAddressOne()";
            const SELECTOR: [u8; 4] = [20u8, 54u8, 238u8, 127u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testUpdateRequirementModuleToAddressOneReturn::_tokenize(ret)
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
    /**Function with signature `testUpdateRequirementModuleToZeroAddress()` and selector `0xa6b3e93d`.
```solidity
function testUpdateRequirementModuleToZeroAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateRequirementModuleToZeroAddressCall;
    ///Container type for the return parameters of the [`testUpdateRequirementModuleToZeroAddress()`](testUpdateRequirementModuleToZeroAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateRequirementModuleToZeroAddressReturn {}
    #[allow(
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
            impl ::core::convert::From<testUpdateRequirementModuleToZeroAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateRequirementModuleToZeroAddressCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateRequirementModuleToZeroAddressCall {
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
            impl ::core::convert::From<testUpdateRequirementModuleToZeroAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateRequirementModuleToZeroAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateRequirementModuleToZeroAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateRequirementModuleToZeroAddressReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateRequirementModuleToZeroAddressCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testUpdateRequirementModuleToZeroAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateRequirementModuleToZeroAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateRequirementModuleToZeroAddress()";
            const SELECTOR: [u8; 4] = [166u8, 179u8, 233u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testUpdateRequirementModuleToZeroAddressReturn::_tokenize(ret)
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
    /**Function with signature `testWithRequireAndModule()` and selector `0x192c28af`.
```solidity
function testWithRequireAndModule() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testWithRequireAndModuleCall;
    ///Container type for the return parameters of the [`testWithRequireAndModule()`](testWithRequireAndModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testWithRequireAndModuleReturn {}
    #[allow(
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
            impl ::core::convert::From<testWithRequireAndModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testWithRequireAndModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testWithRequireAndModuleCall {
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
            impl ::core::convert::From<testWithRequireAndModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testWithRequireAndModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testWithRequireAndModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testWithRequireAndModuleReturn {
            fn _tokenize(
                &self,
            ) -> <testWithRequireAndModuleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testWithRequireAndModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testWithRequireAndModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testWithRequireAndModule()";
            const SELECTOR: [u8; 4] = [25u8, 44u8, 40u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testWithRequireAndModuleReturn::_tokenize(ret)
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
    /**Function with signature `testWithRequireOrModule()` and selector `0x716e2aa8`.
```solidity
function testWithRequireOrModule() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testWithRequireOrModuleCall;
    ///Container type for the return parameters of the [`testWithRequireOrModule()`](testWithRequireOrModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testWithRequireOrModuleReturn {}
    #[allow(
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
            impl ::core::convert::From<testWithRequireOrModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testWithRequireOrModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testWithRequireOrModuleCall {
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
            impl ::core::convert::From<testWithRequireOrModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testWithRequireOrModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testWithRequireOrModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testWithRequireOrModuleReturn {
            fn _tokenize(
                &self,
            ) -> <testWithRequireOrModuleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testWithRequireOrModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testWithRequireOrModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testWithRequireOrModule()";
            const SELECTOR: [u8; 4] = [113u8, 110u8, 42u8, 168u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testWithRequireOrModuleReturn::_tokenize(ret)
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
    /**Function with signature `user1()` and selector `0xac1717b0`.
```solidity
function user1() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct user1Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`user1()`](user1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct user1Return {
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
            impl ::core::convert::From<user1Call> for UnderlyingRustTuple<'_> {
                fn from(value: user1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for user1Call {
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
            impl ::core::convert::From<user1Return> for UnderlyingRustTuple<'_> {
                fn from(value: user1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for user1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for user1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "user1()";
            const SELECTOR: [u8; 4] = [172u8, 23u8, 23u8, 176u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: user1Return = r.into();
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
                        let r: user1Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `user2()` and selector `0xb9edb1af`.
```solidity
function user2() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct user2Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`user2()`](user2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct user2Return {
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
            impl ::core::convert::From<user2Call> for UnderlyingRustTuple<'_> {
                fn from(value: user2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for user2Call {
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
            impl ::core::convert::From<user2Return> for UnderlyingRustTuple<'_> {
                fn from(value: user2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for user2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for user2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "user2()";
            const SELECTOR: [u8; 4] = [185u8, 237u8, 177u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: user2Return = r.into();
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
                        let r: user2Return = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`SequencingModuleCheckerTest`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SequencingModuleCheckerTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        admin(adminCall),
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
        masterModule(masterModuleCall),
        #[allow(missing_docs)]
        mockModule(mockModuleCall),
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
        testCannotInitializeTwice(testCannotInitializeTwiceCall),
        #[allow(missing_docs)]
        testFuzzDataSizeLimit(testFuzzDataSizeLimitCall),
        #[allow(missing_docs)]
        testFuzzDataSizeRevert(testFuzzDataSizeRevertCall),
        #[allow(missing_docs)]
        testFuzzIsAllowedWithVariousAddresses(testFuzzIsAllowedWithVariousAddressesCall),
        #[allow(missing_docs)]
        testInitialization(testInitializationCall),
        #[allow(missing_docs)]
        testInitializeWithDifferentModule(testInitializeWithDifferentModuleCall),
        #[allow(missing_docs)]
        testIsAllowedWithAddressOne(testIsAllowedWithAddressOneCall),
        #[allow(missing_docs)]
        testIsAllowedWithEmptyData(testIsAllowedWithEmptyDataCall),
        #[allow(missing_docs)]
        testIsAllowedWithLargeData(testIsAllowedWithLargeDataCall),
        #[allow(missing_docs)]
        testIsAllowedWithMaxSizeData(testIsAllowedWithMaxSizeDataCall),
        #[allow(missing_docs)]
        testIsAllowedWithMockModuleAllow(testIsAllowedWithMockModuleAllowCall),
        #[allow(missing_docs)]
        testIsAllowedWithMockModuleDeny(testIsAllowedWithMockModuleDenyCall),
        #[allow(missing_docs)]
        testIsAllowedWithOversizedData(testIsAllowedWithOversizedDataCall),
        #[allow(missing_docs)]
        testIsAllowedWithRealModule(testIsAllowedWithRealModuleCall),
        #[allow(missing_docs)]
        testIsAllowedWithZeroAddress(testIsAllowedWithZeroAddressCall),
        #[allow(missing_docs)]
        testMaxDataSizeConstant(testMaxDataSizeConstantCall),
        #[allow(missing_docs)]
        testOnlyOwnerFunctions(testOnlyOwnerFunctionsCall),
        #[allow(missing_docs)]
        testOwnershipTransfer(testOwnershipTransferCall),
        #[allow(missing_docs)]
        testPermissionRequirementModuleGetter(testPermissionRequirementModuleGetterCall),
        #[allow(missing_docs)]
        testRenounceOwnership(testRenounceOwnershipCall),
        #[allow(missing_docs)]
        testStorageLocation(testStorageLocationCall),
        #[allow(missing_docs)]
        testUpdateRequirementModule(testUpdateRequirementModuleCall),
        #[allow(missing_docs)]
        testUpdateRequirementModuleNonAdmin(testUpdateRequirementModuleNonAdminCall),
        #[allow(missing_docs)]
        testUpdateRequirementModuleToAddressOne(
            testUpdateRequirementModuleToAddressOneCall,
        ),
        #[allow(missing_docs)]
        testUpdateRequirementModuleToZeroAddress(
            testUpdateRequirementModuleToZeroAddressCall,
        ),
        #[allow(missing_docs)]
        testWithRequireAndModule(testWithRequireAndModuleCall),
        #[allow(missing_docs)]
        testWithRequireOrModule(testWithRequireOrModuleCall),
        #[allow(missing_docs)]
        user1(user1Call),
        #[allow(missing_docs)]
        user2(user2Call),
    }
    #[automatically_derived]
    impl SequencingModuleCheckerTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 136u8, 122u8, 234u8],
            [10u8, 146u8, 84u8, 228u8],
            [20u8, 54u8, 238u8, 127u8],
            [25u8, 44u8, 40u8, 175u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 188u8, 80u8, 191u8],
            [42u8, 222u8, 56u8, 128u8],
            [46u8, 205u8, 191u8, 86u8],
            [49u8, 201u8, 86u8, 164u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [66u8, 168u8, 90u8, 245u8],
            [67u8, 89u8, 167u8, 78u8],
            [68u8, 118u8, 141u8, 163u8],
            [72u8, 28u8, 106u8, 117u8],
            [76u8, 88u8, 239u8, 123u8],
            [81u8, 217u8, 211u8, 213u8],
            [83u8, 135u8, 71u8, 102u8],
            [87u8, 95u8, 197u8, 213u8],
            [94u8, 94u8, 95u8, 105u8],
            [98u8, 109u8, 7u8, 102u8],
            [98u8, 159u8, 230u8, 49u8],
            [102u8, 217u8, 169u8, 160u8],
            [112u8, 4u8, 234u8, 196u8],
            [113u8, 110u8, 42u8, 168u8],
            [130u8, 237u8, 220u8, 34u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [157u8, 74u8, 169u8, 28u8],
            [161u8, 9u8, 243u8, 154u8],
            [166u8, 179u8, 233u8, 61u8],
            [172u8, 23u8, 23u8, 176u8],
            [174u8, 126u8, 212u8, 159u8],
            [176u8, 70u8, 79u8, 220u8],
            [177u8, 166u8, 53u8, 22u8],
            [181u8, 80u8, 138u8, 169u8],
            [185u8, 237u8, 177u8, 175u8],
            [186u8, 65u8, 79u8, 166u8],
            [187u8, 185u8, 191u8, 133u8],
            [193u8, 46u8, 70u8, 135u8],
            [205u8, 196u8, 163u8, 190u8],
            [209u8, 17u8, 135u8, 221u8],
            [212u8, 140u8, 165u8, 132u8],
            [226u8, 12u8, 159u8, 113u8],
            [240u8, 157u8, 29u8, 68u8],
            [248u8, 81u8, 164u8, 64u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SequencingModuleCheckerTestCalls {
        const NAME: &'static str = "SequencingModuleCheckerTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 47usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::admin(_) => <adminCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::masterModule(_) => {
                    <masterModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mockModule(_) => {
                    <mockModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::testCannotInitializeTwice(_) => {
                    <testCannotInitializeTwiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzzDataSizeLimit(_) => {
                    <testFuzzDataSizeLimitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzzDataSizeRevert(_) => {
                    <testFuzzDataSizeRevertCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzzIsAllowedWithVariousAddresses(_) => {
                    <testFuzzIsAllowedWithVariousAddressesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testInitialization(_) => {
                    <testInitializationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testInitializeWithDifferentModule(_) => {
                    <testInitializeWithDifferentModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithAddressOne(_) => {
                    <testIsAllowedWithAddressOneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithEmptyData(_) => {
                    <testIsAllowedWithEmptyDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithLargeData(_) => {
                    <testIsAllowedWithLargeDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithMaxSizeData(_) => {
                    <testIsAllowedWithMaxSizeDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithMockModuleAllow(_) => {
                    <testIsAllowedWithMockModuleAllowCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithMockModuleDeny(_) => {
                    <testIsAllowedWithMockModuleDenyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithOversizedData(_) => {
                    <testIsAllowedWithOversizedDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithRealModule(_) => {
                    <testIsAllowedWithRealModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedWithZeroAddress(_) => {
                    <testIsAllowedWithZeroAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testMaxDataSizeConstant(_) => {
                    <testMaxDataSizeConstantCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testOnlyOwnerFunctions(_) => {
                    <testOnlyOwnerFunctionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testOwnershipTransfer(_) => {
                    <testOwnershipTransferCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testPermissionRequirementModuleGetter(_) => {
                    <testPermissionRequirementModuleGetterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testRenounceOwnership(_) => {
                    <testRenounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testStorageLocation(_) => {
                    <testStorageLocationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateRequirementModule(_) => {
                    <testUpdateRequirementModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateRequirementModuleNonAdmin(_) => {
                    <testUpdateRequirementModuleNonAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateRequirementModuleToAddressOne(_) => {
                    <testUpdateRequirementModuleToAddressOneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateRequirementModuleToZeroAddress(_) => {
                    <testUpdateRequirementModuleToZeroAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testWithRequireAndModule(_) => {
                    <testWithRequireAndModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testWithRequireOrModule(_) => {
                    <testWithRequireOrModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::user1(_) => <user1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::user2(_) => <user2Call as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls>] = &[
                {
                    fn testCannotInitializeTwice(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testCannotInitializeTwiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testCannotInitializeTwice,
                            )
                    }
                    testCannotInitializeTwice
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SequencingModuleCheckerTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn testUpdateRequirementModuleToAddressOne(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testUpdateRequirementModuleToAddressOneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testUpdateRequirementModuleToAddressOne,
                            )
                    }
                    testUpdateRequirementModuleToAddressOne
                },
                {
                    fn testWithRequireAndModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testWithRequireAndModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testWithRequireAndModule,
                            )
                    }
                    testWithRequireAndModule
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn testFuzzDataSizeRevert(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testFuzzDataSizeRevertCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testFuzzDataSizeRevert,
                            )
                    }
                    testFuzzDataSizeRevert
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testIsAllowedWithOversizedData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithOversizedDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithOversizedData,
                            )
                    }
                    testIsAllowedWithOversizedData
                },
                {
                    fn testIsAllowedWithMaxSizeData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithMaxSizeDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithMaxSizeData,
                            )
                    }
                    testIsAllowedWithMaxSizeData
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn testIsAllowedWithLargeData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithLargeDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithLargeData,
                            )
                    }
                    testIsAllowedWithLargeData
                },
                {
                    fn mockModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <mockModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::mockModule)
                    }
                    mockModule
                },
                {
                    fn testFuzzDataSizeLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testFuzzDataSizeLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testFuzzDataSizeLimit)
                    }
                    testFuzzDataSizeLimit
                },
                {
                    fn manager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <managerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SequencingModuleCheckerTestCalls::manager)
                    }
                    manager
                },
                {
                    fn testIsAllowedWithZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithZeroAddress,
                            )
                    }
                    testIsAllowedWithZeroAddress
                },
                {
                    fn testUpdateRequirementModuleNonAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testUpdateRequirementModuleNonAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testUpdateRequirementModuleNonAdmin,
                            )
                    }
                    testUpdateRequirementModuleNonAdmin
                },
                {
                    fn testPermissionRequirementModuleGetter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testPermissionRequirementModuleGetterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testPermissionRequirementModuleGetter,
                            )
                    }
                    testPermissionRequirementModuleGetter
                },
                {
                    fn testOwnershipTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testOwnershipTransferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testOwnershipTransfer)
                    }
                    testOwnershipTransfer
                },
                {
                    fn testUpdateRequirementModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testUpdateRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testUpdateRequirementModule,
                            )
                    }
                    testUpdateRequirementModule
                },
                {
                    fn testInitializeWithDifferentModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testInitializeWithDifferentModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testInitializeWithDifferentModule,
                            )
                    }
                    testInitializeWithDifferentModule
                },
                {
                    fn testMaxDataSizeConstant(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testMaxDataSizeConstantCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testMaxDataSizeConstant,
                            )
                    }
                    testMaxDataSizeConstant
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::targetArtifactSelectors,
                            )
                    }
                    targetArtifactSelectors
                },
                {
                    fn testIsAllowedWithMockModuleAllow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithMockModuleAllowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithMockModuleAllow,
                            )
                    }
                    testIsAllowedWithMockModuleAllow
                },
                {
                    fn testWithRequireOrModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testWithRequireOrModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testWithRequireOrModule,
                            )
                    }
                    testWithRequireOrModule
                },
                {
                    fn testIsAllowedWithMockModuleDeny(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithMockModuleDenyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithMockModuleDeny,
                            )
                    }
                    testIsAllowedWithMockModuleDeny
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testIsAllowedWithEmptyData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithEmptyDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithEmptyData,
                            )
                    }
                    testIsAllowedWithEmptyData
                },
                {
                    fn testInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testInitializationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testInitialization)
                    }
                    testInitialization
                },
                {
                    fn testUpdateRequirementModuleToZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testUpdateRequirementModuleToZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testUpdateRequirementModuleToZeroAddress,
                            )
                    }
                    testUpdateRequirementModuleToZeroAddress
                },
                {
                    fn user1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <user1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SequencingModuleCheckerTestCalls::user1)
                    }
                    user1
                },
                {
                    fn testIsAllowedWithRealModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithRealModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithRealModule,
                            )
                    }
                    testIsAllowedWithRealModule
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn testIsAllowedWithAddressOne(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithAddressOneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithAddressOne,
                            )
                    }
                    testIsAllowedWithAddressOne
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn user2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <user2Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SequencingModuleCheckerTestCalls::user2)
                    }
                    user2
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SequencingModuleCheckerTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testFuzzIsAllowedWithVariousAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testFuzzIsAllowedWithVariousAddressesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testFuzzIsAllowedWithVariousAddresses,
                            )
                    }
                    testFuzzIsAllowedWithVariousAddresses
                },
                {
                    fn nonAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <nonAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SequencingModuleCheckerTestCalls::nonAdmin)
                    }
                    nonAdmin
                },
                {
                    fn masterModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <masterModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::masterModule)
                    }
                    masterModule
                },
                {
                    fn testOnlyOwnerFunctions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testOnlyOwnerFunctionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testOnlyOwnerFunctions,
                            )
                    }
                    testOnlyOwnerFunctions
                },
                {
                    fn testStorageLocation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testStorageLocationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testStorageLocation)
                    }
                    testStorageLocation
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testRenounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testRenounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testRenounceOwnership)
                    }
                    testRenounceOwnership
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SequencingModuleCheckerTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SequencingModuleCheckerTestCalls::IS_TEST)
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
            ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls>] = &[
                {
                    fn testCannotInitializeTwice(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testCannotInitializeTwiceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testCannotInitializeTwice,
                            )
                    }
                    testCannotInitializeTwice
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn testUpdateRequirementModuleToAddressOne(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testUpdateRequirementModuleToAddressOneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testUpdateRequirementModuleToAddressOne,
                            )
                    }
                    testUpdateRequirementModuleToAddressOne
                },
                {
                    fn testWithRequireAndModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testWithRequireAndModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testWithRequireAndModule,
                            )
                    }
                    testWithRequireAndModule
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn testFuzzDataSizeRevert(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testFuzzDataSizeRevertCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testFuzzDataSizeRevert,
                            )
                    }
                    testFuzzDataSizeRevert
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testIsAllowedWithOversizedData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithOversizedDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithOversizedData,
                            )
                    }
                    testIsAllowedWithOversizedData
                },
                {
                    fn testIsAllowedWithMaxSizeData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithMaxSizeDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithMaxSizeData,
                            )
                    }
                    testIsAllowedWithMaxSizeData
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn testIsAllowedWithLargeData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithLargeDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithLargeData,
                            )
                    }
                    testIsAllowedWithLargeData
                },
                {
                    fn mockModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <mockModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::mockModule)
                    }
                    mockModule
                },
                {
                    fn testFuzzDataSizeLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testFuzzDataSizeLimitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testFuzzDataSizeLimit)
                    }
                    testFuzzDataSizeLimit
                },
                {
                    fn manager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <managerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::manager)
                    }
                    manager
                },
                {
                    fn testIsAllowedWithZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithZeroAddress,
                            )
                    }
                    testIsAllowedWithZeroAddress
                },
                {
                    fn testUpdateRequirementModuleNonAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testUpdateRequirementModuleNonAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testUpdateRequirementModuleNonAdmin,
                            )
                    }
                    testUpdateRequirementModuleNonAdmin
                },
                {
                    fn testPermissionRequirementModuleGetter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testPermissionRequirementModuleGetterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testPermissionRequirementModuleGetter,
                            )
                    }
                    testPermissionRequirementModuleGetter
                },
                {
                    fn testOwnershipTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testOwnershipTransferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testOwnershipTransfer)
                    }
                    testOwnershipTransfer
                },
                {
                    fn testUpdateRequirementModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testUpdateRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testUpdateRequirementModule,
                            )
                    }
                    testUpdateRequirementModule
                },
                {
                    fn testInitializeWithDifferentModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testInitializeWithDifferentModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testInitializeWithDifferentModule,
                            )
                    }
                    testInitializeWithDifferentModule
                },
                {
                    fn testMaxDataSizeConstant(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testMaxDataSizeConstantCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testMaxDataSizeConstant,
                            )
                    }
                    testMaxDataSizeConstant
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::targetArtifactSelectors,
                            )
                    }
                    targetArtifactSelectors
                },
                {
                    fn testIsAllowedWithMockModuleAllow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithMockModuleAllowCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithMockModuleAllow,
                            )
                    }
                    testIsAllowedWithMockModuleAllow
                },
                {
                    fn testWithRequireOrModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testWithRequireOrModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testWithRequireOrModule,
                            )
                    }
                    testWithRequireOrModule
                },
                {
                    fn testIsAllowedWithMockModuleDeny(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithMockModuleDenyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithMockModuleDeny,
                            )
                    }
                    testIsAllowedWithMockModuleDeny
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testIsAllowedWithEmptyData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithEmptyDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithEmptyData,
                            )
                    }
                    testIsAllowedWithEmptyData
                },
                {
                    fn testInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testInitializationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testInitialization)
                    }
                    testInitialization
                },
                {
                    fn testUpdateRequirementModuleToZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testUpdateRequirementModuleToZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testUpdateRequirementModuleToZeroAddress,
                            )
                    }
                    testUpdateRequirementModuleToZeroAddress
                },
                {
                    fn user1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <user1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::user1)
                    }
                    user1
                },
                {
                    fn testIsAllowedWithRealModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithRealModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithRealModule,
                            )
                    }
                    testIsAllowedWithRealModule
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn testIsAllowedWithAddressOne(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testIsAllowedWithAddressOneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testIsAllowedWithAddressOne,
                            )
                    }
                    testIsAllowedWithAddressOne
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn user2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <user2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::user2)
                    }
                    user2
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testFuzzIsAllowedWithVariousAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testFuzzIsAllowedWithVariousAddressesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testFuzzIsAllowedWithVariousAddresses,
                            )
                    }
                    testFuzzIsAllowedWithVariousAddresses
                },
                {
                    fn nonAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <nonAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::nonAdmin)
                    }
                    nonAdmin
                },
                {
                    fn masterModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <masterModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::masterModule)
                    }
                    masterModule
                },
                {
                    fn testOnlyOwnerFunctions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testOnlyOwnerFunctionsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SequencingModuleCheckerTestCalls::testOnlyOwnerFunctions,
                            )
                    }
                    testOnlyOwnerFunctions
                },
                {
                    fn testStorageLocation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testStorageLocationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testStorageLocation)
                    }
                    testStorageLocation
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testRenounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <testRenounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::testRenounceOwnership)
                    }
                    testRenounceOwnership
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SequencingModuleCheckerTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SequencingModuleCheckerTestCalls::IS_TEST)
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
                Self::masterModule(inner) => {
                    <masterModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mockModule(inner) => {
                    <mockModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::testCannotInitializeTwice(inner) => {
                    <testCannotInitializeTwiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzzDataSizeLimit(inner) => {
                    <testFuzzDataSizeLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzzDataSizeRevert(inner) => {
                    <testFuzzDataSizeRevertCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzzIsAllowedWithVariousAddresses(inner) => {
                    <testFuzzIsAllowedWithVariousAddressesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testInitialization(inner) => {
                    <testInitializationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testInitializeWithDifferentModule(inner) => {
                    <testInitializeWithDifferentModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithAddressOne(inner) => {
                    <testIsAllowedWithAddressOneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithEmptyData(inner) => {
                    <testIsAllowedWithEmptyDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithLargeData(inner) => {
                    <testIsAllowedWithLargeDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithMaxSizeData(inner) => {
                    <testIsAllowedWithMaxSizeDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithMockModuleAllow(inner) => {
                    <testIsAllowedWithMockModuleAllowCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithMockModuleDeny(inner) => {
                    <testIsAllowedWithMockModuleDenyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithOversizedData(inner) => {
                    <testIsAllowedWithOversizedDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithRealModule(inner) => {
                    <testIsAllowedWithRealModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedWithZeroAddress(inner) => {
                    <testIsAllowedWithZeroAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testMaxDataSizeConstant(inner) => {
                    <testMaxDataSizeConstantCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testOnlyOwnerFunctions(inner) => {
                    <testOnlyOwnerFunctionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testOwnershipTransfer(inner) => {
                    <testOwnershipTransferCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testPermissionRequirementModuleGetter(inner) => {
                    <testPermissionRequirementModuleGetterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testRenounceOwnership(inner) => {
                    <testRenounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testStorageLocation(inner) => {
                    <testStorageLocationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateRequirementModule(inner) => {
                    <testUpdateRequirementModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateRequirementModuleNonAdmin(inner) => {
                    <testUpdateRequirementModuleNonAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateRequirementModuleToAddressOne(inner) => {
                    <testUpdateRequirementModuleToAddressOneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateRequirementModuleToZeroAddress(inner) => {
                    <testUpdateRequirementModuleToZeroAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testWithRequireAndModule(inner) => {
                    <testWithRequireAndModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testWithRequireOrModule(inner) => {
                    <testWithRequireOrModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::user1(inner) => {
                    <user1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::user2(inner) => {
                    <user2Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::masterModule(inner) => {
                    <masterModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mockModule(inner) => {
                    <mockModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::testCannotInitializeTwice(inner) => {
                    <testCannotInitializeTwiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzzDataSizeLimit(inner) => {
                    <testFuzzDataSizeLimitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzzDataSizeRevert(inner) => {
                    <testFuzzDataSizeRevertCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzzIsAllowedWithVariousAddresses(inner) => {
                    <testFuzzIsAllowedWithVariousAddressesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testInitialization(inner) => {
                    <testInitializationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testInitializeWithDifferentModule(inner) => {
                    <testInitializeWithDifferentModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithAddressOne(inner) => {
                    <testIsAllowedWithAddressOneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithEmptyData(inner) => {
                    <testIsAllowedWithEmptyDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithLargeData(inner) => {
                    <testIsAllowedWithLargeDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithMaxSizeData(inner) => {
                    <testIsAllowedWithMaxSizeDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithMockModuleAllow(inner) => {
                    <testIsAllowedWithMockModuleAllowCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithMockModuleDeny(inner) => {
                    <testIsAllowedWithMockModuleDenyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithOversizedData(inner) => {
                    <testIsAllowedWithOversizedDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithRealModule(inner) => {
                    <testIsAllowedWithRealModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedWithZeroAddress(inner) => {
                    <testIsAllowedWithZeroAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testMaxDataSizeConstant(inner) => {
                    <testMaxDataSizeConstantCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testOnlyOwnerFunctions(inner) => {
                    <testOnlyOwnerFunctionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testOwnershipTransfer(inner) => {
                    <testOwnershipTransferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testPermissionRequirementModuleGetter(inner) => {
                    <testPermissionRequirementModuleGetterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testRenounceOwnership(inner) => {
                    <testRenounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testStorageLocation(inner) => {
                    <testStorageLocationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateRequirementModule(inner) => {
                    <testUpdateRequirementModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateRequirementModuleNonAdmin(inner) => {
                    <testUpdateRequirementModuleNonAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateRequirementModuleToAddressOne(inner) => {
                    <testUpdateRequirementModuleToAddressOneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateRequirementModuleToZeroAddress(inner) => {
                    <testUpdateRequirementModuleToZeroAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testWithRequireAndModule(inner) => {
                    <testWithRequireAndModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testWithRequireOrModule(inner) => {
                    <testWithRequireOrModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::user1(inner) => {
                    <user1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::user2(inner) => {
                    <user2Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SequencingModuleCheckerTest`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SequencingModuleCheckerTestEvents {
        #[allow(missing_docs)]
        RequirementModuleUpdated(RequirementModuleUpdated),
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
    impl SequencingModuleCheckerTestEvents {
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
                37u8, 53u8, 128u8, 248u8, 6u8, 116u8, 28u8, 17u8, 179u8, 212u8, 170u8,
                96u8, 217u8, 202u8, 204u8, 91u8, 239u8, 12u8, 235u8, 179u8, 87u8, 72u8,
                118u8, 127u8, 226u8, 63u8, 17u8, 145u8, 110u8, 47u8, 4u8, 185u8,
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
    impl alloy_sol_types::SolEventInterface for SequencingModuleCheckerTestEvents {
        const NAME: &'static str = "SequencingModuleCheckerTestEvents";
        const COUNT: usize = 23usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <RequirementModuleUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RequirementModuleUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RequirementModuleUpdated)
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
    impl alloy_sol_types::private::IntoLogData for SequencingModuleCheckerTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::RequirementModuleUpdated(inner) => {
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
                Self::RequirementModuleUpdated(inner) => {
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
    /**Creates a new wrapper around an on-chain [`SequencingModuleCheckerTest`](self) contract instance.

See the [wrapper's documentation](`SequencingModuleCheckerTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SequencingModuleCheckerTestInstance<P, N> {
        SequencingModuleCheckerTestInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<SequencingModuleCheckerTestInstance<P, N>>,
    > {
        SequencingModuleCheckerTestInstance::<P, N>::deploy(provider)
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
        SequencingModuleCheckerTestInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`SequencingModuleCheckerTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SequencingModuleCheckerTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SequencingModuleCheckerTestInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SequencingModuleCheckerTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SequencingModuleCheckerTestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SequencingModuleCheckerTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SequencingModuleCheckerTest`](self) contract instance.

See the [wrapper's documentation](`SequencingModuleCheckerTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SequencingModuleCheckerTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> SequencingModuleCheckerTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SequencingModuleCheckerTestInstance<P, N> {
            SequencingModuleCheckerTestInstance {
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
    > SequencingModuleCheckerTestInstance<P, N> {
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
        ///Creates a new call builder for the [`masterModule`] function.
        pub fn masterModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, masterModuleCall, N> {
            self.call_builder(&masterModuleCall)
        }
        ///Creates a new call builder for the [`mockModule`] function.
        pub fn mockModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, mockModuleCall, N> {
            self.call_builder(&mockModuleCall)
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
        ///Creates a new call builder for the [`testCannotInitializeTwice`] function.
        pub fn testCannotInitializeTwice(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testCannotInitializeTwiceCall, N> {
            self.call_builder(&testCannotInitializeTwiceCall)
        }
        ///Creates a new call builder for the [`testFuzzDataSizeLimit`] function.
        pub fn testFuzzDataSizeLimit(
            &self,
            dataSize: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, testFuzzDataSizeLimitCall, N> {
            self.call_builder(
                &testFuzzDataSizeLimitCall {
                    dataSize,
                },
            )
        }
        ///Creates a new call builder for the [`testFuzzDataSizeRevert`] function.
        pub fn testFuzzDataSizeRevert(
            &self,
            dataSize: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, testFuzzDataSizeRevertCall, N> {
            self.call_builder(
                &testFuzzDataSizeRevertCall {
                    dataSize,
                },
            )
        }
        ///Creates a new call builder for the [`testFuzzIsAllowedWithVariousAddresses`] function.
        pub fn testFuzzIsAllowedWithVariousAddresses(
            &self,
            proposer: alloy::sol_types::private::Address,
            originator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testFuzzIsAllowedWithVariousAddressesCall,
            N,
        > {
            self.call_builder(
                &testFuzzIsAllowedWithVariousAddressesCall {
                    proposer,
                    originator,
                },
            )
        }
        ///Creates a new call builder for the [`testInitialization`] function.
        pub fn testInitialization(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testInitializationCall, N> {
            self.call_builder(&testInitializationCall)
        }
        ///Creates a new call builder for the [`testInitializeWithDifferentModule`] function.
        pub fn testInitializeWithDifferentModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testInitializeWithDifferentModuleCall,
            N,
        > {
            self.call_builder(&testInitializeWithDifferentModuleCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithAddressOne`] function.
        pub fn testIsAllowedWithAddressOne(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedWithAddressOneCall, N> {
            self.call_builder(&testIsAllowedWithAddressOneCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithEmptyData`] function.
        pub fn testIsAllowedWithEmptyData(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedWithEmptyDataCall, N> {
            self.call_builder(&testIsAllowedWithEmptyDataCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithLargeData`] function.
        pub fn testIsAllowedWithLargeData(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedWithLargeDataCall, N> {
            self.call_builder(&testIsAllowedWithLargeDataCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithMaxSizeData`] function.
        pub fn testIsAllowedWithMaxSizeData(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedWithMaxSizeDataCall, N> {
            self.call_builder(&testIsAllowedWithMaxSizeDataCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithMockModuleAllow`] function.
        pub fn testIsAllowedWithMockModuleAllow(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testIsAllowedWithMockModuleAllowCall,
            N,
        > {
            self.call_builder(&testIsAllowedWithMockModuleAllowCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithMockModuleDeny`] function.
        pub fn testIsAllowedWithMockModuleDeny(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedWithMockModuleDenyCall, N> {
            self.call_builder(&testIsAllowedWithMockModuleDenyCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithOversizedData`] function.
        pub fn testIsAllowedWithOversizedData(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedWithOversizedDataCall, N> {
            self.call_builder(&testIsAllowedWithOversizedDataCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithRealModule`] function.
        pub fn testIsAllowedWithRealModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedWithRealModuleCall, N> {
            self.call_builder(&testIsAllowedWithRealModuleCall)
        }
        ///Creates a new call builder for the [`testIsAllowedWithZeroAddress`] function.
        pub fn testIsAllowedWithZeroAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedWithZeroAddressCall, N> {
            self.call_builder(&testIsAllowedWithZeroAddressCall)
        }
        ///Creates a new call builder for the [`testMaxDataSizeConstant`] function.
        pub fn testMaxDataSizeConstant(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testMaxDataSizeConstantCall, N> {
            self.call_builder(&testMaxDataSizeConstantCall)
        }
        ///Creates a new call builder for the [`testOnlyOwnerFunctions`] function.
        pub fn testOnlyOwnerFunctions(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testOnlyOwnerFunctionsCall, N> {
            self.call_builder(&testOnlyOwnerFunctionsCall)
        }
        ///Creates a new call builder for the [`testOwnershipTransfer`] function.
        pub fn testOwnershipTransfer(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testOwnershipTransferCall, N> {
            self.call_builder(&testOwnershipTransferCall)
        }
        ///Creates a new call builder for the [`testPermissionRequirementModuleGetter`] function.
        pub fn testPermissionRequirementModuleGetter(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testPermissionRequirementModuleGetterCall,
            N,
        > {
            self.call_builder(&testPermissionRequirementModuleGetterCall)
        }
        ///Creates a new call builder for the [`testRenounceOwnership`] function.
        pub fn testRenounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testRenounceOwnershipCall, N> {
            self.call_builder(&testRenounceOwnershipCall)
        }
        ///Creates a new call builder for the [`testStorageLocation`] function.
        pub fn testStorageLocation(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testStorageLocationCall, N> {
            self.call_builder(&testStorageLocationCall)
        }
        ///Creates a new call builder for the [`testUpdateRequirementModule`] function.
        pub fn testUpdateRequirementModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testUpdateRequirementModuleCall, N> {
            self.call_builder(&testUpdateRequirementModuleCall)
        }
        ///Creates a new call builder for the [`testUpdateRequirementModuleNonAdmin`] function.
        pub fn testUpdateRequirementModuleNonAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testUpdateRequirementModuleNonAdminCall,
            N,
        > {
            self.call_builder(&testUpdateRequirementModuleNonAdminCall)
        }
        ///Creates a new call builder for the [`testUpdateRequirementModuleToAddressOne`] function.
        pub fn testUpdateRequirementModuleToAddressOne(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testUpdateRequirementModuleToAddressOneCall,
            N,
        > {
            self.call_builder(&testUpdateRequirementModuleToAddressOneCall)
        }
        ///Creates a new call builder for the [`testUpdateRequirementModuleToZeroAddress`] function.
        pub fn testUpdateRequirementModuleToZeroAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testUpdateRequirementModuleToZeroAddressCall,
            N,
        > {
            self.call_builder(&testUpdateRequirementModuleToZeroAddressCall)
        }
        ///Creates a new call builder for the [`testWithRequireAndModule`] function.
        pub fn testWithRequireAndModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testWithRequireAndModuleCall, N> {
            self.call_builder(&testWithRequireAndModuleCall)
        }
        ///Creates a new call builder for the [`testWithRequireOrModule`] function.
        pub fn testWithRequireOrModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testWithRequireOrModuleCall, N> {
            self.call_builder(&testWithRequireOrModuleCall)
        }
        ///Creates a new call builder for the [`user1`] function.
        pub fn user1(&self) -> alloy_contract::SolCallBuilder<&P, user1Call, N> {
            self.call_builder(&user1Call)
        }
        ///Creates a new call builder for the [`user2`] function.
        pub fn user2(&self) -> alloy_contract::SolCallBuilder<&P, user2Call, N> {
            self.call_builder(&user2Call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SequencingModuleCheckerTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`RequirementModuleUpdated`] event.
        pub fn RequirementModuleUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, RequirementModuleUpdated, N> {
            self.event_filter::<RequirementModuleUpdated>()
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
