use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::{quote, format_ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let builder_ident = format_ident!("{}Builder", ident);

    let data = input.data;
    match data {
        Data::Struct(data) => {
            let fields = data.fields;
            match fields {
                Fields::Named(named) => {

                },
                _ => todo!(),
            }
        },
        _ => todo!(),
    }

    let tokens = quote! {
        pub struct #builder_ident {

        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {}
            }
        }
    };

    TokenStream::from(tokens)
}
