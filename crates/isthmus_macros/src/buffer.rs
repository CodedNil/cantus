use crate::isthmus_path;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, parse_quote};

#[allow(clippy::missing_panics_doc)]
pub fn attribute(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    input.attrs.push(parse_quote!(#[repr(C)]));
    input
        .attrs
        .push(parse_quote!(#[cfg_attr(target_arch = "spirv", repr(align(16)))]));
    input.attrs.push(parse_quote!(#[derive(Clone, Copy)]));
    let implementation = implementation(&input);
    quote!(#input #implementation).into()
}

fn implementation(input: &DeriveInput) -> TokenStream2 {
    let name = &input.ident;
    let isthmus = isthmus_path();
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(input, "BufferData requires a struct").to_compile_error();
    };
    let Fields::Named(fields) = &data.fields else {
        return syn::Error::new_spanned(input, "BufferData requires named fields").to_compile_error();
    };
    let names = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    let types = fields.named.iter().map(|field| &field.ty).collect::<Vec<_>>();
    let mut generics = input.generics.clone();
    for ty in &types {
        generics
            .make_where_clause()
            .predicates
            .push(parse_quote!(#ty: #isthmus::BufferData));
    }
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #isthmus::BufferData for #name #type_generics #where_clause {
            const BUFFER_ALIGN: usize = {
                let mut alignment = 1;
                #(
                    if <#types as #isthmus::BufferData>::BUFFER_ALIGN > alignment {
                        alignment = <#types as #isthmus::BufferData>::BUFFER_ALIGN;
                    }
                )*
                alignment
            };
            const BUFFER_SIZE: usize = {
                let mut offset = 0;
                #(
                    offset = #isthmus::__private::align_to(
                        offset,
                        <#types as #isthmus::BufferData>::BUFFER_ALIGN,
                    );
                    offset += <#types as #isthmus::BufferData>::BUFFER_SIZE;
                )*
                #isthmus::__private::align_to(offset, Self::BUFFER_ALIGN)
            };

            fn write_at(&self, words: &mut [u32], byte: usize) {
                let mut offset = 0;
                #(
                    offset = #isthmus::__private::align_to(
                        offset,
                        <#types as #isthmus::BufferData>::BUFFER_ALIGN,
                    );
                    <#types as #isthmus::BufferData>::write_at(
                        &self.#names,
                        words,
                        byte + offset,
                    );
                    offset += <#types as #isthmus::BufferData>::BUFFER_SIZE;
                )*
            }
        }
    }
}
