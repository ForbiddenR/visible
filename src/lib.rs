use proc_macro::TokenStream;
use quote::quote;
use syn::{Data::Struct, DataStruct, DeriveInput, Fields, FieldsNamed, parse_macro_input};

#[proc_macro_derive(Data)]
pub fn visible(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {pub #name: #ty}
    });

    let visible_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    visible_version.into()
}
