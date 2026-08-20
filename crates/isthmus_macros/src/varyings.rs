use crate::isthmus_path;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemStruct, Type, parse_macro_input, parse_quote};

pub fn derive(input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as ItemStruct);
    let Fields::Named(_) = &item.fields else {
        return syn::Error::new_spanned(item, "Varyings requires named fields")
            .to_compile_error()
            .into();
    };
    let isthmus = isthmus_path();
    if let Fields::Named(fields) = &mut item.fields {
        for field in &mut fields.named {
            let Type::Path(path) = &field.ty else {
                continue;
            };
            if path.qself.is_none()
                && path.path.segments.len() == 1
                && matches!(
                    path.path.segments[0].ident.to_string().as_str(),
                    "Vec2" | "Vec3" | "Vec4" | "UVec2" | "UVec3" | "UVec4"
                )
            {
                let name = &path.path.segments[0].ident;
                field.ty = parse_quote!(#isthmus::glam::#name);
            }
        }
    }
    item.attrs.push(parse_quote!(#[gpu]));
    let bridge = format_ident!("__isthmus_varyings_{}", item.ident);
    quote! {
        macro_rules! #bridge {
            ($implementation:item) => {
                #isthmus::lower_pass! {
                    #item
                    $implementation
                }
            };
        }
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub(crate) use #bridge;
    }
    .into()
}
