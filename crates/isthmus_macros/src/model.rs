use crate::{
    isthmus_path,
    syntax::{GpuRole, Stage, StageContract},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, Result, Type};

/// Canonical representation shared by host pass generation and shader ABI lowering.
pub struct PassSchema {
    pub vertex: StageContract,
    pub fragment: StageContract,
    pub shared: Option<Type>,
    pub instance: Type,
    pub resources: Vec<ResourceSchema>,
}

pub struct ResourceSchema {
    pub name: Ident,
    pub cpu: Option<TokenStream>,
}

impl PassSchema {
    pub fn parse(vertex: &ItemFn, fragment: &ItemFn, varying_count: usize) -> Result<Self> {
        let vertex = StageContract::parse(vertex, Stage::Vertex, 0)?;
        let fragment_contract = StageContract::parse(fragment, Stage::Fragment, varying_count)?;
        let mut shared = None;
        let mut instance = None;
        let mut resources: Vec<ResourceSchema> = Vec::new();
        for stage in [&vertex, &fragment_contract] {
            let mut resource_index = 0;
            for parameter in &stage.parameters {
                match parameter.role {
                    GpuRole::Shared => merge_type(&mut shared, &parameter.ty, "all shader stages must use the same shared type")?,
                    GpuRole::Instance => merge_type(&mut instance, &parameter.ty, "all shader stages must use the same instance type")?,
                    GpuRole::Resource => {
                        let resource = ResourceSchema {
                            name: parameter.name.clone(),
                            cpu: resource_cpu_type(&parameter.ty)?,
                        };
                        if resources[..resource_index].iter().any(|previous| previous.name == resource.name) {
                            return Err(syn::Error::new_spanned(&parameter.name, "resource names must be unique within a shader stage"));
                        }
                        if let Some(previous) = resources.get(resource_index) {
                            if previous.name != resource.name || !same_optional_tokens(previous.cpu.as_ref(), resource.cpu.as_ref()) {
                                return Err(syn::Error::new_spanned(&parameter.name, "shader stages must declare shared resources in the same order"));
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
        let instance = instance.ok_or_else(|| syn::Error::new_spanned(fragment, "a pass must accept instance data in at least one stage"))?;
        Ok(Self {
            vertex,
            fragment: fragment_contract,
            shared,
            instance,
            resources,
        })
    }

    pub fn carries_instance(&self) -> bool {
        self.fragment.has_implicit(GpuRole::Instance)
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

fn same_optional_tokens(left: Option<&TokenStream>, right: Option<&TokenStream>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => same_tokens(left, right),
        (None, None) => true,
        _ => false,
    }
}

fn resource_cpu_type(ty: &Type) -> Result<Option<TokenStream>> {
    let isthmus = isthmus_path();
    let Type::Reference(reference) = ty else {
        return Err(syn::Error::new_spanned(ty, "GPU resources must be references"));
    };
    let resource = &reference.elem;
    if matches!(resource.as_ref(), Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Sampler")) {
        Ok(None)
    } else {
        Ok(Some(quote!(<#resource as #isthmus::__private::ResourceType>::Binding)))
    }
}
