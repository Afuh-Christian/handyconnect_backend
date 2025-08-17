use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;



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

    // let col_idents: Vec<_> = fields
    //     .iter()
    //     .map(|f| f.ident.as_ref().unwrap())
    //     .collect();



let col_idents: Vec<_> = fields.iter().map(|f| {
    let ident = f.ident.as_ref().unwrap();
    match &f.ty {
        syn::Type::Path(type_path) if type_path.path.segments[0].ident == "Option" => {
            quote! { { match &self.#ident { Some(val) => val.to_string(), None => "NULL".to_string() } } }
        }
        _ => {
            quote! { self.#ident.to_string() }
        }
    }
}).collect();

    let placeholders: Vec<String> =
        (1..=col_names.len()).map(|i| format!("${}", i)).collect();

    let expanded = quote! {
      impl ColumnsAndPlaceholdersTrait for #name {
             fn column_names() -> Vec<&'static str> {
                vec![ #( #col_names ),* ]
            }
             fn placeholders() -> Vec<&'static str> {
                vec![ #( #placeholders ),* ]
            }
      fn values(&self) -> Vec<String> {
    vec![ #( #col_idents ),* ]
}
        }
    };
    expanded.into()
}
