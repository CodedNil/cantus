use crate::{
    abi::{fragment_functions, vertex_functions},
    isthmus_path,
    model::PassSchema,
    render,
    syntax::{GpuParameter, GpuRole, PassOptions, gpu_attribute},
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Fields, File, FnArg, GenericArgument, ImplItem, Item, ItemFn, ItemImpl, PathArguments, Result as SynResult, Type, parse_macro_input, parse_quote};

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let option_tokens = TokenStream2::from(args);
    let item = parse_macro_input!(input as Item);
    let result = match item {
        Item::Impl(implementation) => expand_pass_impl(implementation, &option_tokens),
        Item::Struct(mut pass) => {
            if option_tokens.is_empty() {
                if !pass.generics.params.is_empty() {
                    return syn::Error::new_spanned(&pass.generics, "render pass types cannot be generic").to_compile_error().into();
                }
                let pass_fields = match &pass.fields {
                    Fields::Named(fields) => fields
                        .named
                        .iter()
                        .filter_map(|field| {
                            let Type::Path(path) = &field.ty else {
                                return None;
                            };
                            let segment = path.path.segments.last()?;
                            let name = segment.ident.to_string();
                            let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                                return None;
                            };
                            ((name == "Instances" || name == "Instance") && matches!(arguments.args.first(), Some(GenericArgument::Type(_))) && arguments.args.len() == 1)
                                .then(|| field.ident.as_ref().unwrap())
                        })
                        .collect::<Vec<_>>(),
                    _ => Vec::new(),
                };
                if pass_fields.len() > 1 {
                    return syn::Error::new_spanned(
                        &pass.fields,
                        "a shader pass can own at most one instance container; use a system to coordinate multiple passes",
                    )
                    .to_compile_error()
                    .into();
                }
                let name = &pass.ident;
                let visibility = &pass.vis;
                pass.attrs.push(parse_quote!(#[cfg(feature = "cpu")]));
                let runtime_impl = (!pass_fields.is_empty()).then(|| {
                    let implementation = render::implementation(name, &pass.generics, pass_fields);
                    quote!(#[cfg(feature = "cpu")] #implementation)
                });
                Ok(quote! {
                    #pass
                    #[cfg(not(feature = "cpu"))]
                    #visibility struct #name;
                    #runtime_impl
                })
            } else {
                Err(syn::Error::new_spanned(option_tokens, "pass structs do not take pipeline options"))
            }
        }
        item => Err(syn::Error::new_spanned(item, "pass requires a struct or impl block")),
    };
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}

fn expand_pass_impl(mut implementation: ItemImpl, options: &TokenStream2) -> SynResult<TokenStream2> {
    if !implementation.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(&implementation.generics, "render pass implementations cannot be generic"));
    }
    let fragment = implementation
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Fn(function) if function.sig.ident == "fragment" && has_gpu_marker(&function.attrs) => Some(function),
            _ => None,
        })
        .ok_or_else(|| syn::Error::new_spanned(&implementation, "missing `fragment` function"))?;
    let mut bridge = fragment
        .sig
        .inputs
        .first()
        .and_then(|argument| match argument {
            FnArg::Typed(argument) => match argument.ty.as_ref() {
                Type::Path(path) => Some(path.path.clone()),
                _ => None,
            },
            FnArg::Receiver(_) => None,
        })
        .ok_or_else(|| syn::Error::new_spanned(&fragment.sig, "fragment must accept a named varying type"))?;
    let varying = bridge.segments.last().unwrap().ident.clone();
    bridge.segments.last_mut().unwrap().ident = format_ident!("__isthmus_varyings_{varying}");
    implementation.attrs.push(parse_quote!(#[isthmus_pass_options(#options)]));
    Ok(quote!(#bridge! { #implementation }))
}

pub fn lower(input: TokenStream) -> TokenStream {
    let file = parse_macro_input!(input as File);
    match lower_pass_impl(file) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn lower_pass_impl(mut file: File) -> SynResult<TokenStream2> {
    let implementation_index = file
        .items
        .iter()
        .position(|item| matches!(item, Item::Impl(_)))
        .ok_or_else(|| syn::Error::new_spanned(&file, "pass bridge requires an impl block"))?;
    let Item::Impl(mut implementation) = file.items.remove(implementation_index) else {
        unreachable!()
    };
    let options_index = implementation.attrs.iter().position(|attribute| attribute.path().is_ident("isthmus_pass_options")).unwrap();
    let options_attribute = implementation.attrs.remove(options_index);
    let options = options_attribute.parse_args::<PassOptions>()?;
    let pass_type = (*implementation.self_ty).clone();
    let mut cpu_items = Vec::new();
    for item in implementation.items {
        match item {
            ImplItem::Fn(function) if has_gpu_marker(&function.attrs) => {
                file.items.push(Item::Fn(ItemFn {
                    attrs: function.attrs,
                    vis: function.vis,
                    modifiers: function.modifiers,
                    sig: function.sig,
                    block: Box::new(function.block),
                }));
            }
            item => cpu_items.push(item),
        }
    }
    if !cpu_items.is_empty() {
        implementation.items = cpu_items;
        implementation.attrs.push(parse_quote!(#[cfg(feature = "cpu")]));
        file.items.push(Item::Impl(implementation));
    }
    expand_pass(&mut file, &options, &pass_type)
}

fn expand_pass(file: &mut File, options: &PassOptions, pass_type: &Type) -> SynResult<TokenStream2> {
    let pass_name = match pass_type {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| &segment.ident)
            .ok_or_else(|| syn::Error::new_spanned(pass_type, "pass requires a named type"))?,
        _ => {
            return Err(syn::Error::new_spanned(pass_type, "pass requires a named type"));
        }
    };
    let shader_module = format_ident!("isthmus_{}", pass_name.to_string().to_lowercase());
    let varying_indices = file
        .items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| match item {
            Item::Struct(item) if has_gpu_marker(&item.attrs) => Some(index),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [varying_index] = varying_indices.as_slice() else {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), "shader requires exactly one `#[gpu]` varying struct"));
    };
    let Item::Struct(varyings) = &mut file.items[*varying_index] else { unreachable!() };
    take_gpu_marker(&mut varyings.attrs);
    let Fields::Named(fields) = &mut varyings.fields else {
        return Err(syn::Error::new_spanned(varyings, "Varyings requires named fields"));
    };
    let varying_fields = fields
        .named
        .iter_mut()
        .map(|field| {
            let flat = match take_gpu_role(&mut field.attrs)? {
                Some(GpuRole::Flat) => true,
                Some(_) => {
                    return Err(syn::Error::new_spanned(field, "varying fields only support `#[gpu(flat)]`"));
                }
                None => false,
            };
            Ok((field.ident.clone().unwrap(), field.ty.clone(), flat))
        })
        .collect::<SynResult<Vec<_>>>()?;
    file.items.remove(*varying_index);

    let vertex = take_function(&mut file.items, "vertex")?;
    let fragment = take_function(&mut file.items, "fragment")?;
    let varying_type = match fragment.sig.inputs.first() {
        Some(FnArg::Typed(argument)) => argument.ty.as_ref().clone(),
        _ => {
            return Err(syn::Error::new_spanned(&fragment.sig, "fragment must accept a named varying type"));
        }
    };
    let model = PassSchema::parse(&vertex, &fragment)?;
    let pass = pass_declaration(&model, options, pass_type, &shader_module);
    let (vertex_impl, vertex_entry) = vertex_functions(vertex, &model, &varying_fields)?;
    let (fragment_impl, fragment_entry) = fragment_functions(fragment, &model, &varying_fields, &varying_type)?;
    let remaining = &file.items;
    Ok(quote! {
        #pass
        #(#remaining)*
        #[doc(hidden)]
        pub mod #shader_module {
            use super::*;

            #vertex_impl
            #fragment_impl
            #vertex_entry
            #fragment_entry
        }
    })
}

fn pass_declaration(model: &PassSchema, options: &PassOptions, pass_type: &Type, shader_module: &syn::Ident) -> TokenStream2 {
    let isthmus = isthmus_path();
    let data = &model.instance;
    let shared_impl = model.shared.as_ref().map_or_else(
        || {
            quote! {
                impl<T: #isthmus::BufferData> #isthmus::__private::PassShared<T> for #pass_type {
                    const SHARED_BUFFER: bool = false;
                }
            }
        },
        |shared| {
            quote! {
                impl #isthmus::__private::PassShared<#shared> for #pass_type {
                    const SHARED_BUFFER: bool = true;
                }
            }
        },
    );
    let resources_type = if model.resources.is_empty() {
        quote!(())
    } else {
        let kinds = model.resources.iter().map(|(_, kind)| kind);
        quote!((#(&'a #kinds,)*))
    };
    let resource_bindings = if model.resources.is_empty() {
        quote!(std::boxed::Box::new([]))
    } else {
        let names = model.resources.iter().map(|(name, _)| name).collect::<Vec<_>>();
        let binding_names = names.clone();
        quote! {
            let (#(#names,)*) = resources;
            std::boxed::Box::new([#(#isthmus::__private::ResourceBinding::binding(#binding_names)),*])
        }
    };
    let topology = format_ident!("{}", options.topology);
    let blend = match options.blend {
        "Replace" => quote!(#isthmus::wgpu::BlendState::REPLACE),
        "Alpha" => quote!(#isthmus::wgpu::BlendState::ALPHA_BLENDING),
        "PremultipliedAlpha" => {
            quote!(#isthmus::wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING)
        }
        "Add" => quote!(#isthmus::wgpu::BlendState::ADDITIVE),
        _ => unreachable!(),
    };
    let vertices = options.vertices;
    quote! {
        #[cfg(feature = "cpu")]
        impl #isthmus::__private::PassContract for #pass_type {
            type Instance = #data;
            type Resources<'a> = #resources_type;
            const NAME: &'static str = #isthmus::__private::pass_module_name(concat!(
                module_path!(),
                "::",
                stringify!(#shader_module),
            ));
            const PIPELINE: #isthmus::__private::Pipeline = #isthmus::__private::Pipeline {
                topology: #isthmus::wgpu::PrimitiveTopology::#topology,
                blend: #blend,
                vertices: #vertices,
            };

            fn bindings(resources: Self::Resources<'_>) -> std::boxed::Box<[#isthmus::__private::Binding]> {
                #resource_bindings
            }
        }

        #[cfg(feature = "cpu")]
        #shared_impl
    }
}

fn take_function(items: &mut Vec<Item>, name: &str) -> SynResult<ItemFn> {
    let index = items
        .iter()
        .position(|item| matches!(item, Item::Fn(function) if function.sig.ident == name && has_gpu_marker(&function.attrs)))
        .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), format!("missing `{name}` function")))?;
    let Item::Fn(mut function) = items.remove(index) else { unreachable!() };
    take_gpu_marker(&mut function.attrs);
    Ok(function)
}

fn has_gpu_marker(attributes: &[syn::Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.path().is_ident("gpu") && matches!(attribute.meta, syn::Meta::Path(_)))
}

fn take_gpu_marker(attributes: &mut Vec<syn::Attribute>) {
    attributes.retain(|attribute| !(attribute.path().is_ident("gpu") && matches!(attribute.meta, syn::Meta::Path(_))));
}

fn take_gpu_role(attributes: &mut Vec<syn::Attribute>) -> SynResult<Option<GpuRole>> {
    let Some(index) = gpu_attribute(attributes)? else {
        return Ok(None);
    };
    let parameter = attributes.remove(index).parse_args::<GpuParameter>()?;
    if parameter.index.is_some() {
        return Err(syn::Error::new(parameter.kind.span(), "varying fields do not take a value"));
    }
    Ok(Some(parameter.role))
}
