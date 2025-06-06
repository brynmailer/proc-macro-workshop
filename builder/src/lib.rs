use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn ty_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }
        }
    }

    None
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        unimplemented!();
    };

    let optionized_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if ty_inner_type(ty).is_some() {
            quote! { #name: #ty }
        } else {
            quote! { #name: std::option::Option<#ty> }
        }
    });

    let builder_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if let Some(inner_ty) = ty_inner_type(ty) {
            quote! {
                pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;

        if ty_inner_type(&f.ty).is_some() {
            quote! { #name: self.#name.clone() }
        } else {
            quote! { #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))? }
        }
    });

    let empty_fields = fields.iter().map(|f| {
        let name = &f.ident;

        quote! { #name: None }
    });

    let expanded = quote! {
        pub struct #builder_ident {
            #(#optionized_fields,)*
        }

        impl #builder_ident {
            #(#builder_methods)*

            pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#builder_fields,)*
                })
            }
        }

        impl #name {
            fn builder() -> #builder_ident {
                #builder_ident {
                    #(#empty_fields,)*
                }
            }
        }
    };

    expanded.into()
}
