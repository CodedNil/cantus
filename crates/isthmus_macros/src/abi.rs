use crate::{
    isthmus_path,
    syntax::{GpuRole, StageContract, StageParameter},
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Expr, FnArg, Ident, ItemFn, Pat, Result as SynResult, Stmt, Type, parse_quote, visit_mut::VisitMut,
};

fn lower_parameters<'a>(
    inputs: &mut [FnArg],
    parameters: impl IntoIterator<Item = (usize, &'a StageParameter)>,
    default_instance: &Expr,
    varying_fields: Option<&[(Ident, Type, bool)]>,
) -> Vec<Stmt> {
    let isthmus = isthmus_path();
    let mut initializers = Vec::new();
    for (argument_index, parameter) in parameters {
        let FnArg::Typed(argument) = &mut inputs[argument_index] else {
            unreachable!()
        };
        argument.attrs.remove(parameter.attribute);
        let role = &parameter.role;
        if matches!(role.role, GpuRole::VertexIndex | GpuRole::InstanceIndex) {
            let kind = &role.kind;
            argument.attrs.push(parse_quote!(#[spirv(#kind)]));
            continue;
        }
        if role.role == GpuRole::Resource {
            let binding = parameter.binding.unwrap();
            let storage = matches!(
                argument.ty.as_ref(),
                Type::Reference(reference) if matches!(reference.elem.as_ref(), Type::Slice(_))
            );
            argument.attrs.push(if storage {
                parse_quote!(#[spirv(storage_buffer, descriptor_set = 0, binding = #binding)])
            } else {
                parse_quote!(#[spirv(descriptor_set = 0, binding = #binding)])
            });
            continue;
        }
        let name = &parameter.name;
        let ty = &parameter.ty;
        let mut index: Expr = if role.role == GpuRole::Shared {
            parse_quote!(0usize)
        } else if role.role == GpuRole::Instance {
            role.index.clone().unwrap_or_else(|| default_instance.clone())
        } else {
            unreachable!()
        };
        let binding = parameter.binding.unwrap();
        if let Some(fields) = varying_fields {
            align_varying_hygiene(&mut index, fields);
        }
        argument.attrs.push(parse_quote!(
            #[spirv(storage_buffer, descriptor_set = 0, binding = #binding)]
        ));
        *argument.ty = Type::Verbatim(quote!(&[#ty]));
        initializers.push(parse_quote!(
            let #name = #isthmus::__private::reference(#name, #index);
        ));
    }
    initializers
}

fn borrow_data_parameters(function: &mut ItemFn, contract: &StageContract) {
    for parameter in &contract.parameters {
        if matches!(parameter.role.role, GpuRole::Shared | GpuRole::Instance) {
            let FnArg::Typed(argument) = &mut function.sig.inputs[parameter.argument] else {
                unreachable!()
            };
            let ty = &parameter.ty;
            *argument.ty = parse_quote!(&#ty);
        }
    }
}

fn argument_names<'a>(inputs: impl IntoIterator<Item = &'a FnArg>) -> SynResult<Vec<Pat>> {
    inputs
        .into_iter()
        .map(|argument| match argument {
            FnArg::Typed(argument) => Ok((*argument.pat).clone()),
            FnArg::Receiver(receiver) => Err(syn::Error::new_spanned(
                receiver,
                "shader functions cannot have self",
            )),
        })
        .collect()
}

fn strip_gpu_attributes(function: &mut ItemFn) {
    for argument in &mut function.sig.inputs {
        if let FnArg::Typed(argument) = argument {
            argument
                .attrs
                .retain(|attribute| !attribute.path().is_ident("gpu"));
        }
    }
}

fn align_varying_hygiene(expression: &mut Expr, fields: &[(Ident, Type, bool)]) {
    struct VaryingNames<'a>(&'a [(Ident, Type, bool)]);

    impl VisitMut for VaryingNames<'_> {
        fn visit_ident_mut(&mut self, i: &mut Ident) {
            if let Some((field, _, _)) = self.0.iter().find(|(field, _, _)| *field == *i) {
                *i = field.clone();
            }
        }
    }

    VaryingNames(fields).visit_expr_mut(expression);
}

pub fn vertex_functions(
    mut function: ItemFn,
    contract: &StageContract,
    fields: &[(Ident, Type, bool)],
    carry_instance: bool,
) -> SynResult<(ItemFn, TokenStream2)> {
    let arguments = argument_names(&function.sig.inputs)?;
    let mut wrapper_inputs = function.sig.inputs.iter().cloned().collect::<Vec<_>>();
    let implicit_instance = contract.has_implicit(GpuRole::Instance);
    let mut instance_index = contract
        .parameters
        .iter()
        .find(|parameter| parameter.role.role == GpuRole::InstanceIndex)
        .map(|parameter| parameter.name.clone());
    if (implicit_instance || carry_instance) && instance_index.is_none() {
        wrapper_inputs.push(parse_quote!(
            #[spirv(instance_index)] __isthmus_instance_index: u32
        ));
        instance_index = Some(format_ident!("__isthmus_instance_index"));
    }
    let default_instance = instance_index
        .as_ref()
        .map_or_else(|| parse_quote!(0usize), |index| parse_quote!(#index as usize));
    let initializers = lower_parameters(
        &mut wrapper_inputs,
        contract
            .parameters
            .iter()
            .map(|parameter| (parameter.argument, parameter)),
        &default_instance,
        None,
    );
    function.sig.ident = format_ident!("vertex_impl");
    borrow_data_parameters(&mut function, contract);
    strip_gpu_attributes(&mut function);
    let output_parameters = fields.iter().enumerate().map(|(location, (name, ty, flat))| {
        let output = format_ident!("out_{name}");
        if *flat {
            quote!(#[spirv(location = #location, flat)] #output: &mut #ty)
        } else {
            quote!(#[spirv(location = #location)] #output: &mut #ty)
        }
    });
    let assignments = fields.iter().map(|(name, _, _)| {
        let output = format_ident!("out_{name}");
        quote!(*#output = output.varyings.#name;)
    });
    let instance_location = fields.len();
    let instance_output = carry_instance.then(|| {
        quote!(
            #[spirv(location = #instance_location, flat)]
            out_isthmus_instance_index: &mut u32,
        )
    });
    let instance_assignment = if carry_instance {
        let instance_index = instance_index.as_ref().ok_or_else(|| {
            syn::Error::new_spanned(&function.sig, "instance varying requires an instance index")
        })?;
        Some(quote!(*out_isthmus_instance_index = #instance_index;))
    } else {
        None
    };
    let isthmus = isthmus_path();
    let wrapper = quote! {
        #[#isthmus::spirv_std::spirv(vertex)]
        pub fn vertex(
            #(#wrapper_inputs,)*
            #[spirv(position)] out_position: &mut #isthmus::glam::Vec4,
            #(#output_parameters,)*
            #instance_output
        ) {
            #(#initializers)*
            let output = vertex_impl(#(#arguments),*);
            *out_position = output.position;
            #(#assignments)*
            #instance_assignment
        }
    };
    Ok((function, wrapper))
}

pub fn fragment_functions(
    mut function: ItemFn,
    contract: &StageContract,
    fields: &[(Ident, Type, bool)],
    varying: &Type,
    carry_instance: bool,
) -> SynResult<(ItemFn, TokenStream2)> {
    let first = function
        .sig
        .inputs
        .first()
        .ok_or_else(|| syn::Error::new_spanned(&function.sig, "fragment must accept Varyings"))?;
    let FnArg::Typed(_) = first else {
        return Err(syn::Error::new_spanned(
            first,
            "shader functions cannot have self",
        ));
    };
    let mut other_inputs = function.sig.inputs.iter().skip(1).cloned().collect::<Vec<_>>();
    let default_instance = if carry_instance {
        parse_quote!(__isthmus_instance_index as usize)
    } else {
        parse_quote!(0usize)
    };
    let initializers = lower_parameters(
        &mut other_inputs,
        contract
            .parameters
            .iter()
            .filter(|parameter| parameter.argument != 0)
            .map(|parameter| (parameter.argument - 1, parameter)),
        &default_instance,
        Some(fields),
    );
    let other_names = argument_names(&other_inputs)?;
    function.sig.ident = format_ident!("fragment_impl");
    borrow_data_parameters(&mut function, contract);
    // Stage inputs are values in the shader ABI, even when the Rust body only reads them.
    function
        .attrs
        .push(parse_quote!(#[cfg_attr(feature = "cpu", allow(clippy::needless_pass_by_value))]));
    strip_gpu_attributes(&mut function);
    let varying_parameters = fields.iter().enumerate().map(|(location, (name, ty, flat))| {
        if *flat {
            quote!(#[spirv(location = #location, flat)] #name: #ty)
        } else {
            quote!(#[spirv(location = #location)] #name: #ty)
        }
    });
    let names = fields.iter().map(|(name, _, _)| name);
    let instance_location = fields.len();
    let instance_input = carry_instance.then(|| {
        quote!(
            #[spirv(location = #instance_location, flat)]
            __isthmus_instance_index: u32,
        )
    });
    let isthmus = isthmus_path();
    let wrapper = quote! {
        #[#isthmus::spirv_std::spirv(fragment)]
        pub fn fragment(
            #(#varying_parameters,)*
            #instance_input
            #(#other_inputs,)*
            #[spirv(location = 0)] out_color: &mut #isthmus::glam::Vec4
        ) {
            #(#initializers)*
            *out_color = fragment_impl(#varying { #(#names),* }, #(#other_names),*);
        }
    };
    Ok((function, wrapper))
}
