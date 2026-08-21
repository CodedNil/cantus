use syn::{
    FnArg, Ident, ItemFn, Pat, Result as SynResult, Token, Type,
    parse::{Parse, ParseStream},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GpuRole {
    VertexIndex,
    InstanceIndex,
    Shared,
    Instance,
    Resource,
}

#[derive(Clone, Copy)]
pub enum Stage {
    Vertex,
    Fragment,
}

pub struct PassOptions {
    pub topology: &'static str,
    pub blend: &'static str,
    pub vertices: u32,
}

impl Default for PassOptions {
    fn default() -> Self {
        Self {
            topology: "TriangleStrip",
            blend: "PremultipliedAlpha",
            vertices: 4,
        }
    }
}

pub struct StageParameter {
    pub argument: usize,
    pub binding: Option<u32>,
    pub name: Ident,
    pub ty: Type,
    pub role: GpuRole,
}

pub struct StageContract {
    pub parameters: Vec<StageParameter>,
}

impl StageContract {
    pub fn parse(function: &ItemFn, stage: Stage, varying_count: usize) -> SynResult<Self> {
        let mut parameters = Vec::new();
        let mut resource_binding = 1;
        for (argument_index, argument) in function.sig.inputs.iter().enumerate() {
            let FnArg::Typed(argument) = argument else {
                return Err(syn::Error::new_spanned(argument, "shader functions cannot have self"));
            };
            let is_varyings = matches!(stage, Stage::Fragment) && argument_index < varying_count;
            if is_varyings {
                continue;
            }
            let role = infer_role(argument_index, argument, stage)?;
            if matches!(role, GpuRole::VertexIndex | GpuRole::InstanceIndex) && !is_named_type(&argument.ty, "u32") {
                return Err(syn::Error::new_spanned(&argument.ty, "GPU index parameters must use `u32`"));
            }
            let Pat::Ident(pattern) = argument.pat.as_ref() else {
                return Err(syn::Error::new_spanned(&argument.pat, "GPU parameters require an identifier"));
            };
            if role_is_unique(role) && parameters.iter().any(|parameter: &StageParameter| parameter.role == role) {
                return Err(syn::Error::new_spanned(argument, "a shader stage can only declare this GPU role once"));
            }
            let binding = match role {
                GpuRole::Instance => Some(0),
                GpuRole::Resource => {
                    let binding = resource_binding;
                    resource_binding += 1;
                    Some(binding)
                }
                _ => None,
            };
            parameters.push(StageParameter {
                argument: argument_index,
                binding,
                name: pattern.ident.clone(),
                ty: (*argument.ty).clone(),
                role,
            });
        }
        Ok(Self { parameters })
    }

    pub fn has_implicit(&self, role: GpuRole) -> bool {
        self.parameters.iter().any(|parameter| parameter.role == role)
    }
}

fn infer_role(argument_index: usize, argument: &syn::PatType, stage: Stage) -> SynResult<GpuRole> {
    let Pat::Ident(pattern) = argument.pat.as_ref() else {
        return Err(syn::Error::new_spanned(&argument.pat, "GPU parameters require an identifier"));
    };
    let name = pattern.ident.to_string();
    let role = if matches!(stage, Stage::Vertex) && argument_index == 0 && is_named_type(&argument.ty, "u32") {
        GpuRole::VertexIndex
    } else if matches!(stage, Stage::Vertex) && name == "instance" && is_named_type(&argument.ty, "u32") {
        GpuRole::InstanceIndex
    } else if name == "frame" {
        GpuRole::Shared
    } else if matches!(argument.ty.as_ref(), Type::Reference(_)) {
        GpuRole::Resource
    } else {
        GpuRole::Instance
    };
    Ok(role)
}

fn is_named_type(ty: &Type, name: &str) -> bool {
    matches!(
        ty,
        Type::Path(path) if path.qself.is_none()
            && path.path.segments.last().is_some_and(|segment| segment.ident == name)
    )
}

const fn role_is_unique(role: GpuRole) -> bool {
    !matches!(role, GpuRole::Resource)
}

impl Parse for PassOptions {
    fn parse(input: ParseStream<'_>) -> SynResult<Self> {
        let mut options = Self::default();
        let mut seen = Vec::<Ident>::new();
        while !input.is_empty() {
            let key = input.parse::<Ident>()?;
            if seen.iter().any(|option| option == &key) {
                return Err(syn::Error::new(key.span(), "duplicate shader option"));
            }
            seen.push(key.clone());
            input.parse::<Token![=]>()?;
            match key.to_string().as_str() {
                "vertices" => {
                    let value = input.parse::<syn::LitInt>()?;
                    options.vertices = value.base10_parse()?;
                    if options.vertices == 0 {
                        return Err(syn::Error::new_spanned(value, "vertex count must be positive"));
                    }
                }
                "topology" => {
                    let value = input.parse::<Ident>()?;
                    options.topology = match value.to_string().as_str() {
                        "point_list" => "PointList",
                        "line_list" => "LineList",
                        "line_strip" => "LineStrip",
                        "triangle_list" => "TriangleList",
                        "triangle_strip" => "TriangleStrip",
                        _ => return Err(syn::Error::new(value.span(), "unknown topology")),
                    };
                }
                "blend" => {
                    let value = input.parse::<Ident>()?;
                    options.blend = match value.to_string().as_str() {
                        "replace" => "Replace",
                        "alpha" => "Alpha",
                        "premultiplied_alpha" => "PremultipliedAlpha",
                        "add" => "Add",
                        _ => return Err(syn::Error::new(value.span(), "unknown blend mode")),
                    };
                }
                _ => return Err(syn::Error::new(key.span(), "unknown shader option")),
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(options)
    }
}
