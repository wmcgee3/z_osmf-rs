use darling::util::Ignored;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::utils::{extract_optional_type, is_option, is_phantom_data};

impl From<Endpoint> for proc_macro::TokenStream {
    fn from(value: Endpoint) -> Self {
        let Endpoint {
            ref ident,
            ref generics,
            ..
        } = &value;

        let new_fn = value.new_fn();
        let get_response_fn = value.get_response_fn();

        let setter_fns = value
            .data
            .as_ref()
            .take_struct()
            .unwrap()
            .fields
            .iter()
            .map(|f| f.setter())
            .collect::<Vec<_>>();

        let (impl_, ty, where_clause) = generics.split_for_impl();

        quote! {
            impl #impl_ #ident #ty #where_clause {
                #new_fn

                #( #setter_fns )*

                #get_response_fn

                pub async fn build(self) -> Result<T, crate::error::Error> {
                    use crate::convert::TryIntoTarget;

                    self.get_response().await?.try_into_target().await
                }
            }
        }
        .into()
    }
}

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
    fn new_fn(&self) -> TokenStream {
        let (optional_fields, required_fields): (Vec<&EndpointField>, Vec<&EndpointField>) = self
            .data
            .as_ref()
            .take_struct()
            .unwrap()
            .iter()
            .partition(|f| is_option(&f.ty) || is_phantom_data(&f.ty));

        let (args, required_assignments): (Vec<_>, Vec<_>) = required_fields
            .iter()
            .map(|f| {
                let EndpointField { ident, ty, .. } = f;

                if ty.to_token_stream().to_string() == "Box < str >" {
                    (
                        quote! { #ident: impl std::fmt::Display },
                        quote! { #ident: #ident.to_string().into() },
                    )
                } else {
                    (
                        quote! { #ident: impl Into<#ty> },
                        quote! { #ident: #ident.into() },
                    )
                }
            })
            .unzip();

        let optional_assignments = optional_fields
            .iter()
            .map(|f| {
                let EndpointField { ident, .. } = &f;

                quote! { #ident: Default::default() }
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

    fn get_response_fn(&self) -> TokenStream {
        let Endpoint {
            data, method, path, ..
        } = &self;

        let fields = data.as_ref().take_struct().unwrap();

        let path_builders: Vec<_> = fields.iter().map(|f| f.path_builder()).collect();
        let request_builders: Vec<_> = fields.iter().map(|f| f.request_builder()).collect();

        quote! {
            fn get_request(&self) -> Result<reqwest::Request, crate::error::Error> {
                let path = {
                    #( #path_builders )*

                    format!(#path)
                };

                let mut request_builder = self.core
                    .client
                    .#method(format!("{}{}", self.core.base_url, path));

                #( #request_builders )*

                let read = self.core.token.read().map_err(|err| crate::Error::Custom(err.to_string().into()))?;
                if let Some(ref token) = *read {
                    request_builder = request_builder.headers(token.into());
                }

                Ok(request_builder.build()?)
            }

            async fn get_response(&self) -> Result<reqwest::Response, crate::error::Error> {
                use crate::error::CheckStatus;

                let request = self.get_request()?;
                let response = self.core.client.execute(request).await?;

                response.check_status().await
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

impl EndpointField {
    fn path_builder(&self) -> Option<TokenStream> {
        match self {
            EndpointField {
                skip_builder: true, ..
            }
            | EndpointField { path: false, .. } => None,
            EndpointField {
                ident: Some(ident),
                builder_fn: Some(builder_fn),
                ..
            } => Some(quote! {
                let #ident = #builder_fn(self);
            }),
            EndpointField {
                ident: Some(ident), ..
            } => Some(quote! {
                let #ident = &self.#ident;
            }),
            _ => None,
        }
    }

    fn request_builder(&self) -> Option<TokenStream> {
        match self {
            EndpointField {
                skip_builder: true, ..
            }
            | EndpointField { path: true, .. } => None,
            EndpointField {
                builder_fn: Some(builder_fn),
                ..
            } => Some(quote! {
                request_builder = #builder_fn(request_builder, self);
            }),
            EndpointField {
                header: Some(header),
                ident: Some(ident),
                ty,
                ..
            } if ty.to_token_stream().to_string() == "Option < Box < str > >" => Some(quote! {
                if let Some(value) = &self.#ident {
                    request_builder = request_builder.header(#header, value.as_ref());
                }
            }),
            EndpointField {
                header: Some(header),
                ident: Some(ident),
                ty,
                ..
            } if is_option(ty) => Some(quote! {
                if let Some(value) = &self.#ident {
                    request_builder = request_builder.header(#header, *value);
                }
            }),
            EndpointField {
                header: Some(header),
                ident: Some(ident),
                ..
            } => Some(quote! {
                request_builder = request_builder.header(#header, &self.#ident);
            }),
            EndpointField {
                query: Some(query),
                ident: Some(ident),
                ty,
                ..
            } if is_option(ty) => Some(quote! {
                if let Some(value) = &self.#ident {
                    request_builder = request_builder.query(&[(#query, &value)]);
                }
            }),
            EndpointField {
                query: Some(query),
                ident: Some(ident),
                ..
            } => Some(quote! {
                request_builder = request_builder.query(&[(#query, &self.#ident)]);
            }),
            _ => None,
        }
    }

    fn setter(&self) -> Option<TokenStream> {
        match self {
            EndpointField {
                ty, skip_setter, ..
            } if !is_option(ty) | skip_setter => None,
            EndpointField {
                setter_fn,
                ident: Some(ident),
                ty,
                ..
            } if ty.to_token_stream().to_string() == "Option < Box < str > >" => {
                let body = if let Some(setter_fn) = setter_fn {
                    quote! {
                        #setter_fn(self, value)
                    }
                } else {
                    quote! {
                        let mut new = self;
                        new.#ident = Some(value.to_string().into());

                        new
                    }
                };

                Some(quote! {
                    pub fn #ident<V>(self, value: V) -> Self
                    where
                        V: std::fmt::Display,
                    {
                        #body
                    }
                })
            }
            EndpointField {
                setter_fn,
                ident: Some(ident),
                ty,
                ..
            } => {
                let ty = extract_optional_type(ty).unwrap();

                let body = if let Some(setter_fn) = setter_fn {
                    quote! {
                        #setter_fn(self, Some(value.into()))
                    }
                } else {
                    quote! {
                        let mut new = self;
                        new.#ident = Some(value.into());

                        new
                    }
                };

                Some(quote! {
                    pub fn #ident<V>(self, value: V) -> Self
                    where
                        V: Into<#ty>,
                    {
                        #body
                    }
                })
            }
            _ => None,
        }
    }
}
