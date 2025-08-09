use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FieldNames)]
pub fn field_names_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        syn::Data::Struct(data_struct) => {
            data_struct.fields.iter().filter_map(|f| f.ident.as_ref().map(|id| id.to_string())).collect::<Vec<_>>()
        }
        _ => {
            return syn::Error::new_spanned(name, "FieldNames can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let field_strs = fields.iter().map(|f| f.as_str());

    let expanded = quote! {
        impl FieldNames for #name {
            pub fn field_names() -> &'static [&'static str] {
                &[#(#field_strs),*]
            }
        }
    };

    TokenStream::from(expanded)
}


