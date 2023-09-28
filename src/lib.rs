// Copyright 2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/sneks/blob/stylus/licenses/COPYRIGHT.md

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum};

#[proc_macro_derive(SimpleSnakeNames)]
pub fn derive_simple_snake_names(input: TokenStream) -> TokenStream {
    let ItemEnum {
        ident,
        generics,
        variants,
        ..
    } = parse_macro_input!(input as ItemEnum);

    let mut arms = quote!();
    for variant in variants {
        let name = variant.ident;
        let snek = name.to_string().to_case(Case::Snake);
        arms.extend(match variant.fields {
            Fields::Unit => quote! { #name => #snek, },
            Fields::Unnamed(..) => quote! { #name(..) => #snek, },
            Fields::Named(..) => quote! { #name { .. } => #snek, },
        });
    }

    quote! {
        impl #generics #ident #generics {
            fn name(&self) -> &'static str {
                use #ident::*;
                match self {
                    #arms
                }
            }
        }
    }
    .into()
}
