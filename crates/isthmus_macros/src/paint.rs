use crate::isthmus_path;
use quote::{format_ident, quote};
use syn::{Expr, FnArg, ItemFn, Pat, Stmt, Type, parse_quote};

pub fn expand(function: &ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let isthmus = isthmus_path();
    let name = &function.sig.ident;
    let visibility = &function.vis;
    let shader_name = format_ident!("__isthmus_paint_{name}");
    let instance_name = format_ident!("__IsthmusPaint{}", pascal(&name.to_string()));
    let drawable_name = format_ident!("__IsthmusDrawable{}", pascal(&name.to_string()));
    let mut frame = None;
    let mut values = Vec::new();
    let mut resources = Vec::new();
    for argument in &function.sig.inputs {
        let FnArg::Typed(argument) = argument else {
            return Err(syn::Error::new_spanned(argument, "paint functions cannot have a receiver"));
        };
        let Pat::Ident(pattern) = argument.pat.as_ref() else {
            return Err(syn::Error::new_spanned(&argument.pat, "paint parameters require identifiers"));
        };
        if pattern.ident == "frame" {
            frame = Some(argument.clone());
        } else if matches!(argument.ty.as_ref(), Type::Reference(_)) {
            if !is_image_heap(argument.ty.as_ref()) && !is_text_resource(argument.ty.as_ref()) {
                return Err(syn::Error::new_spanned(
                    argument,
                    "paint resources are internal; capture ShaderData values, Images, or text resources",
                ));
            }
            resources.push(argument.clone());
        } else {
            values.push(argument.clone());
        }
    }
    let frame = frame.ok_or_else(|| syn::Error::new_spanned(&function.sig, "paint requires `frame`"))?;
    if !matches!(function.sig.inputs.first(), Some(FnArg::Typed(argument)) if matches!(argument.pat.as_ref(), Pat::Ident(pattern) if pattern.ident == "frame")) {
        return Err(syn::Error::new_spanned(&frame, "paint globals must be the first parameter"));
    }
    let (last, setup) = function
        .block
        .stmts
        .split_last()
        .ok_or_else(|| syn::Error::new_spanned(&function.block, "paint function is empty"))?;
    let Stmt::Expr(Expr::MethodCall(shade), _) = last else {
        return Err(syn::Error::new_spanned(last, "paint must end with `quad.shade(|fragment| ...)`"));
    };
    if shade.method != "shade" || shade.args.len() != 1 {
        return Err(syn::Error::new_spanned(shade, "paint must end with `quad.shade(|fragment| ...)`"));
    }
    let Expr::Closure(closure) = shade.args.first().unwrap() else {
        return Err(syn::Error::new_spanned(&shade.args, "shade requires a fragment closure"));
    };
    if closure.inputs.len() != 1 || !matches!(closure.inputs.first(), Some(Pat::Ident(pattern)) if pattern.ident == "fragment") {
        return Err(syn::Error::new_spanned(&closure.inputs, "shade closure must accept `fragment`"));
    }

    let quad = &shade.receiver;
    let shade_body = &closure.body;
    let frame_ty = &frame.ty;
    let payload = values
        .iter()
        .filter(|argument| !matches!(argument.pat.as_ref(), Pat::Ident(pattern) if pattern.ident == "quad"))
        .collect::<Vec<_>>();
    let payload_names = payload.iter().map(|argument| argument.pat.as_ref().clone()).collect::<Vec<_>>();
    let payload_types = payload
        .iter()
        .map(|argument| {
            if is_bool(argument.ty.as_ref()) {
                parse_quote!(u32)
            } else {
                argument.ty.as_ref().clone()
            }
        })
        .collect::<Vec<_>>();
    let value_arguments = values.iter();
    let payload_fields = if payload.is_empty() {
        quote!(_isthmus: u32)
    } else {
        quote!(#(#payload_names: #payload_types),*)
    };
    let payload_values = payload.iter().map(|argument| {
        let name = argument.pat.as_ref();
        if is_bool(argument.ty.as_ref()) {
            quote!(#name: u32::from(#name))
        } else {
            quote!(#name: #name)
        }
    });
    let payload_init = if payload.is_empty() {
        quote!(#instance_name { _isthmus: 0 })
    } else {
        quote!(#instance_name { #(#payload_values),* })
    };
    let payload_unpack = payload.iter().map(|argument| {
        let name = argument.pat.as_ref();
        if is_bool(argument.ty.as_ref()) {
            quote!(let #name = instance.#name != 0;)
        } else {
            quote!(let #name = instance.#name;)
        }
    });
    let resource_names = resources.iter().map(|resource| resource.pat.as_ref());
    let mut shader_resources = resources.clone();
    for resource in &mut shader_resources {
        let binding = text_binding(resource.ty.as_ref()).unwrap_or(5);
        let storage = matches!(resource.ty.as_ref(), Type::Reference(reference) if matches!(reference.elem.as_ref(), Type::Slice(_)));
        resource.attrs.push(if storage {
            parse_quote!(#[spirv(storage_buffer, descriptor_set = 0, binding = #binding)])
        } else {
            parse_quote!(#[spirv(descriptor_set = 0, binding = #binding)])
        });
    }
    let module_name = quote!(#isthmus::__private::shader_module_name(module_path!()));
    Ok(quote! {
        #[repr(C)]
        #[derive(Clone, Copy)]
        #[cfg_attr(not(target_arch = "spirv"), derive(#isthmus::__private::bytemuck::Pod, #isthmus::__private::bytemuck::Zeroable))]
        #[doc(hidden)]
        pub struct #instance_name {
            #payload_fields
        }
        impl #isthmus::ShaderData for #instance_name {}

        pub mod #shader_name {
            use super::*;

            #[cfg(not(target_arch = "spirv"))]
            pub struct Pipeline;

            #[cfg(not(target_arch = "spirv"))]
            impl #isthmus::__private::ShaderSpec for Pipeline {
                type Shared = #frame_ty;
                type Instance = #instance_name;
                const NAME: &'static str = #module_name;
            }

            #[cfg_attr(not(target_arch = "spirv"), allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref))]
            fn fragment_impl(
                pixel: #isthmus::glam::Vec2,
                frame: &#frame_ty,
                instance: #instance_name,
                quad: #isthmus::Quad,
                #(#resources,)*
            ) -> #isthmus::glam::Vec4 {
                #(#payload_unpack)*
                let frame = *frame;
                #(#setup)*
                let local = quad.local(pixel);
                let fragment = #isthmus::__private::Fragment {
                    pixel,
                    local,
                    uv: local / quad.size + 0.5,
                    size: quad.size,
                };
                #shade_body
            }

            #[allow(unused_variables)]
            #[#isthmus::spirv_std::spirv(fragment)]
            pub fn fragment(
                #[spirv(location = 0)] pixel: #isthmus::glam::Vec2,
                #[spirv(location = 1, flat)] draw_index: u32,
                #[spirv(push_constant)] frame: &#frame_ty,
                #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] draws: &[#isthmus::__private::DrawRecord],
                #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] payload: &[u32],
                #(#shader_resources,)*
                #[spirv(location = 0)] out_color: &mut #isthmus::glam::Vec4,
            ) {
                let draw = draws[draw_index as usize];
                let instance = unsafe {
                    #isthmus::spirv_std::ByteAddressableBuffer::from_slice(payload).load::<#instance_name>(draw.payload)
                };
                *out_color = fragment_impl(pixel, frame, instance, draw.quad, #(#resource_names),*);
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        #[repr(C)]
        struct #drawable_name {
            quad: #isthmus::Quad,
            instance: #instance_name,
        }

        #[cfg(not(target_arch = "spirv"))]
        impl #isthmus::__private::Paint<#frame_ty> for #drawable_name {
            fn paint(self, canvas: &mut #isthmus::__private::Canvas<#frame_ty>) {
                canvas.emit::<#shader_name::Pipeline>(self.quad, self.instance);
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        #visibility fn #name(#(#value_arguments),*) -> impl #isthmus::__private::Paint<#frame_ty> + use<> {
            #drawable_name {
                quad: #quad,
                instance: #payload_init,
            }
        }
    })
}

pub fn is_image_heap(ty: &Type) -> bool {
    let Type::Reference(reference) = ty else { return false };
    matches!(reference.elem.as_ref(), Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Images"))
}

pub fn is_text_resource(ty: &Type) -> bool {
    text_binding(ty).is_some()
}

fn text_binding(ty: &Type) -> Option<u32> {
    let Type::Reference(reference) = ty else { return None };
    let Type::Slice(slice) = reference.elem.as_ref() else { return None };
    let Type::Path(path) = slice.elem.as_ref() else { return None };
    match path.path.segments.last()?.ident.to_string().as_str() {
        "PlacedGlyph" => Some(2),
        "Glyph" => Some(3),
        "Edge" => Some(4),
        _ => None,
    }
}

fn is_bool(ty: &Type) -> bool {
    matches!(ty, Type::Path(path) if path.path.is_ident("bool"))
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
