use quote::{format_ident, quote, ToTokens};

pub(crate) fn extract_box_type(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            if path_segment.ident == format_ident!("{}", "Box") {
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

pub(crate) fn extract_optional_type(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            if path_segment.ident == format_ident!("{}", "Option") {
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

pub(crate) fn vec_to_slice_type(ty: syn::Type) -> syn::Type {
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

    ty
}
