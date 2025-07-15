use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed, ItemStruct, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn visible(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    match &mut input.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            for field in named.iter_mut() {
                field.vis = parse_quote!(pub);
            }
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            for field in unnamed.iter_mut() {
                field.vis = parse_quote!(pub);
            }
        }
        Fields::Unit => {}
    }

    TokenStream::from(quote! {#input})
}
