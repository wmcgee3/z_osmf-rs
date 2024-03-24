use quote::{format_ident, ToTokens};

pub(crate) fn extract_optional_type(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            if is_option(ty) {
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

pub(crate) fn is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            return path_segment.ident == format_ident!("{}", "Option");
        }
    }

    false
}

pub(crate) fn is_phantom_data(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            return path_segment.ident == format_ident!("{}", "PhantomData");
        }
    }

    false
}
