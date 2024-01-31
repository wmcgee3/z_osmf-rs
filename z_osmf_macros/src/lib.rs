#![forbid(unsafe_code)]

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

mod endpoint;
mod utils;

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
