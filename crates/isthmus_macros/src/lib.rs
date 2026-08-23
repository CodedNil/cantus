use isthmus_build::artifact::{shader_artifact, shader_dependency, shader_is_fresh, shader_source};
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::{collections::HashSet, env, path::Path};
use syn::visit::{self, Visit};
use syn::visit_mut::{VisitMut, visit_expr_mut};

mod buffer;
mod paint;

fn fragment_entry(frame: &TokenStream2, text: bool, payload: bool, images: bool, body: &TokenStream2) -> TokenStream2 {
    let isthmus = isthmus_path();
    let text_resources = text.then(|| {
        quote! {
            #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] placed_glyphs: &[#isthmus::text::PlacedGlyph],
            #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] glyphs: &[#isthmus::text::Glyph],
            #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] edges: &[#isthmus::text::Edge],
        }
    });
    let payload_resource = payload.then(|| quote!(#[spirv(storage_buffer, descriptor_set = 0, binding = 1)] payload: &[u32],));
    let image_resource = images.then(|| quote!(#[spirv(descriptor_set = 0, binding = 5)] images: &#isthmus::__private::ImageHeap,));
    quote! {
        #[#isthmus::spirv_std::spirv(fragment)]
        pub fn fragment(
            #[spirv(location = 0)] pixel: #isthmus::glam::Vec2,
            #[spirv(location = 1, flat)] draw_index: u32,
            #[spirv(push_constant)] __isthmus_frame: &#frame,
            #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] draws: &[#isthmus::__private::DrawRecord],
            #payload_resource
            #text_resources
            #image_resource
            #[spirv(location = 0)] out_color: &mut #isthmus::glam::Vec4,
        ) {
            let _ = (&__isthmus_frame, draws);
            let draw = draws[draw_index as usize];
            let local = draw.quad.local(pixel);
            let fragment = #isthmus::Fragment::new(pixel, local, draw.quad.size);
            #body
        }
    }
}

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

#[proc_macro_derive(ShaderData)]
pub fn derive_shader_data(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    buffer::derive(&input).into()
}

#[proc_macro_derive(Program)]
pub fn derive_program(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    program_items(&input).unwrap_or_else(syn::Error::into_compile_error).into()
}

/// Extracts inline paint closures and keeps their surrounding implementation on the host.
///
/// Generated shaders receive Isthmus' cross-target float extension methods and lower
/// captured images to the renderer's internal descriptor heap.
#[proc_macro_attribute]
pub fn paint(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return syn::Error::new(proc_macro2::Span::call_site(), "paint takes no options").to_compile_error().into();
    }
    extract_paints(input)
}

fn extract_paints(input: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(input as syn::Item);
    let mut inline = InlinePaints::default();
    inline.visit_item_mut(&mut item);
    if let Some(error) = inline.error {
        return error.to_compile_error().into();
    }
    let shaders = inline.shaders;
    quote!(#(#shaders)* #[cfg(not(target_arch = "spirv"))] #[allow(clippy::wildcard_imports)] #item).into()
}

#[derive(Default)]
struct InlinePaints {
    method: String,
    bindings: HashSet<String>,
    method_receiver: bool,
    next: usize,
    shaders: Vec<TokenStream2>,
    error: Option<syn::Error>,
}

impl VisitMut for InlinePaints {
    fn visit_impl_item_fn_mut(&mut self, i: &mut syn::ImplItemFn) {
        self.method = i.sig.ident.to_string();
        self.bindings = host_bindings(i.sig.inputs.iter(), &i.block);
        self.method_receiver = true;
        self.visit_block_mut(&mut i.block);
    }

    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        self.method = i.sig.ident.to_string();
        self.bindings = host_bindings(i.sig.inputs.iter(), &i.block);
        self.method_receiver = false;
        self.visit_block_mut(&mut i.block);
    }

    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        if let syn::Expr::MethodCall(call) = i {
            let text = call.method == "paint_text";
            if (call.method == "paint_quad" || text) && !call.args.is_empty() {
                match self.expand_call(call, text) {
                    Ok(Some(replacement)) => {
                        *i = replacement;
                        return;
                    }
                    Ok(None) => {}
                    Err(error) => {
                        self.error = Some(error);
                        return;
                    }
                }
            }
        }
        visit_expr_mut(self, i);
    }
}

impl InlinePaints {
    fn expand_call(&mut self, call: &syn::ExprMethodCall, text: bool) -> syn::Result<Option<syn::Expr>> {
        if call.args.len() != 2 {
            return Err(syn::Error::new_spanned(
                &call.args,
                "paint_quad and paint_text take geometry followed by a typed shader closure; shader inputs are inferred",
            ));
        }
        let Some(syn::Expr::Closure(closure)) = call.args.last().cloned() else {
            return Ok(None);
        };
        if !closure.inputs.iter().all(|input| matches!(input, syn::Pat::Type(_))) {
            return Err(syn::Error::new_spanned(&closure.inputs, "inline paint closure parameters require explicit types"));
        }
        if closure.inputs.len() < 2 {
            return Err(syn::Error::new_spanned(
                &closure.inputs,
                "inline paint closures require the fragment input followed by the program globals",
            ));
        }
        let input_start = 2;
        let input_values = self.input_values(&closure, input_start)?;
        let block = match closure.body.as_ref() {
            syn::Expr::Block(body) => body.block.clone(),
            expression => syn::parse_quote!({ #expression }),
        };
        let name = format_ident!("__isthmus_inline_{}_{}", self.method, self.next);
        self.next += 1;
        let mut inputs = closure.inputs.iter();
        let syn::Pat::Type(shader_input) = inputs.next().unwrap() else { unreachable!() };
        let syn::Pat::Type(frame) = inputs.next().unwrap() else { unreachable!() };
        let syn::Pat::Ident(shader_name) = shader_input.pat.as_ref() else {
            return Err(syn::Error::new_spanned(&shader_input.pat, "shader fragment requires an identifier"));
        };
        if text {
            let shader_type = match shader_input.ty.as_ref() {
                syn::Type::Path(path) => path.path.segments.last().map(|segment| segment.ident.to_string()),
                _ => None,
            };
            if shader_type.as_deref() != Some("TextFragment") {
                return Err(syn::Error::new_spanned(&shader_input.ty, "paint_text closure's first parameter must be TextFragment"));
            }
        } else if shader_name.ident != "fragment" {
            return Err(syn::Error::new_spanned(
                &shader_input.pat,
                "paint_quad shader input must be the first parameter and named `fragment`",
            ));
        }
        let shader_inputs = inputs
            .map(|input| {
                let syn::Pat::Type(input) = input else { unreachable!() };
                input.clone()
            })
            .collect::<Vec<_>>();
        let captures = shader_inputs.iter().map(paint::Capture::new).collect::<syn::Result<Vec<_>>>()?;
        let expansion = paint::expand(&name, frame, shader_input, &captures, &block, text)?;
        let paint::Expansion { items, instance, pipeline } = expansion;
        self.shaders.push(items);
        let geometry = call.args.first().unwrap().clone();
        Ok(Some(rewrite_call(
            call,
            &Rewrite {
                text,
                closure: &closure,
                geometry: &geometry,
                captures: &captures,
                input_values: &input_values,
                instance: &instance,
                pipeline: &pipeline,
            },
        )))
    }

    fn input_values(&self, closure: &syn::ExprClosure, input_start: usize) -> syn::Result<Vec<syn::Expr>> {
        closure
            .inputs
            .iter()
            .skip(input_start)
            .map(|input| {
                let syn::Pat::Type(input) = input else { unreachable!() };
                let syn::Pat::Ident(name) = input.pat.as_ref() else {
                    return Err(syn::Error::new_spanned(&input.pat, "inferred shader inputs require identifier parameters"));
                };
                let name = &name.ident;
                Ok(if self.bindings.contains(&name.to_string()) || !self.method_receiver {
                    syn::parse_quote!(#name)
                } else {
                    syn::parse_quote!(self.#name)
                })
            })
            .collect()
    }
}

struct Rewrite<'a> {
    text: bool,
    closure: &'a syn::ExprClosure,
    geometry: &'a syn::Expr,
    captures: &'a [paint::Capture],
    input_values: &'a [syn::Expr],
    instance: &'a syn::Ident,
    pipeline: &'a syn::Ident,
}

fn rewrite_call(call: &syn::ExprMethodCall, rewrite: &Rewrite<'_>) -> syn::Expr {
    let Rewrite {
        text,
        closure,
        geometry,
        captures,
        input_values,
        instance,
        pipeline,
    } = rewrite;
    let values = captures
        .iter()
        .zip(input_values.iter())
        .map(|(capture, value)| {
            let frame: syn::Expr = syn::parse_quote!(__isthmus_frame);
            let field = &capture.name;
            let value = capture.encode(value, &frame);
            quote!(#field: #value)
        })
        .collect::<Vec<_>>();
    let payload = if *text {
        quote!(#instance { line: __isthmus_geometry, #(#values),* })
    } else if values.is_empty() {
        quote!(#instance {})
    } else {
        quote!(#instance { #(#values),* })
    };
    let isthmus = isthmus_path();
    let generics = if *text {
        quote!(::<#pipeline::Pipeline, _>)
    } else {
        quote!(::<#pipeline::Pipeline, _, _>)
    };
    let receiver = &call.receiver;
    let method = &call.method;
    syn::parse2(quote!(
        #receiver.#method #generics (#geometry, |__isthmus_frame, __isthmus_geometry| {
            use #isthmus::float::FloatExt as _;
            let _ = #closure;
            #payload
        })
    ))
    .unwrap()
}

fn host_bindings<'a>(inputs: impl Iterator<Item = &'a syn::FnArg>, block: &syn::Block) -> HashSet<String> {
    let mut bindings = HostBindings::default();
    for input in inputs {
        if let syn::FnArg::Typed(input) = input {
            bindings.visit_pat(&input.pat);
        }
    }
    bindings.visit_block(block);
    bindings.names
}

#[derive(Default)]
struct HostBindings {
    names: HashSet<String>,
}

impl<'ast> Visit<'ast> for HostBindings {
    fn visit_pat_ident(&mut self, i: &'ast syn::PatIdent) {
        self.names.insert(i.ident.to_string());
        visit::visit_pat_ident(self, i);
    }

    fn visit_expr_closure(&mut self, _closure: &'ast syn::ExprClosure) {}
}

fn program_items(input: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    let name = input.ident.clone();
    let syn::Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "program globals require a struct"));
    };
    let syn::Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(input, "program globals require named fields"));
    };
    if !fields.named.iter().any(|field| field.ident.as_ref().is_some_and(|field| field == "info")) {
        return Err(syn::Error::new_spanned(input, "program globals require an `info: FrameInfo` field"));
    }
    let module = program_module(&name)?;
    let isthmus = isthmus_path();
    Ok(quote! {
        #module

        #[cfg(not(target_arch = "spirv"))]
        impl #name {
            fn __isthmus_frame_info_mut(&mut self) -> &mut #isthmus::FrameInfo {
                let _: &mut #isthmus::FrameInfo = &mut self.info;
                &mut self.info
            }

            pub const fn program() -> #isthmus::__private::Program<Self> {
                #isthmus::__private::Program::new(__ISTHMUS_SHADER, Self::__isthmus_frame_info_mut)
            }
        }

    })
}

fn program_module(shared: &syn::Ident) -> syn::Result<TokenStream2> {
    let Ok(crate_dir) = env::var("CARGO_MANIFEST_DIR") else {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), "missing package directory"));
    };
    let artifact = match shader_artifact(Path::new(&crate_dir)) {
        Ok(artifact) => artifact,
        Err(error) => return Err(syn::Error::new(proc_macro2::Span::call_site(), error)),
    };
    if env::var("CARGO_PKG_NAME").as_deref() != Ok("isthmus-shader") {
        let source = shader_source(Path::new(&crate_dir)).map_err(|error| syn::Error::new(proc_macro2::Span::call_site(), error))?;
        let dependency = shader_dependency(Path::new(&crate_dir)).map_err(|error| syn::Error::new(proc_macro2::Span::call_site(), error))?;
        ensure_fresh(&artifact, &source, dependency.as_deref())?;
    }
    let artifact = artifact.to_string_lossy();
    let isthmus = isthmus_path();
    Ok(quote! {
        #[doc(hidden)]
        pub mod __isthmus_quad {
            use super::*;
            use #isthmus::float::FloatExt as _;

            #[#isthmus::spirv_std::spirv(vertex)]
            pub fn vertex(
                #[spirv(vertex_index)] vertex: u32,
                #[spirv(instance_index)] draw_index: u32,
                #[spirv(push_constant)] frame: &#shared,
                #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] draws: &[#isthmus::__private::DrawRecord],
                #[spirv(position)] out_position: &mut #isthmus::glam::Vec4,
                #[spirv(location = 0)] out_pixel: &mut #isthmus::glam::Vec2,
                #[spirv(location = 1, flat)] out_draw_index: &mut u32,
            ) {
                let draw = draws[draw_index as usize];
                let sample = draw.quad.sample(vertex, frame.info.screen_size);
                *out_position = sample.position;
                *out_pixel = sample.pixel;
                *out_draw_index = draw_index;
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        const __ISTHMUS_SHADER: #isthmus::__private::ShaderModule =
            #isthmus::__private::ShaderModule::new(
                include_bytes!(#artifact),
                #isthmus::__private::shader_module_name(module_path!()),
            );
    })
}

fn ensure_fresh(artifact: &Path, source: &Path, dependency: Option<&Path>) -> syn::Result<()> {
    if !shader_is_fresh(artifact, source, dependency) {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), "shader artifact is missing or stale; run `isthmus build`"));
    }
    Ok(())
}
