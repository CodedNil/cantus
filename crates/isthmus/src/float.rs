macro_rules! gpu_math {
    (trait $($method:ident),*) => { $(#[cfg(target_arch = "spirv")] fn $method(self) -> Self;)* };
    (impl $($method:ident),*) => { $(#[cfg(target_arch = "spirv")] fn $method(self) -> Self { spirv_std::num_traits::Float::$method(self) })* };
}

macro_rules! gpu_math_binary {
    (trait $($method:ident),*) => { $(#[cfg(target_arch = "spirv")] fn $method(self, other: Self) -> Self;)* };
    (impl $($method:ident),*) => { $(#[cfg(target_arch = "spirv")] fn $method(self, other: Self) -> Self { spirv_std::num_traits::Float::$method(self, other) })* };
}

pub trait FloatExt {
    fn approach(&mut self, target: Self, max_delta: Self);
    #[must_use]
    fn saturate(self) -> Self;
    #[must_use]
    fn lerp(self, other: Self, factor: Self) -> Self;
    #[must_use]
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self;
    gpu_math!(trait sin, cos, floor, fract, exp, sqrt, round);
    gpu_math_binary!(trait powf, atan2);
}

impl FloatExt for f32 {
    fn approach(&mut self, target: Self, max_delta: Self) {
        *self += (target - *self).clamp(-max_delta, max_delta);
    }

    fn saturate(self) -> Self {
        glam::FloatExt::saturate(self)
    }

    fn lerp(self, other: Self, factor: Self) -> Self {
        glam::FloatExt::lerp(self, other, factor)
    }

    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        let factor = ((self - edge0) / (edge1 - edge0)).saturate();
        factor * factor * (3.0 - 2.0 * factor)
    }

    gpu_math!(impl sin, cos, floor, fract, exp, sqrt, round);
    gpu_math_binary!(impl powf, atan2);
}
