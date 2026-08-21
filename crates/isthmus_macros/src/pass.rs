use crate::{
    abi::{fragment_functions, vertex_functions},
    isthmus_path,
    model::PassSchema,
    render,
    syntax::PassOptions,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Fields, FnArg, GenericArgument, Item, ItemFn, ItemMod, PathArguments, Result as SynResult, Type, parse_macro_input, parse_quote};

pub fn expand_shader(args: TokenStream, input: TokenStream) -> TokenStream {
    let options = parse_macro_input!(args as PassOptions);
    let mut module = parse_macro_input!(input as ItemMod);
    match expand_shader_module(&mut module, &options) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_shader_module(module: &mut ItemMod, options: &PassOptions) -> SynResult<TokenStream2> {
    let Some((_, items)) = &mut module.content else {
        return Err(syn::Error::new_spanned(module, "shader requires an inline module"));
    };
    let vertex = take_function(items, "vertex")?;
    let fragment = take_function(items, "fragment")?;
    let varying_fields = varying_fields(&vertex, &fragment)?;
    let model = PassSchema::parse(&vertex, &fragment, varying_fields.len())?;
    let isthmus = isthmus_path();
    let varying_names = varying_fields.iter().map(|(name, _)| name);
    let varying_types = varying_fields.iter().map(|(_, ty)| ty);
    let pipeline: Type = parse_quote!(Pipeline);
    let shader_module = module.ident.clone();
    let pass = pass_declaration(&model, options, &pipeline, &shader_module, true);
    let (vertex_impl, vertex_entry) = vertex_functions(vertex, &model, &varying_fields)?;
    let (fragment_impl, fragment_entry) = fragment_functions(fragment, &model, &varying_fields)?;
    let attrs = &module.attrs;
    let visibility = &module.vis;
    let name = &module.ident;
    let remaining = &*items;
    Ok(quote! {
        #(#attrs)*
        #visibility mod #name {
            use super::*;
            #[doc(hidden)]
            pub struct Raster {
                pub position: #isthmus::glam::Vec4,
                #(pub #varying_names: #varying_types,)*
            }
            #[cfg(not(target_arch = "spirv"))]
            pub struct Pipeline;
            #pass
            #(#remaining)*
            #vertex_impl
            #fragment_impl
            #vertex_entry
            #fragment_entry
        }
    })
}

fn varying_fields(vertex: &ItemFn, fragment: &ItemFn) -> SynResult<Vec<(syn::Ident, Type)>> {
    use crate::syntax::{GpuRole, Stage, StageContract};
    let vertex = StageContract::parse(vertex, Stage::Vertex, 0)?;
    let data_types = vertex
        .parameters
        .iter()
        .filter(|parameter| matches!(parameter.role, GpuRole::Shared | GpuRole::Instance))
        .map(|parameter| &parameter.ty)
        .collect::<Vec<_>>();
    let mut fields = Vec::new();
    for argument in &fragment.sig.inputs {
        let FnArg::Typed(argument) = argument else {
            return Err(syn::Error::new_spanned(argument, "shader functions cannot have self"));
        };
        let syn::Pat::Ident(pattern) = argument.pat.as_ref() else {
            return Err(syn::Error::new_spanned(&argument.pat, "shader parameters require an identifier"));
        };
        let resource = matches!(argument.ty.as_ref(), Type::Reference(_));
        let argument_ty = &argument.ty;
        let data = data_types.iter().any(|ty| quote::quote!(#ty).to_string() == quote::quote!(#argument_ty).to_string());
        if resource || data || pattern.ident == "frame" {
            break;
        }
        fields.push((pattern.ident.clone(), (*argument.ty).clone()));
    }
    if fields.is_empty() {
        return Err(syn::Error::new_spanned(&fragment.sig, "fragment must begin with at least one raster value"));
    }
    Ok(fields)
}

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let option_tokens = TokenStream2::from(args);
    let item = parse_macro_input!(input as Item);
    let result = match item {
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
                pass.attrs.push(parse_quote!(#[cfg(not(target_arch = "spirv"))]));
                let runtime_impl = (!pass_fields.is_empty()).then(|| {
                    let implementation = render::implementation(name, &pass.generics, pass_fields);
                    quote!(#[cfg(not(target_arch = "spirv"))] #implementation)
                });
                Ok(quote! {
                    #pass
                    #runtime_impl
                })
            } else {
                Err(syn::Error::new_spanned(option_tokens, "systems do not take pipeline options"))
            }
        }
        item => Err(syn::Error::new_spanned(item, "system requires a struct")),
    };
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}

fn pass_declaration(model: &PassSchema, options: &PassOptions, pass_type: &Type, shader_module: &syn::Ident, module_scoped: bool) -> TokenStream2 {
    let isthmus = isthmus_path();
    let data = &model.instance;
    let resources_name = match pass_type {
        Type::Path(path) => format_ident!("{}Resources", path.path.segments.last().unwrap().ident),
        _ => unreachable!(),
    };
    let shared_impl = model.shared.as_ref().map_or_else(
        || quote!(impl<T: #isthmus::ShaderData> #isthmus::__private::PassShared<T> for #pass_type {}),
        |shared| quote!(impl #isthmus::__private::PassShared<#shared> for #pass_type {}),
    );
    let has_host_resources = model.resources.iter().any(|resource| resource.cpu.is_some());
    let resource_declaration = if has_host_resources {
        let resources = model.resources.iter().filter_map(|resource| resource.cpu.as_ref().map(|kind| (&resource.name, kind)));
        let (names, kinds): (Vec<_>, Vec<_>) = resources.unzip();
        quote! {
            #[cfg(not(target_arch = "spirv"))]
            pub struct #resources_name<'a> {
                #(pub #names: &'a #kinds,)*
            }
        }
    } else {
        quote!()
    };
    let resources_type = if has_host_resources { quote!(#resources_name<'a>) } else { quote!(()) };
    let resources_argument = if has_host_resources { quote!(#resources_name<'_>) } else { quote!(()) };
    let resource_bindings = if model.resources.is_empty() {
        quote!(std::boxed::Box::new([]))
    } else {
        let bindings = model.resources.iter().map(|resource| {
            let name = &resource.name;
            if resource.cpu.is_some() {
                quote!(#isthmus::__private::ResourceBinding::binding(resources.#name))
            } else {
                quote!(#isthmus::__private::Binding::Sampler)
            }
        });
        quote! {
            std::boxed::Box::new([#(#bindings),*])
        }
    };
    let topology = match options.topology {
        "PointList" => format_ident!("POINT_LIST"),
        "LineList" => format_ident!("LINE_LIST"),
        "LineStrip" => format_ident!("LINE_STRIP"),
        "TriangleList" => format_ident!("TRIANGLE_LIST"),
        "TriangleStrip" => format_ident!("TRIANGLE_STRIP"),
        _ => unreachable!(),
    };
    let blend = match options.blend {
        "Replace" => quote!(#isthmus::__private::Blend::Replace),
        "Alpha" => quote!(#isthmus::__private::Blend::Alpha),
        "PremultipliedAlpha" => {
            quote!(#isthmus::__private::Blend::PremultipliedAlpha)
        }
        "Add" => quote!(#isthmus::__private::Blend::Add),
        _ => unreachable!(),
    };
    let vertices = options.vertices;
    let name = if module_scoped {
        quote!(#isthmus::__private::pass_module_name(module_path!()))
    } else {
        quote!(#isthmus::__private::pass_module_name(concat!(module_path!(), "::", stringify!(#shader_module))))
    };
    quote! {
        #resource_declaration

        #[cfg(not(target_arch = "spirv"))]
        impl #isthmus::__private::PassContract for #pass_type {
            type Instance = #data;
            type Resources<'a> = #resources_type;
            const NAME: &'static str = #name;
            const PIPELINE: #isthmus::__private::Pipeline = #isthmus::__private::Pipeline {
                topology: #isthmus::ash::vk::PrimitiveTopology::#topology,
                blend: #blend,
                vertices: #vertices,
            };

            fn bindings(resources: Self::Resources<'_>) -> std::boxed::Box<[#isthmus::__private::Binding]> {
                #resource_bindings
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        #shared_impl

        #[cfg(not(target_arch = "spirv"))]
        pub fn instances<Shared: #isthmus::ShaderData>(
            passes: &#isthmus::PassBuilder<'_, Shared>,
            resources: #resources_argument,
            values: impl IntoIterator<Item = #data>,
        ) -> #isthmus::Instances<#data>
        where
            #pass_type: #isthmus::__private::PassShared<Shared>,
        {
            passes.instances::<#pass_type>(resources, values)
        }

        #[cfg(not(target_arch = "spirv"))]
        pub fn instances_with_capacity<Shared: #isthmus::ShaderData>(
            passes: &#isthmus::PassBuilder<'_, Shared>,
            resources: #resources_argument,
            capacity: usize,
        ) -> #isthmus::Instances<#data>
        where
            #pass_type: #isthmus::__private::PassShared<Shared>,
        {
            passes.instances_with_capacity::<#pass_type>(resources, capacity)
        }

        #[cfg(not(target_arch = "spirv"))]
        pub fn instance<Shared: #isthmus::ShaderData>(
            passes: &#isthmus::PassBuilder<'_, Shared>,
            resources: #resources_argument,
            value: #data,
        ) -> #isthmus::Instance<#data>
        where
            #pass_type: #isthmus::__private::PassShared<Shared>,
        {
            passes.instance::<#pass_type>(resources, value)
        }
    }
}

fn take_function(items: &mut Vec<Item>, name: &str) -> SynResult<ItemFn> {
    let index = items
        .iter()
        .position(|item| matches!(item, Item::Fn(function) if function.sig.ident == name))
        .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), format!("missing `{name}` function")))?;
    let Item::Fn(mut function) = items.remove(index) else { unreachable!() };
    take_gpu_marker(&mut function.attrs);
    Ok(function)
}

fn take_gpu_marker(attributes: &mut Vec<syn::Attribute>) {
    attributes.retain(|attribute| !(attribute.path().is_ident("gpu") && matches!(attribute.meta, syn::Meta::Path(_))));
}
