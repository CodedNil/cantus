use syn::{
    Attribute, Expr, FnArg, Ident, ItemFn, Pat, Result as SynResult, Token, Type,
    parse::{Parse, ParseStream},
};

#[derive(Clone)]
pub struct GpuParameter {
    pub kind: Ident,
    pub role: GpuRole,
    pub index: Option<Expr>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GpuRole {
    VertexIndex,
    InstanceIndex,
    Shared,
    Instance,
    Resource,
    Flat,
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
    pub attribute: usize,
    pub binding: Option<u32>,
    pub name: Ident,
    pub ty: Type,
    pub role: GpuParameter,
}

pub struct StageContract {
    pub parameters: Vec<StageParameter>,
}

impl StageContract {
    pub fn parse(function: &ItemFn, stage: Stage) -> SynResult<Self> {
        let mut parameters = Vec::new();
        let mut resource_binding = 2;
        for (argument_index, argument) in function.sig.inputs.iter().enumerate() {
            let FnArg::Typed(argument) = argument else {
                return Err(syn::Error::new_spanned(argument, "shader functions cannot have self"));
            };
            let attribute = gpu_attribute(&argument.attrs)?;
            let is_varyings = matches!(stage, Stage::Fragment) && argument_index == 0;
            if is_varyings {
                if let Some(attribute) = attribute {
                    return Err(syn::Error::new_spanned(
                        &argument.attrs[attribute],
                        "the fragment varying parameter does not take a GPU role",
                    ));
                }
                continue;
            }
            let Some(attribute) = attribute else {
                return Err(syn::Error::new_spanned(argument, "shader parameters require a `#[gpu(...)]` role"));
            };
            let role: GpuParameter = argument.attrs[attribute].parse_args()?;
            validate_role(&role, stage)?;
            if matches!(role.role, GpuRole::VertexIndex | GpuRole::InstanceIndex) && !is_named_type(&argument.ty, "u32") {
                return Err(syn::Error::new_spanned(&argument.ty, "GPU index parameters must use `u32`"));
            }
            let Pat::Ident(pattern) = argument.pat.as_ref() else {
                return Err(syn::Error::new_spanned(&argument.pat, "GPU parameters require an identifier"));
            };
            if role_is_unique(role.role) && parameters.iter().any(|parameter: &StageParameter| parameter.role.role == role.role) {
                return Err(syn::Error::new_spanned(&argument.attrs[attribute], "a shader stage can only declare this GPU role once"));
            }
            let binding = match role.role {
                GpuRole::Shared => Some(0),
                GpuRole::Instance => Some(1),
                GpuRole::Resource => {
                    let binding = resource_binding;
                    resource_binding += 1;
                    Some(binding)
                }
                _ => None,
            };
            parameters.push(StageParameter {
                argument: argument_index,
                attribute,
                binding,
                name: pattern.ident.clone(),
                ty: (*argument.ty).clone(),
                role,
            });
        }
        Ok(Self { parameters })
    }

    pub fn has_implicit(&self, role: GpuRole) -> bool {
        self.parameters.iter().any(|parameter| parameter.role.role == role && parameter.role.index.is_none())
    }
}

fn is_named_type(ty: &Type, name: &str) -> bool {
    matches!(
        ty,
        Type::Path(path) if path.qself.is_none()
            && path.path.segments.last().is_some_and(|segment| segment.ident == name)
    )
}

const fn role_is_unique(role: GpuRole) -> bool {
    !matches!(role, GpuRole::Resource | GpuRole::Flat)
}

fn validate_role(parameter: &GpuParameter, stage: Stage) -> SynResult<()> {
    if parameter.index.is_some() && parameter.role != GpuRole::Instance {
        return Err(syn::Error::new(parameter.kind.span(), "only instance parameters can specify an index"));
    }
    let allowed = match stage {
        Stage::Vertex => matches!(
            parameter.role,
            GpuRole::VertexIndex | GpuRole::InstanceIndex | GpuRole::Shared | GpuRole::Instance | GpuRole::Resource
        ),
        Stage::Fragment => matches!(parameter.role, GpuRole::Shared | GpuRole::Instance | GpuRole::Resource),
    };
    if allowed {
        Ok(())
    } else {
        Err(syn::Error::new(parameter.kind.span(), "this GPU role is not valid for this shader stage"))
    }
}

pub fn gpu_attribute(attributes: &[Attribute]) -> SynResult<Option<usize>> {
    let mut matches = attributes.iter().enumerate().filter(|(_, attribute)| attribute.path().is_ident("gpu"));
    let found = matches.next().map(|(index, _)| index);
    if let Some((_, duplicate)) = matches.next() {
        return Err(syn::Error::new_spanned(duplicate, "only one GPU role may be specified"));
    }
    Ok(found)
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

impl Parse for GpuParameter {
    fn parse(input: ParseStream<'_>) -> SynResult<Self> {
        let kind: Ident = input.parse()?;
        let role = match kind.to_string().as_str() {
            "vertex_index" => GpuRole::VertexIndex,
            "instance_index" => GpuRole::InstanceIndex,
            "shared" => GpuRole::Shared,
            "instance" => GpuRole::Instance,
            "resource" => GpuRole::Resource,
            "flat" => GpuRole::Flat,
            _ => return Err(syn::Error::new(kind.span(), "unknown GPU parameter role")),
        };
        let index = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { kind, role, index })
    }
}
