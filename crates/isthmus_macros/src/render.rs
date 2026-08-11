use crate::isthmus_path;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Fields, Generics, Ident, parse_macro_input};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(input, "Render requires a struct")
            .to_compile_error()
            .into();
    };
    let Fields::Named(fields) = &data.fields else {
        return syn::Error::new_spanned(input, "Render requires named fields")
            .to_compile_error()
            .into();
    };
    let fields = fields.named.iter().filter_map(|field| {
        let skipped = field.attrs.iter().any(|attribute| {
            attribute.path().is_ident("render")
                && attribute.parse_args::<Ident>().is_ok_and(|value| value == "skip")
        });
        (!skipped).then(|| field.ident.as_ref().unwrap())
    });
    implementation(name, &input.generics, fields).into()
}

pub fn implementation(
    name: &Ident,
    generics: &Generics,
    fields: impl IntoIterator<Item = impl ToTokens>,
) -> TokenStream2 {
    let fields = fields.into_iter().collect::<Vec<_>>();
    let isthmus = isthmus_path();
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #isthmus::Render for #name #type_generics #where_clause {
            fn prepare(&mut self) {
                #(#isthmus::Render::prepare(&mut self.#fields);)*
            }

            fn draw<'pass>(
                &'pass self,
                pass: &mut #isthmus::wgpu::RenderPass<'pass>,
            ) {
                #(#isthmus::Render::draw(&self.#fields, pass);)*
            }
        }
    }
}
