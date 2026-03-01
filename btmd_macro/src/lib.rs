use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro_derive(ArgLookup, attributes(arg_def))]
pub fn derive_arg_lookup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("ArgLookup can only be used on Enums");
    };

    let mut match_arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        let construct_variant = match &variant.fields {
            syn::Fields::Unit => quote! { #name::#variant_ident },
            syn::Fields::Unnamed(fields) => {
                let defaults = vec![quote! { Default::default() }; fields.unnamed.len()];
                quote! { #name::#variant_ident(#(#defaults),*) }
            }
            syn::Fields::Named(fields) => {
                let fields = fields.named.iter().map(|f| {
                    let field_ident = &f.ident;
                    quote! { #field_ident: Default::default() }
                });
                quote! { #name::#variant_ident { #(#fields),* } }
            }
        };

        for attr in &variant.attrs {
            if attr.path().is_ident("arg_def") {
                let arg_name: LitStr = attr.parse_args().expect("Expected string in arg_def");

                match_arms.push(quote! {
                    #arg_name => #construct_variant,
                });
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn get_arg(name: &str) -> #name {
                match name {
                    #(#match_arms)*
                    _ => panic!("Invalid argument name: '{}' for enum {}", name, stringify!(#name)),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ConfigLookup, attributes(config_def))]
pub fn derive_config_lookup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("ConfigLookup can only be used on Enums");
    };

    let mut match_arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        let construct_variant = match &variant.fields {
            syn::Fields::Unit => quote! { #name::#variant_ident },
            syn::Fields::Unnamed(fields) => {
                let defaults = vec![quote! { Default::default() }; fields.unnamed.len()];
                quote! { #name::#variant_ident(#(#defaults),*) }
            }
            syn::Fields::Named(fields) => {
                let fields = fields.named.iter().map(|f| {
                    let field_ident = &f.ident;
                    quote! { #field_ident: Default::default() }
                });
                quote! { #name::#variant_ident { #(#fields),* } }
            }
        };

        for attr in &variant.attrs {
            if attr.path().is_ident("config_def") {
                let arg_name: LitStr = attr.parse_args().expect("Expected string in config_def");

                match_arms.push(quote! {
                    #arg_name => #construct_variant,
                });
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn get_type(name: &str) -> #name {
                match name {
                    #(#match_arms)*
                    _ => panic!("Invalid config name: '{}' for enum {}", name, stringify!(#name)),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn unwrap_val(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as UnwrapArgInput);
    let value = input.value;
    let variant = input.variant;

    let expanded = quote! {
        match #value {
            ValueTypes::#variant(v) => v.to_owned(),
            _ => Default::default(),
        }
    };
    expanded.into()
}

struct UnwrapArgInput {
    value: syn::Expr,
    variant: syn::Ident,
}

impl Parse for UnwrapArgInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse()?;
        input.parse::<Token![,]>()?;
        let variant = if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            syn::Ident::new(&lit.value(), lit.span())
        } else {
            input.parse()?
        };
        Ok(UnwrapArgInput { value, variant })
    }
}