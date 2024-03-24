#![forbid(unsafe_code)]

mod endpoint;
mod getter;
mod utils;

use darling::FromDeriveInput;
use proc_macro::TokenStream;

use self::endpoint::Endpoint;
use self::getter::Getter;

#[proc_macro_derive(Endpoint, attributes(endpoint))]
pub fn derive_endpoint(input: TokenStream) -> TokenStream {
    let input = &syn::parse_macro_input!(input as syn::DeriveInput);
    let endpoint = Endpoint::from_derive_input(input).unwrap();

    endpoint.into()
}

#[proc_macro_derive(Getters, attributes(getter))]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let input = &syn::parse_macro_input!(input as syn::DeriveInput);
    let getter = Getter::from_derive_input(input).unwrap();

    getter.into()
}
