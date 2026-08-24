use crate::{fragment_entry, isthmus_path};
use quote::{format_ident, quote};
use syn::{Block, Expr, Ident, Pat, PatType, Type, parse_quote};

#[derive(Clone)]
pub struct Capture {
    pub name: Ident,
    source: Type,
    kind: CaptureKind,
}

#[derive(Clone, Copy)]
enum CaptureKind {
    Plain,
    Bool,
    Image,
}

impl Capture {
    pub fn new(input: &PatType) -> syn::Result<Self> {
        let Pat::Ident(name) = input.pat.as_ref() else {
            return Err(syn::Error::new_spanned(&input.pat, "shader inputs require identifiers"));
        };
        let source = input.ty.as_ref().clone();
        let kind = if matches!(&source, Type::Path(path) if path.path.is_ident("bool")) {
            CaptureKind::Bool
        } else if matches!(&source, Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Image")) {
            CaptureKind::Image
        } else {
            CaptureKind::Plain
        };
        Ok(Self {
            name: name.ident.clone(),
            source,
            kind,
        })
    }

    pub const fn is_image(&self) -> bool {
        matches!(self.kind, CaptureKind::Image)
    }

    fn storage(&self, isthmus: &proc_macro2::TokenStream) -> Type {
        match self.kind {
            CaptureKind::Bool => parse_quote!(u32),
            CaptureKind::Image => parse_quote!(#isthmus::__private::ImageHandle),
            CaptureKind::Plain => self.source.clone(),
        }
    }

    fn unpack(&self, isthmus: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let name = &self.name;
        match self.kind {
            CaptureKind::Plain => quote!(let #name = instance.#name;),
            CaptureKind::Bool => quote!(let #name = instance.#name != 0;),
            CaptureKind::Image => quote!(let #name = #isthmus::__private::ShaderImage::new(images, instance.#name);),
        }
    }

    pub fn encode(&self, value: &Expr, frame: &Expr) -> proc_macro2::TokenStream {
        match self.kind {
            CaptureKind::Plain => quote!(#value),
            CaptureKind::Bool => quote!(u32::from(#value)),
            CaptureKind::Image => quote!((#frame).__image(&(#value))),
        }
    }
}

pub struct Expansion {
    pub items: proc_macro2::TokenStream,
    pub instance: Ident,
    pub pipeline: Ident,
}

pub fn expand(name: &Ident, shader_input: &PatType, captures: &[Capture], body: &Block, text: bool) -> syn::Result<Expansion> {
    let isthmus = isthmus_path();
    let shader_name = format_ident!("__isthmus_paint_{name}");
    let instance_name = format_ident!("__IsthmusPaint{}", pascal(&name.to_string()));
    let Pat::Ident(shader_input_name) = shader_input.pat.as_ref() else {
        return Err(syn::Error::new_spanned(&shader_input.pat, "shader fragment requires an identifier"));
    };
    let shader_input_name = &shader_input_name.ident;
    let shader_input_type = shader_input.ty.as_ref();
    let line_type = syn::parse2::<Type>(quote!(#isthmus::text::Line))?;
    let mut payload = Vec::<(Ident, Type)>::new();
    if text {
        payload.push((format_ident!("line"), line_type));
    }
    for capture in captures {
        payload.push((capture.name.clone(), capture.storage(&isthmus)));
    }
    let payload_names = payload.iter().map(|(name, _)| name);
    let payload_types = payload.iter().map(|(_, storage)| storage);
    let payload_fields = quote!(#(#payload_names: #payload_types),*);
    let payload_types = payload.iter().map(|(_, storage)| storage);
    let shader_data = if payload.is_empty() {
        quote!(unsafe impl #isthmus::ShaderData for #instance_name {})
    } else {
        quote!(unsafe impl #isthmus::ShaderData for #instance_name where #(#payload_types: #isthmus::ShaderData,)* {})
    };
    let line_unpack = text.then(|| quote!(let line = instance.line;));
    let payload_unpack = captures.iter().map(|capture| capture.unpack(&isthmus));
    let instance_load = (!payload.is_empty()).then(|| {
        quote! {
            let instance = unsafe {
                #isthmus::spirv_std::ByteAddressableBuffer::from_slice(payload).load::<#instance_name>(draw.payload)
            };
        }
    });
    let shade_body = if text {
        quote!({
            let #shader_input_name: #shader_input_type = #isthmus::TextFragment::new(fragment, line, placed_glyphs, glyphs, edges);
            #body
        })
    } else {
        quote!(#body)
    };
    let entry = fragment_entry(
        text,
        !payload.is_empty(),
        captures.iter().any(Capture::is_image),
        shader_input.ty.as_ref(),
        shader_input_name,
        &quote! {
            #instance_load
            #line_unpack
            #(#payload_unpack)*
            *out_color = #shade_body;
        },
    );
    let module_name = quote!(#isthmus::__private::shader_module_name(module_path!()));
    let items = quote! {
        #[repr(C)]
        #[derive(Clone, Copy)]
        #[cfg_attr(not(target_arch = "spirv"), derive(#isthmus::__private::bytemuck::Pod, #isthmus::__private::bytemuck::Zeroable))]
        #[doc(hidden)]
        pub struct #instance_name {
            #payload_fields
        }
        #shader_data

        pub mod #shader_name {
            use super::*;
            use #isthmus::FloatExt as _;

            #[cfg(not(target_arch = "spirv"))]
            pub struct Pipeline;

            #[cfg(not(target_arch = "spirv"))]
            impl #isthmus::__private::ShaderSpec for Pipeline {
                type Instance = #instance_name;
                const PIPELINE: #isthmus::__private::PaintPipeline = #isthmus::__private::PaintPipeline::new(#module_name);
            }

            #entry
        }
    };
    Ok(Expansion {
        items,
        instance: instance_name,
        pipeline: shader_name,
    })
}

fn pascal(name: &str) -> String {
    let mut output = String::new();
    let mut upper = true;
    for character in name.chars() {
        if character == '_' {
            upper = true;
        } else if upper {
            output.extend(character.to_uppercase());
            upper = false;
        } else {
            output.push(character);
        }
    }
    output
}
