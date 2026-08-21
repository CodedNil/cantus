use isthmus_build::artifact::shader_artifact;
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::{env, path::Path};

mod abi;
mod buffer;
mod model;
mod pass;
mod render;
mod syntax;
mod varyings;

fn isthmus_path() -> TokenStream2 {
    match crate_name("isthmus") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let name = format_ident!("{name}");
            quote!(::#name)
        }
        Err(_) => quote!(::isthmus),
    }
}

/// Declares host/GPU data and rejects layouts that Rust-GPU cannot expose to WGSL.
#[proc_macro_attribute]
pub fn data(_args: TokenStream, input: TokenStream) -> TokenStream {
    buffer::attribute(input)
}

#[proc_macro_derive(Varyings, attributes(gpu))]
pub fn derive_varyings(input: TokenStream) -> TokenStream {
    varyings::derive(input)
}

#[proc_macro_derive(Render, attributes(render))]
pub fn derive_render(input: TokenStream) -> TokenStream {
    render::derive(input)
}

/// Declares a typed CPU/GPU render pass.
#[proc_macro_attribute]
pub fn pass(args: TokenStream, input: TokenStream) -> TokenStream {
    pass::expand(args, input)
}

/// Keeps a shader helper outlined when its signature is legal in SPIR-V.
#[proc_macro_attribute]
pub fn outline(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return syn::Error::new(proc_macro2::Span::call_site(), "outline takes no arguments").to_compile_error().into();
    }
    let function = syn::parse_macro_input!(input as syn::ItemFn);
    quote!(#[cfg_attr(target_arch = "spirv", inline(never))] #function).into()
}

/// Internal second half of the pass expansion.
#[proc_macro]
#[doc(hidden)]
pub fn lower_pass(input: TokenStream) -> TokenStream {
    pass::lower(input)
}

/// Includes the package's checked-in shader artifact.
#[proc_macro]
pub fn shader_module(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        return syn::Error::new(proc_macro2::Span::call_site(), "shader_module takes no arguments")
            .to_compile_error()
            .into();
    }
    let Ok(crate_dir) = env::var("CARGO_MANIFEST_DIR") else {
        return syn::Error::new(proc_macro2::Span::call_site(), "missing package directory").to_compile_error().into();
    };
    let artifact = match shader_artifact(Path::new(&crate_dir)) {
        Ok(artifact) => artifact,
        Err(error) => {
            return syn::Error::new(proc_macro2::Span::call_site(), error).to_compile_error().into();
        }
    };
    let artifact = artifact.to_string_lossy();
    let isthmus = isthmus_path();
    quote!(#isthmus::wgpu::include_wgsl!(#artifact)).into()
}
