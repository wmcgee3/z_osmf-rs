use darling::util::Ignored;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::utils::{extract_optional_type, is_option};

impl From<Getter> for proc_macro::TokenStream {
    fn from(value: Getter) -> Self {
        let Getter {
            ident, generics, ..
        } = &value;

        let (impl_, ty, where_clause) = generics.split_for_impl();

        let getters = value
            .data
            .as_ref()
            .take_struct()
            .unwrap()
            .fields
            .iter()
            .map(|f| f.getter())
            .collect::<Vec<_>>();

        quote! {
            impl #impl_ #ident #ty #where_clause {
                #( #getters )*
            }
        }
        .into()
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(getter), supports(struct_named))]
pub(crate) struct Getter {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    data: darling::ast::Data<Ignored, GetterField>,
}

#[derive(FromField)]
#[darling(attributes(getter))]
pub(crate) struct GetterField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    skip: bool,
    #[darling(default)]
    copy: bool,
}

impl GetterField {
    fn getter(&self) -> Option<TokenStream> {
        match self {
            GetterField { skip: true, .. } => None,
            GetterField {
                copy: true,
                ident: Some(ident),
                ty,
                ..
            } => Some(quote! {
                pub fn #ident(&self) -> #ty {
                    self.#ident
                }
            }),
            GetterField {
                ident: Some(ident),
                ty,
                ..
            } if ty
                .to_token_stream()
                .to_string()
                .starts_with("Option < Box < ") =>
            {
                let ty = extract_box_type(&extract_optional_type(ty).unwrap()).unwrap();

                Some(quote! {
                    pub fn #ident(&self) -> Option<&#ty> {
                        self.#ident.as_deref()
                    }
                })
            }
            GetterField {
                ident: Some(ident),
                ty,
                ..
            } if is_option(ty) => {
                let ty = vec_to_slice_type(&extract_optional_type(ty).unwrap());

                Some(quote! {
                    pub fn #ident(&self) -> Option<&#ty> {
                        self.#ident.as_ref()
                    }
                })
            }
            GetterField {
                ident: Some(ident),
                ty,
                ..
            } if is_box(ty) => {
                let ty = extract_box_type(ty).unwrap();

                Some(quote! {
                    pub fn #ident(&self) -> &#ty {
                        &self.#ident
                    }
                })
            }
            GetterField {
                ident: Some(ident),
                ty,
                ..
            } => {
                let ty = vec_to_slice_type(ty);

                Some(quote! {
                    pub fn #ident(&self) -> &#ty {
                        &self.#ident
                    }
                })
            }
            _ => None,
        }
    }
}

fn extract_box_type(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            if is_box(ty) {
                if let syn::PathArguments::AngleBracketed(angle_bracketed) = &path_segment.arguments
                {
                    let tokens = angle_bracketed.args.first().unwrap().to_token_stream();
                    let new_ty = syn::parse::<syn::Type>(tokens.into()).unwrap();

                    return Some(new_ty);
                }
            }
        }
    }

    None
}

fn is_box(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            return path_segment.ident == format_ident!("{}", "Box");
        }
    }

    false
}

fn vec_to_slice_type(ty: &syn::Type) -> syn::Type {
    if let syn::Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            if path_segment.ident == format_ident!("{}", "Vec") {
                if let syn::PathArguments::AngleBracketed(angle_bracketed) = &path_segment.arguments
                {
                    let tokens = angle_bracketed.args.first().unwrap().to_token_stream();
                    let new_ty = syn::parse::<syn::Type>(quote! {[#tokens]}.into()).unwrap();

                    return new_ty;
                }
            }
        }
    }

    ty.clone()
}
