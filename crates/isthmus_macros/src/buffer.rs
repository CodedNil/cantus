use crate::isthmus_path;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_quote};

pub fn derive(input: &DeriveInput) -> proc_macro2::TokenStream {
    if !input
        .attrs
        .iter()
        .any(|attribute| matches!(&attribute.meta, syn::Meta::List(list) if attribute.path().is_ident("repr") && list.tokens.to_string() == "C"))
    {
        return syn::Error::new_spanned(input, "ShaderData requires #[repr(C)]").to_compile_error();
    }
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
        #[cfg(not(target_arch = "spirv"))]
        unsafe impl #impl_generics #isthmus::__private::bytemuck::Zeroable for #name #type_generics #where_clause {}
        #[cfg(not(target_arch = "spirv"))]
        unsafe impl #impl_generics #isthmus::__private::bytemuck::Pod for #name #type_generics #where_clause {}
        unsafe impl #impl_generics #isthmus::ShaderData for #name #type_generics #where_clause {}
    }
}
