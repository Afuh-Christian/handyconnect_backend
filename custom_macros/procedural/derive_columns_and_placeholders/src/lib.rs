use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use models::column_place_holder_trait::ColumnsAndPlaceholdersTrait;



#[proc_macro_derive(ColumnsAndPlaceholders)]
pub fn derive_columns_and_placeholders(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        syn::Data::Struct(s) => s.fields,
        _ => panic!("Only works with structs"),
    };

    let col_names: Vec<String> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    let col_idents: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();

    let placeholders: Vec<String> =
        (1..=col_names.len()).map(|i| format!("${}", i)).collect();

    let expanded = quote! {
        impl ColumnsAndPlaceholdersTrait for #name {
            pub fn column_names() -> Vec<&'static str> {
                vec![ #( #col_names ),* ]
            }
            pub fn placeholders() -> Vec<&'static str> {
                vec![ #( #placeholders ),* ]
            }
            pub fn values(&self) -> Vec<String> {
                vec![ #( self.#col_idents.to_string() ),* ]
            }
        }
    };
    expanded.into()
}
