use darling::util::Ignored;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::{extract_optional_type, string_to_str_type};

#[derive(FromDeriveInput)]
#[darling(attributes(getter), supports(struct_named))]
pub(crate) struct Getter {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    data: darling::ast::Data<Ignored, GetterField>,
}

impl Getter {
    pub(crate) fn get_getters(&self) -> Vec<TokenStream> {
        match &self.data {
            darling::ast::Data::Struct(fields) => fields
                .iter()
                .filter(|f| !f.skip)
                .map(|f| {
                    let GetterField { ident, ty, .. } = f;
                    if let Some(optional_ty) = extract_optional_type(ty) {
                        let (ty, method) = string_to_str_type(&optional_ty);

                        quote! {
                            pub fn #ident(&self) -> Option<&#ty> {
                                self.#ident.as_ref()#method
                            }
                        }
                    } else {
                        let (ty, _) = string_to_str_type(ty);

                        quote! {
                            pub fn #ident(&self) -> &#ty {
                                &self.#ident
                            }
                        }
                    }
                })
                .collect(),
            _ => panic!(),
        }
    }
}

#[derive(FromField)]
#[darling(attributes(getter))]
pub(crate) struct GetterField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    skip: bool,
}
