use crate::{
    isthmus_path,
    syntax::{GpuRole, Stage, StageContract},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, Result, Type};

pub struct PassModel {
    pub vertex: StageContract,
    pub fragment: StageContract,
    pub shared: Option<Type>,
    pub instance: Type,
    pub resources: Vec<(Ident, TokenStream)>,
    pub carry_instance: bool,
}

impl PassModel {
    pub fn parse(vertex: &ItemFn, fragment: &ItemFn) -> Result<Self> {
        let vertex = StageContract::parse(vertex, Stage::Vertex)?;
        let fragment_contract = StageContract::parse(fragment, Stage::Fragment)?;
        let carry_instance = fragment_contract.has_implicit(GpuRole::Instance);
        if carry_instance
            && vertex.parameters.iter().any(|parameter| {
                parameter.role.role == GpuRole::Instance && parameter.role.index.is_some()
            })
        {
            return Err(syn::Error::new_spanned(
                fragment,
                "fragment instance needs an explicit index when the vertex remaps instances",
            ));
        }

        let mut shared = None;
        let mut instance = None;
        let mut resources: Vec<(Ident, TokenStream)> = Vec::new();
        for stage in [&vertex, &fragment_contract] {
            let mut resource_index = 0;
            for parameter in &stage.parameters {
                match parameter.role.role {
                    GpuRole::Shared => merge_type(
                        &mut shared,
                        &parameter.ty,
                        "all shader stages must use the same shared type",
                    )?,
                    GpuRole::Instance => merge_type(
                        &mut instance,
                        &parameter.ty,
                        "all shader stages must use the same instance type",
                    )?,
                    GpuRole::Resource => {
                        let resource = (parameter.name.clone(), resource_cpu_type(&parameter.ty)?);
                        if resources[..resource_index]
                            .iter()
                            .any(|previous| previous.0 == resource.0)
                        {
                            return Err(syn::Error::new_spanned(
                                &parameter.name,
                                "resource names must be unique within a shader stage",
                            ));
                        }
                        if let Some(previous) = resources.get(resource_index) {
                            if previous.0 != resource.0 || !same_tokens(&previous.1, &resource.1) {
                                return Err(syn::Error::new_spanned(
                                    &parameter.name,
                                    "shader stages must declare shared resources in the same order",
                                ));
                            }
                        } else {
                            resources.push(resource);
                        }
                        resource_index += 1;
                    }
                    _ => {}
                }
            }
        }
        let instance = instance.ok_or_else(|| {
            syn::Error::new_spanned(
                fragment,
                "a pass must declare `#[gpu(instance)]` data in at least one stage",
            )
        })?;
        Ok(Self {
            vertex,
            fragment: fragment_contract,
            shared,
            instance,
            resources,
            carry_instance,
        })
    }
}

fn merge_type(target: &mut Option<Type>, ty: &Type, message: &str) -> Result<()> {
    if let Some(previous) = target {
        if !same_tokens(previous, ty) {
            return Err(syn::Error::new_spanned(ty, message));
        }
    } else {
        *target = Some(ty.clone());
    }
    Ok(())
}

fn same_tokens(left: &impl quote::ToTokens, right: &impl quote::ToTokens) -> bool {
    quote!(#left).to_string() == quote!(#right).to_string()
}

fn resource_cpu_type(ty: &Type) -> Result<TokenStream> {
    let isthmus = isthmus_path();
    let Type::Reference(reference) = ty else {
        return Err(syn::Error::new_spanned(ty, "GPU resources must be references"));
    };
    if let Type::Slice(slice) = reference.elem.as_ref() {
        let element = &slice.elem;
        return Ok(quote!(#isthmus::Storage<#element>));
    }
    let Type::Path(path) = reference.elem.as_ref() else {
        return Err(syn::Error::new_spanned(ty, "unsupported GPU resource type"));
    };
    let name = &path.path.segments.last().unwrap().ident;
    if name == "Sampler" {
        Ok(quote!(#isthmus::FilteringSampler))
    } else if name == "Image2dArray" {
        Ok(quote!(#isthmus::TextureView<#isthmus::Texture2DArray>))
    } else {
        Err(syn::Error::new_spanned(ty, "unsupported GPU resource type"))
    }
}
