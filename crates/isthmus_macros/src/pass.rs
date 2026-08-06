use crate::{
    abi::{fragment_functions, vertex_functions},
    isthmus_path,
    model::PassModel,
    syntax::{GpuParameter, GpuRole, PassOptions, gpu_attribute},
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Fields, File, FnArg, Ident, ImplItem, Item, ItemFn, ItemImpl, Result as SynResult, Type,
    parse_macro_input, parse_quote,
};

#[allow(clippy::missing_panics_doc)]
pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let option_tokens = TokenStream2::from(args.clone());
    let item = parse_macro_input!(input as Item);
    let result = match item {
        Item::Impl(implementation) => syn::parse::<PassOptions>(args)
            .and_then(|_| expand_pass_impl(implementation, &option_tokens)),
        Item::Struct(mut pass) => {
            if option_tokens.is_empty() {
                let isthmus = isthmus_path();
                let pass_fields = match &pass.fields {
                    Fields::Named(fields) => fields
                        .named
                        .iter()
                        .filter_map(|field| {
                            let Type::Path(path) = &field.ty else {
                                return None;
                            };
                            path.path
                                .segments
                                .last()
                                .map(|segment| segment.ident.to_string())
                                .filter(|name| name == "Pass" || name == "StatePass")
                                .map(|_| field.ident.as_ref().unwrap())
                        })
                        .collect::<Vec<_>>(),
                    _ => Vec::new(),
                };
                if pass_fields.len() > 1 {
                    return syn::Error::new_spanned(
                        &pass.fields,
                        "a pass can own at most one runtime pass; use a system to coordinate multiple passes",
                    )
                    .to_compile_error()
                    .into();
                }
                let name = &pass.ident;
                let (impl_generics, type_generics, where_clause) = pass.generics.split_for_impl();
                let visibility = &pass.vis;
                pass.attrs.push(parse_quote!(#[cfg(feature = "cpu")]));
                let runtime_impls = (!pass_fields.is_empty()).then(|| {
                    quote! {
                        #[cfg(feature = "cpu")]
                        impl #impl_generics #isthmus::Render for #name #type_generics #where_clause {
                            fn prepare(&mut self) {
                                #(#isthmus::Render::prepare(&mut self.#pass_fields);)*
                            }

                            fn draw<'pass>(
                                &'pass self,
                                pass: &mut #isthmus::wgpu::RenderPass<'pass>,
                            ) {
                                #(#isthmus::Render::draw(&self.#pass_fields, pass);)*
                            }
                        }
                    }
                });
                Ok(quote! {
                    #pass
                    #[cfg(not(feature = "cpu"))]
                    #visibility struct #name #impl_generics #where_clause;
                    #runtime_impls
                })
            } else {
                Err(syn::Error::new_spanned(
                    option_tokens,
                    "pass structs do not take pipeline options",
                ))
            }
        }
        item => Err(syn::Error::new_spanned(
            item,
            "pass requires a struct or impl block",
        )),
    };
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}

fn expand_pass_impl(mut implementation: ItemImpl, options: &TokenStream2) -> SynResult<TokenStream2> {
    let fragment = implementation
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Fn(function)
                if function.sig.ident == "fragment" && has_gpu_marker(&function.attrs) =>
            {
                Some(function)
            }
            _ => None,
        })
        .ok_or_else(|| syn::Error::new_spanned(&implementation, "missing `fragment` function"))?;
    let varying = fragment
        .sig
        .inputs
        .first()
        .and_then(|argument| match argument {
            FnArg::Typed(argument) => match argument.ty.as_ref() {
                Type::Path(path) => path.path.segments.last().map(|segment| segment.ident.clone()),
                _ => None,
            },
            FnArg::Receiver(_) => None,
        })
        .ok_or_else(|| {
            syn::Error::new_spanned(&fragment.sig, "fragment must accept a named varying type")
        })?;
    let bridge = format_ident!("__isthmus_varyings_{varying}");
    implementation
        .attrs
        .push(parse_quote!(#[isthmus_pass_options(#options)]));
    Ok(quote!(#bridge! { #implementation }))
}

#[doc(hidden)]
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
    let options_index = implementation
        .attrs
        .iter()
        .position(|attribute| attribute.path().is_ident("isthmus_pass_options"))
        .unwrap();
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
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "shader requires exactly one `#[gpu]` varying struct",
        ));
    };
    let Item::Struct(varyings) = &mut file.items[*varying_index] else {
        unreachable!()
    };
    let varying_name = varyings.ident.clone();
    take_gpu_marker(&mut varyings.attrs);
    let Fields::Named(fields) = &mut varyings.fields else {
        return Err(syn::Error::new_spanned(
            varyings,
            "Varyings requires named fields",
        ));
    };
    let varying_fields = fields
        .named
        .iter_mut()
        .map(|field| {
            let flat = match take_gpu_role(&mut field.attrs)? {
                Some(GpuRole::Flat) => true,
                Some(_) => {
                    return Err(syn::Error::new_spanned(
                        field,
                        "varying fields only support `#[gpu(flat)]`",
                    ));
                }
                None => false,
            };
            Ok((field.ident.clone().unwrap(), field.ty.clone(), flat))
        })
        .collect::<SynResult<Vec<_>>>()?;
    file.items.remove(*varying_index);

    let vertex = take_function(&mut file.items, "vertex")?;
    let fragment = take_function(&mut file.items, "fragment")?;
    let model = PassModel::parse(&vertex, &fragment)?;
    let names = PassNames::from_varyings(&varying_name);
    let pass = pass_declaration(&model, options, &names, pass_type);
    let (vertex_impl, vertex_entry) =
        vertex_functions(vertex, &model.vertex, &varying_fields, model.carry_instance)?;
    let (fragment_impl, fragment_entry) = fragment_functions(
        fragment,
        &model.fragment,
        &varying_fields,
        &varying_name,
        model.carry_instance,
    )?;
    let remaining = &file.items;
    let entry_functions = names.module.as_ref().map_or_else(
        || {
            quote! {
                #vertex_impl
                #fragment_impl
                #vertex_entry
                #fragment_entry
            }
        },
        |module| {
            quote! {
                #[doc(hidden)]
                pub mod #module {
                    use super::*;
                    #vertex_impl
                    #fragment_impl
                    #vertex_entry
                    #fragment_entry
                }
            }
        },
    );
    Ok(quote! {
        #pass
        #(#remaining)*
        #entry_functions
    })
}

struct PassNames {
    resources: Ident,
    module: Option<Ident>,
}

impl PassNames {
    fn from_varyings(varyings: &Ident) -> Self {
        let name = varyings.to_string();
        let prefix = name.strip_suffix("Varyings").unwrap_or(&name);
        if prefix.is_empty() {
            Self {
                resources: format_ident!("Resources"),
                module: None,
            }
        } else {
            Self {
                resources: format_ident!("{prefix}Resources"),
                module: Some(format_ident!("{}", snake_case(prefix))),
            }
        }
    }
}

fn snake_case(name: &str) -> String {
    let mut output = String::new();
    for (index, character) in name.chars().enumerate() {
        if index > 0 && character.is_uppercase() {
            output.push('_');
        }
        output.extend(character.to_lowercase());
    }
    output
}

fn pass_declaration(
    model: &PassModel,
    options: &PassOptions,
    pass_names: &PassNames,
    pass_type: &Type,
) -> TokenStream2 {
    let isthmus = isthmus_path();
    let resources_type_name = &pass_names.resources;
    let instance_buffer = model.instance.is_some();
    let data = model.instance.clone().unwrap_or_else(|| parse_quote!(()));
    let shared_impl = model.shared.as_ref().map_or_else(
        || {
            quote! {
                impl<T: #isthmus::BufferData> #isthmus::PassShared<T> for #pass_type {
                    const SHARED_BUFFER: bool = false;
                }
            }
        },
        |shared| {
            quote! {
                impl #isthmus::PassShared<#shared> for #pass_type {
                    const SHARED_BUFFER: bool = true;
                }
            }
        },
    );
    let resources_type = if model.resources.is_empty() {
        quote!(())
    } else {
        quote!(#resources_type_name<'a>)
    };
    let resources_declaration = if model.resources.is_empty() {
        quote!()
    } else {
        let (names, kinds): (Vec<_>, Vec<_>) =
            model.resources.iter().map(|(name, kind)| (name, kind)).unzip();
        quote! {
            #[cfg(feature = "cpu")]
            pub struct #resources_type_name<'a> {
                #(pub #names: &'a #kinds),*
            }

            #[cfg(feature = "cpu")]
            impl #isthmus::ResourceBindings for #resources_type_name<'_> {
                fn into_bindings(self) -> std::vec::Vec<#isthmus::Binding> {
                    std::vec![#(<#kinds as #isthmus::CpuResource>::clone_binding(self.#names)),*]
                }
            }
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
    let pass_name = pass_names.module.as_ref().map_or_else(
        || quote!(#isthmus::pass_module_name(module_path!())),
        |module| {
            quote!(#isthmus::pass_module_name(concat!(
                module_path!(),
                "::",
                stringify!(#module)
            )))
        },
    );
    quote! {
        #[cfg(feature = "cpu")]
        impl #isthmus::PassContract for #pass_type {
            type Instance = #data;
            type Resources<'a> = #resources_type;
            const NAME: &'static str = #pass_name;
            const INSTANCE_BUFFER: bool = #instance_buffer;
            const PIPELINE: #isthmus::Pipeline = #isthmus::Pipeline {
                topology: #isthmus::wgpu::PrimitiveTopology::#topology,
                blend: #blend,
                vertices: #vertices,
            };
        }

        #[cfg(feature = "cpu")]
        #shared_impl
        #resources_declaration
    }
}

fn take_function(items: &mut Vec<Item>, name: &str) -> SynResult<ItemFn> {
    let index = items
        .iter()
        .position(|item| {
            matches!(item, Item::Fn(function) if function.sig.ident == name && has_gpu_marker(&function.attrs))
        })
        .ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("missing `{name}` function"),
            )
        })?;
    let Item::Fn(mut function) = items.remove(index) else {
        unreachable!()
    };
    take_gpu_marker(&mut function.attrs);
    Ok(function)
}

fn has_gpu_marker(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("gpu") && matches!(attribute.meta, syn::Meta::Path(_))
    })
}

fn take_gpu_marker(attributes: &mut Vec<syn::Attribute>) {
    attributes.retain(|attribute| {
        !(attribute.path().is_ident("gpu") && matches!(attribute.meta, syn::Meta::Path(_)))
    });
}

fn take_gpu_role(attributes: &mut Vec<syn::Attribute>) -> SynResult<Option<GpuRole>> {
    let Some(index) = gpu_attribute(attributes)? else {
        return Ok(None);
    };
    let parameter = attributes.remove(index).parse_args::<GpuParameter>()?;
    if parameter.index.is_some() {
        return Err(syn::Error::new(
            parameter.kind.span(),
            "varying fields do not take a value",
        ));
    }
    Ok(Some(parameter.role))
}
