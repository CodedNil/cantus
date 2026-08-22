use isthmus_build::artifact::{shader_artifact, shader_source};
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::{collections::HashSet, env, fs, path::Path};
use syn::visit::{self, Visit};
use syn::visit_mut::{VisitMut, visit_expr_method_call_mut, visit_expr_mut};
use syn::{parse::Parse, parse::ParseStream};

mod buffer;
mod paint;

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

/// Declares host/GPU data and rejects layouts that Rust-GPU cannot expose to SPIR-V.
#[proc_macro_attribute]
pub fn data(_args: TokenStream, input: TokenStream) -> TokenStream {
    buffer::attribute(input)
}

/// Extracts inline paint closures and keeps their surrounding implementation on the host.
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
    quote!(#(#shaders)* #[cfg(not(target_arch = "spirv"))] #item).into()
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

    fn visit_expr_method_call_mut(&mut self, i: &mut syn::ExprMethodCall) {
        let text = i.method == "paint_text";
        if (i.method != "paint_quad" && !text) || i.args.is_empty() {
            visit_expr_method_call_mut(self, i);
            return;
        }
        if i.args.len() != 2 {
            self.error = Some(syn::Error::new_spanned(
                &i.args,
                "paint_quad and paint_text take geometry followed by a typed shader closure; closure captures are inferred",
            ));
            return;
        }
        let Some(syn::Expr::Closure(closure)) = i.args.last().cloned() else {
            visit_expr_method_call_mut(self, i);
            return;
        };
        let geometry = i.args.first().cloned();
        let mut captures = Vec::<syn::Expr>::new();
        let input_start = 2;
        let captured_count = closure
            .inputs
            .iter()
            .skip(input_start)
            .filter(|input| {
                let syn::Pat::Type(input) = input else { return true };
                !paint::is_image_heap(input.ty.as_ref()) && !paint::is_text_resource(input.ty.as_ref())
            })
            .count();
        if captured_count != 0 {
            for input in closure.inputs.iter().skip(input_start) {
                let syn::Pat::Type(input) = input else { continue };
                if paint::is_image_heap(input.ty.as_ref()) || paint::is_text_resource(input.ty.as_ref()) {
                    continue;
                }
                let syn::Pat::Ident(name) = input.pat.as_ref() else {
                    self.error = Some(syn::Error::new_spanned(&input.pat, "implicit paint captures require identifier parameters"));
                    return;
                };
                let name = &name.ident;
                captures.push(if self.bindings.contains(&name.to_string()) || !self.method_receiver {
                    syn::parse_quote!(#name)
                } else {
                    syn::parse_quote!(self.#name)
                });
            }
        }
        if captured_count != captures.len() {
            self.error = Some(syn::Error::new_spanned(
                &closure.inputs,
                "inline paint closure takes globals and fragment followed by typed inferred captures; image heaps are implicit",
            ));
            return;
        }
        if !closure.inputs.iter().all(|input| matches!(input, syn::Pat::Type(_))) {
            self.error = Some(syn::Error::new_spanned(&closure.inputs, "inline paint closure parameters require explicit types"));
            return;
        }
        let mut block = match closure.body.as_ref() {
            syn::Expr::Block(body) => body.block.clone(),
            expression => syn::parse_quote!({ #expression }),
        };
        let isthmus = isthmus_path();
        let mut image_access = ImageAccess { isthmus: &isthmus, used: false };
        image_access.visit_block_mut(&mut block);
        let images = image_access.used.then(|| quote!(images: &#isthmus::__private::Images,));
        let name = format_ident!("__isthmus_inline_{}_{}", self.method, self.next);
        self.next += 1;
        let inputs = &closure.inputs;
        let block = &block;
        let mut inputs = inputs.iter();
        let frame = inputs.next().unwrap();
        let shader_input = inputs.next().unwrap();
        let function = if text {
            let syn::Pat::Type(text) = shader_input else { unreachable!() };
            let syn::Pat::Ident(text_name) = text.pat.as_ref() else {
                self.error = Some(syn::Error::new_spanned(&text.pat, "text fragment requires an identifier"));
                return;
            };
            let text_type = match text.ty.as_ref() {
                syn::Type::Path(path) => path.path.segments.last().map(|segment| segment.ident.to_string()),
                _ => None,
            };
            if text_type.as_deref() != Some("TextFragment") {
                self.error = Some(syn::Error::new_spanned(&text.ty, "paint_text closure's second parameter must be TextFragment"));
                return;
            }
            let text_name = &text_name.ident;
            let rest = inputs;
            syn::parse2::<syn::ItemFn>(quote! {
                fn #name(
                    #frame,
                    line: ::isthmus::text::Line,
                    #(#rest,)*
                    placed_glyphs: &[::isthmus::text::PlacedGlyph],
                    glyphs: &[::isthmus::text::Glyph],
                    edges: &[::isthmus::text::Edge],
                    #images
                ) {
                    line.quad().shade(|fragment| {
                        let #text_name = ::isthmus::TextFragment::new(fragment, line, placed_glyphs, glyphs, edges);
                        #block
                    })
                }
            })
        } else {
            let syn::Pat::Type(fragment) = shader_input else { unreachable!() };
            let fragment = &fragment.pat;
            let rest = inputs;
            syn::parse2::<syn::ItemFn>(quote! {
                fn #name(#frame, quad: ::isthmus::Quad, #(#rest,)* #images) {
                    quad.shade(|#fragment| #block)
                }
            })
        };
        let mut function = match function {
            Ok(function) => function,
            Err(error) => {
                self.error = Some(error);
                return;
            }
        };
        if let Some(syn::FnArg::Typed(frame)) = function.sig.inputs.first_mut()
            && let syn::Pat::Ident(name) = frame.pat.as_mut()
            && name.ident == "_frame"
        {
            name.ident = format_ident!("frame");
        }
        match paint::expand(&function) {
            Ok(shader) => self.shaders.push(shader),
            Err(error) => {
                self.error = Some(error);
                return;
            }
        }
        let geometry = geometry.unwrap();
        i.method = syn::parse_quote!(paint);
        i.args = if text {
            syn::parse_quote!(#name(#geometry, #(#captures),*))
        } else {
            syn::parse_quote!(#name(#isthmus::__private::IntoQuad::into_quad(#geometry), #(#captures),*))
        };
    }
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

struct ImageAccess<'a> {
    isthmus: &'a TokenStream2,
    used: bool,
}

impl VisitMut for ImageAccess<'_> {
    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        let syn::Expr::MethodCall(call) = i else {
            visit_expr_mut(self, i);
            return;
        };
        let syn::Expr::Path(receiver) = call.receiver.as_ref() else {
            visit_expr_method_call_mut(self, call);
            return;
        };
        if !receiver.path.is_ident("fragment") {
            visit_expr_method_call_mut(self, call);
            return;
        }
        if call.method == "sample_image" && call.args.len() == 2 {
            let isthmus = self.isthmus;
            let mut arguments = call.args.iter();
            let handle = arguments.next().unwrap();
            let uv = arguments.next().unwrap();
            *i = syn::parse_quote!(#isthmus::__private::sample_image(images, #handle, #uv));
            self.used = true;
        } else {
            visit_expr_method_call_mut(self, call);
        }
    }
}

/// Declares the package's checked-in SPIR-V module and its typed pipeline table.
#[proc_macro]
pub fn shader_module(input: TokenStream) -> TokenStream {
    let declaration = syn::parse_macro_input!(input as ShaderModuleDeclaration);
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
    let pipelines = match pipeline_paths(Path::new(&crate_dir)) {
        Ok(pipelines) => pipelines,
        Err(error) => return syn::Error::new(proc_macro2::Span::call_site(), error).to_compile_error().into(),
    };
    let indices = 0..pipelines.len();
    let default_text_index = pipelines.len();
    let isthmus = isthmus_path();
    let visibility = declaration.visibility;
    let name = declaration.name;
    let shared = declaration.shared;
    quote! {
        #[doc(hidden)]
        pub mod __isthmus_quad {
            use super::*;

            #[allow(unused_variables)]
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
                let sample = draw.quad.sample(vertex, #isthmus::FrameGlobals::frame_info(frame).screen_size);
                *out_position = sample.position;
                *out_pixel = sample.pixel;
                *out_draw_index = draw_index;
            }
        }

        #[doc(hidden)]
        pub mod __isthmus_text {
            use super::*;

            #[cfg(not(target_arch = "spirv"))]
            pub struct Pipeline;

            #[cfg(not(target_arch = "spirv"))]
            impl #isthmus::__private::ShaderSpec for Pipeline {
                type Shared = #shared;
                type Instance = #isthmus::text::Line;
                const NAME: &'static str = #isthmus::__private::shader_module_name(module_path!());
            }

            #[allow(unused_variables)]
            #[#isthmus::spirv_std::spirv(fragment)]
            pub fn fragment(
                #[spirv(location = 0)] pixel: #isthmus::glam::Vec2,
                #[spirv(location = 1, flat)] draw_index: u32,
                #[spirv(push_constant)] frame: &#shared,
                #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] draws: &[#isthmus::__private::DrawRecord],
                #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] payload: &[u32],
                #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] placed_glyphs: &[#isthmus::text::PlacedGlyph],
                #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] glyphs: &[#isthmus::text::Glyph],
                #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] edges: &[#isthmus::text::Edge],
                #[spirv(location = 0)] out_color: &mut #isthmus::glam::Vec4,
            ) {
                let draw = draws[draw_index as usize];
                let line = unsafe {
                    #isthmus::spirv_std::ByteAddressableBuffer::from_slice(payload).load::<#isthmus::text::Line>(draw.payload)
                };
                let local = draw.quad.local(pixel);
                let fragment = #isthmus::__private::Fragment {
                    pixel,
                    local,
                    uv: local / draw.quad.size + 0.5,
                    size: draw.quad.size,
                };
                let text = #isthmus::TextFragment::new(fragment, line, placed_glyphs, glyphs, edges);
                *out_color = text.color(text.alpha());
            }
        }

        #(#[cfg(not(target_arch = "spirv"))]
        impl #isthmus::__private::PipelineIdentity for #pipelines {
            const KEY: #isthmus::__private::PipelineKey = #isthmus::__private::PipelineKey::new(#indices);
        })*

        #[cfg(not(target_arch = "spirv"))]
        impl #isthmus::__private::PipelineIdentity for __isthmus_text::Pipeline {
            const KEY: #isthmus::__private::PipelineKey = #isthmus::__private::PipelineKey::new(#default_text_index);
        }

        #[cfg(not(target_arch = "spirv"))]
        fn __isthmus_register_pipelines(canvas: &mut #isthmus::__private::Canvas<#shared>) {
            #(canvas.register::<#pipelines>();)*
            canvas.register_default_text::<__isthmus_text::Pipeline>();
        }

        #[cfg(not(target_arch = "spirv"))]
        #visibility const #name: #isthmus::__private::ShaderModule<#shared> =
            #isthmus::__private::ShaderModule::new(
                include_bytes!(#artifact),
                __isthmus_register_pipelines,
                #isthmus::__private::shader_module_name(module_path!()),
            );
    }
    .into()
}

struct ShaderModuleDeclaration {
    visibility: syn::Visibility,
    name: syn::Ident,
    shared: syn::Type,
}

impl Parse for ShaderModuleDeclaration {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let shared = input.parse()?;
        Ok(Self { visibility, name, shared })
    }
}

fn pipeline_paths(crate_dir: &Path) -> Result<Vec<syn::Path>, String> {
    let source = shader_source(crate_dir)?;
    let root = source
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .filter(|_| source.file_name().is_some_and(|name| name == "mod.rs"))
        .map(str::to_owned);
    let mut pipelines = Vec::new();
    scan_module(&source, &root.into_iter().collect::<Vec<_>>(), &mut pipelines)?;
    pipelines
        .into_iter()
        .map(|path| syn::parse_str(&std::format!("crate::{}", path.join("::"))).map_err(|error| error.to_string()))
        .collect()
}

fn scan_module(source: &Path, module: &[String], pipelines: &mut Vec<Vec<String>>) -> Result<(), String> {
    let text = fs::read_to_string(source).map_err(|error| std::format!("failed to read shader source {}: {error}", source.display()))?;
    let file = syn::parse_file(&text).map_err(|error| std::format!("failed to parse shader source {}: {error}", source.display()))?;
    let directory = source.parent().unwrap_or_else(|| Path::new("."));
    for item in &file.items {
        let attributes: &[syn::Attribute] = match item {
            syn::Item::Fn(item) => &item.attrs,
            syn::Item::Impl(item) => &item.attrs,
            _ => &[],
        };
        let painted = attributes
            .iter()
            .any(|attribute| attribute.path().segments.last().is_some_and(|segment| segment.ident == "paint"));
        if painted {
            let mut calls = PaintCalls::default();
            calls.visit_item(item);
            for name in calls.names {
                let mut path = module.to_owned();
                path.push(name);
                path.push(String::from("Pipeline"));
                pipelines.push(path);
            }
        }
        if let syn::Item::Mod(child) = item {
            let mut child_module = module.to_owned();
            child_module.push(child.ident.to_string());
            if child.content.is_some() {
                continue;
            }
            let direct = directory.join(std::format!("{}.rs", child.ident));
            let nested = directory.join(child.ident.to_string()).join("mod.rs");
            let child_source = if direct.exists() { direct } else { nested };
            scan_module(&child_source, &child_module, pipelines)?;
        }
    }
    Ok(())
}

#[derive(Default)]
struct PaintCalls {
    method: String,
    next: usize,
    names: Vec<String>,
}

impl<'ast> Visit<'ast> for PaintCalls {
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        self.method = i.sig.ident.to_string();
        visit::visit_block(self, &i.block);
    }

    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.method = i.sig.ident.to_string();
        visit::visit_block(self, &i.block);
    }

    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if matches!(i.method.to_string().as_str(), "paint_quad" | "paint_text") && matches!(i.args.last(), Some(syn::Expr::Closure(_))) {
            self.names.push(std::format!("__isthmus_paint___isthmus_inline_{}_{}", self.method, self.next));
            self.next += 1;
        }
        visit::visit_expr_method_call(self, i);
    }
}
