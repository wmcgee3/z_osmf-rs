use darling::util::Ignored;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_optional_type;

#[derive(FromDeriveInput)]
#[darling(attributes(endpoint), supports(struct_named))]
pub(crate) struct Endpoint {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    data: darling::ast::Data<Ignored, EndpointField>,

    method: syn::Ident,
    path: String,
}

impl Endpoint {
    pub(crate) fn new_fn(&self) -> TokenStream {
        let (optional_fields, required_fields): (Vec<&EndpointField>, Vec<&EndpointField>) = self
            .data
            .as_ref()
            .take_struct()
            .unwrap()
            .iter()
            .partition(|f| f.optional);

        let (args, required_assignments): (Vec<_>, Vec<_>) = required_fields
            .iter()
            .map(|f| {
                let EndpointField { ident, ty, .. } = f;

                (
                    quote! { #ident: impl Into<#ty> },
                    quote! { #ident: #ident.into() },
                )
            })
            .unzip();

        let optional_assignments = optional_fields
            .iter()
            .map(|f| {
                let EndpointField { ident, ty, .. } = &f;

                quote! { #ident: <#ty>::default() }
            })
            .collect::<Vec<_>>();

        quote! {
            pub(crate) fn new( #( #args, )* ) -> Self {
                Self {
                    #( #required_assignments, )*
                    #( #optional_assignments, )*
                }
            }
        }
    }

    pub(crate) fn setter_fns(&self) -> Vec<TokenStream> {
        self.data
            .as_ref()
            .take_struct()
            .unwrap()
            .fields
            .iter()
            .filter(|f| f.optional && !f.skip_setter)
            .map(|f| {
                let EndpointField {
                    ident,
                    ty,
                    setter_fn,
                    ..
                } = f;

                if let Some(setter_fn) = setter_fn {
                    quote! {
                        pub fn #ident(mut self, value: impl Into<#ty>) -> Self {
                            #setter_fn(self, value.into())
                        }
                    }
                } else if let Some(optional_ty) = extract_optional_type(ty) {
                    quote! {
                        pub fn #ident(mut self, value: impl Into<#optional_ty>) -> Self {
                            self.#ident = Some(value.into());

                            self
                        }
                    }
                } else {
                    quote! {
                        pub fn #ident(mut self, value: impl Into<#ty>) -> Self {
                            self.#ident = value.into();

                            self
                        }
                    }
                }
            })
            .collect()
    }

    pub(crate) fn get_response_fn(&self) -> TokenStream {
        let Endpoint {
            data, method, path, ..
        } = &self;

        let fields = data.as_ref().take_struct().unwrap();

        let path_idents = fields
            .iter()
            .filter(|f| f.path)
            .map(|f| &f.ident)
            .collect::<Vec<_>>();

        let (optional_fields, required_fields): (Vec<&EndpointField>, Vec<&EndpointField>) =
            fields.iter().partition(|f| f.optional);

        let optional_builders = optional_fields
            .iter()
            .map(|f| {
                let EndpointField {
                    ident,
                    query,
                    header,
                    builder_fn,
                    ..
                } = f;

                if let Some(builder_fn) = builder_fn {
                    quote! {
                        request_builder = #builder_fn(request_builder, &self);
                    }
                } else if let Some(header) = header {
                    quote! {
                        if let Some(value) = &self.#ident {
                            request_builder = request_builder.header(#header, value.clone());
                        }
                    }
                } else if let Some(query) = query {
                    quote! {
                        if let Some(value) = &self.#ident {
                            request_builder = request_builder.query(&[(#query, &value)]);
                        }
                    }
                } else {
                    quote! {}
                }
            })
            .collect::<Vec<_>>();

        let required_builders = required_fields
            .iter()
            .map(|f| {
                let EndpointField {
                    ident,
                    query,
                    header,
                    ..
                } = f;

                if query.is_some() {
                    quote! {
                        .query(&[(#query, &self.#ident)])
                    }
                } else if header.is_some() {
                    quote! {
                        .header(#header, &self.#ident)
                    }
                } else {
                    quote! {}
                }
            })
            .collect::<Vec<_>>();

        quote! {
            async fn get_response(&self) -> anyhow::Result<reqwest::Response> {
                let path = {
                    let Self {
                        #( #path_idents, )*
                        ..
                    } = &self;

                    format!(#path)
                };

                let mut request_builder = self.client.#method(format!("{}{}", self.base_url, path))
                #( #required_builders )* ;

                #( #optional_builders )*

                Ok(request_builder.send().await?.error_for_status()?)
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromField)]
#[darling(attributes(endpoint))]
struct EndpointField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    optional: bool,
    query: Option<String>,
    #[darling(default)]
    path: bool,
    header: Option<String>,
    #[darling(default)]
    skip_setter: bool,
    setter_fn: Option<syn::ExprPath>,
    #[darling(default)]
    skip_builder: bool,
    builder_fn: Option<syn::ExprPath>,
}
