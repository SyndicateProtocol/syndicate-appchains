use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Error, Fields};

/// Derives the `IntoIterator` trait for a struct, converting all fields into string key-value pairs.
///
/// This macro automatically implements `IntoIterator` for your struct where:
/// - Keys are the field names as strings
/// - Values are the field values converted to strings via `to_string()`
///
/// # Example
/// ```
/// use block_builder_macro::IntoStringKVPairs;
///
/// #[derive(IntoStringKVPairs)]
/// struct Config {
///     port: u16,
///     name: String,
/// }
///
/// let config = Config { port: 8080, name: "server".to_string() };
/// let pairs: Vec<(String, String)> = config.into_iter().collect();
/// // Results in: [("port", "8080"), ("name", "server")]
/// ```
#[proc_macro_derive(IntoStringKVPairs)]
pub fn into_string_kv_pairs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match impl_into_string_kv_pairs(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn impl_into_string_kv_pairs(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;

    // Get fields from the struct
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(Error::new_spanned(
                    input,
                    "IntoKVPairs can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(Error::new_spanned(
                input,
                "IntoKVPairs can only be derived for structs",
            ))
        }
    };

    // Verify there's at least one field
    if fields.is_empty() {
        return Err(Error::new_spanned(
            input,
            "Cannot derive IntoKVPairs for struct with no fields",
        ));
    }

    let field_names = fields.iter().map(|f| &f.ident);
    let field_names_str = fields.iter().map(|f| {
        f.ident
            .as_ref()
            .expect("Field must have an identifier")
            .to_string()
    });

    Ok(quote! {
        impl IntoIterator for #struct_name {
            type Item = (String, String);
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                vec![
                    #(
                        (#field_names_str.to_string(),
                         self.#field_names.to_string()),
                    )*
                ].into_iter()
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to strip whitespace and normalize output
    fn normalize_tokens(tokens: proc_macro2::TokenStream) -> String {
        tokens.to_string().replace([' ', '\n'], "")
    }

    #[test]
    fn test_derive_for_valid_struct() {
        let input = quote! {
            struct Test {
                field1: u32,
                field2: String,
            }
        };

        let input: DeriveInput = syn::parse2(input).unwrap();
        assert!(impl_into_string_kv_pairs(&input).is_ok());
    }

    #[test]
    fn test_derive_fails_for_empty_struct() {
        let input = quote! {
            struct Test {}
        };

        let input: DeriveInput = syn::parse2(input).unwrap();
        assert!(impl_into_string_kv_pairs(&input).is_err());
    }

    #[test]
    fn test_exact_iterator_output() {
        // Define test struct with the same values as Configuration::default()
        let input = quote! {
            struct Configuration {
                port: u16,
                genesis_timestamp: u64,
                chain_id: u64,
            }
        };

        let input: DeriveInput = syn::parse2(input).unwrap();
        let output = impl_into_string_kv_pairs(&input).unwrap();

        // Create the expected implementation
        let expected = quote! {
            impl IntoIterator for Configuration {
                type Item = (String, String);
                type IntoIter = std::vec::IntoIter<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    vec![
                        ("port".to_string(), self.port.to_string()),
                        ("genesis_timestamp".to_string(), self.genesis_timestamp.to_string()),
                        ("chain_id".to_string(), self.chain_id.to_string()),
                    ].into_iter()
                }
            }
        };

        // Compare normalized token streams
        assert_eq!(normalize_tokens(expected), normalize_tokens(output));
    }
}
