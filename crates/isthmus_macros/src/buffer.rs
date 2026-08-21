use crate::isthmus_path;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, parse_quote};

pub fn attribute(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let isthmus = isthmus_path();
    input.attrs.push(parse_quote!(#[repr(C)]));
    input.attrs.push(parse_quote!(#[derive(Clone, Copy)]));
    input
        .attrs
        .push(parse_quote!(#[cfg_attr(not(target_arch = "spirv"), derive(#isthmus::__private::bytemuck::Pod, #isthmus::__private::bytemuck::Zeroable))]));
    let implementation = implementation(&input);
    quote!(#input #implementation).into()
}

fn implementation(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let isthmus = isthmus_path();
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(input, "ShaderData requires a struct").to_compile_error();
    };
    let Fields::Named(fields) = &data.fields else {
        return syn::Error::new_spanned(input, "ShaderData requires named fields").to_compile_error();
    };
    let types = fields.named.iter().map(|field| &field.ty).collect::<Vec<_>>();
    let mut generics = input.generics.clone();
    for ty in &types {
        generics.make_where_clause().predicates.push(parse_quote!(#ty: #isthmus::ShaderData));
    }
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #isthmus::ShaderData for #name #type_generics #where_clause {}
    }
}
