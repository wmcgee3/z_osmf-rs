#![forbid(unsafe_code)]

mod endpoint;
mod getter;
mod utils;

use darling::FromDeriveInput;
use getter::Getter;
use proc_macro::TokenStream;
use quote::quote;

use self::endpoint::Endpoint;

#[proc_macro_derive(Endpoint, attributes(endpoint))]
pub fn derive_endpoint(input: TokenStream) -> TokenStream {
    let input = &syn::parse_macro_input!(input as syn::DeriveInput);
    let endpoint = Endpoint::from_derive_input(input).unwrap();

    let Endpoint {
        ref ident,
        ref generics,
        ..
    } = &endpoint;

    let (impl_, ty, where_clause) = generics.split_for_impl();

    let new_fn = endpoint.new_fn();
    let setter_fns = endpoint.setter_fns();
    let get_response_fn = endpoint.get_response_fn();

    quote! {
        impl #impl_ #ident #ty #where_clause {
            #new_fn

            #( #setter_fns )*

            #get_response_fn

            pub async fn build(self) -> Result<T, crate::error::Error> {
                self.get_response().await?.try_into_target().await
            }
        }
    }
    .into()
}

#[proc_macro_derive(Getters, attributes(getter))]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let input = &syn::parse_macro_input!(input as syn::DeriveInput);
    let getter = Getter::from_derive_input(input).unwrap();

    let Getter {
        ident, generics, ..
    } = &getter;

    let (impl_, ty, where_clause) = generics.split_for_impl();

    let getters = getter.get_getters();

    quote! {
        impl #impl_ #ident #ty #where_clause {
            #( #getters )*
        }
    }
    .into()
}
