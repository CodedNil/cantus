use crate::isthmus_path;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemStruct, parse_macro_input, parse_quote};

#[allow(clippy::missing_panics_doc)]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as ItemStruct);
    let Fields::Named(_) = &item.fields else {
        return syn::Error::new_spanned(item, "Varyings requires named fields")
            .to_compile_error()
            .into();
    };
    item.attrs.push(parse_quote!(#[gpu]));
    let bridge = format_ident!("__isthmus_varyings_{}", item.ident);
    let isthmus = isthmus_path();
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
