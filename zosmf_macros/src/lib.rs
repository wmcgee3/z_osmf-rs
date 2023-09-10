#![forbid(unsafe_code)]

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

mod endpoint;

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

    let new_fn = endpoint.get_new_fn();
    let setter_fns = endpoint.get_setter_fns();
    let build_fn = endpoint.get_request_builder_fn();

    quote! {
        impl #impl_ #ident #ty #where_clause {
            #new_fn

            #( #setter_fns )*

            #build_fn
        }
    }
    .into()
}
