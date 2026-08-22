macro_rules! math {
    ($value:expr, $method:ident) => {{
        #[cfg(target_arch = "spirv")]
        {
            spirv_std::num_traits::Float::$method($value)
        }
        #[cfg(not(target_arch = "spirv"))]
        {
            f32::$method($value)
        }
    }};
}

pub trait FloatExt {
    fn approach(&mut self, target: Self, max_delta: Self);
    #[must_use]
    fn saturate(self) -> Self;
    #[must_use]
    fn lerp(self, other: Self, factor: Self) -> Self;
    #[must_use]
    fn sin(self) -> Self;
    #[must_use]
    fn cos(self) -> Self;
    #[must_use]
    fn floor(self) -> Self;
    #[must_use]
    fn fract(self) -> Self;
    #[must_use]
    fn exp(self) -> Self;
    #[must_use]
    fn powf(self, exponent: Self) -> Self;
    #[must_use]
    fn atan2(self, other: Self) -> Self;
    #[must_use]
    fn sqrt(self) -> Self;
    #[must_use]
    fn round(self) -> Self;
    #[must_use]
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self;
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

    fn sin(self) -> Self {
        math!(self, sin)
    }

    fn cos(self) -> Self {
        math!(self, cos)
    }

    fn floor(self) -> Self {
        math!(self, floor)
    }

    fn fract(self) -> Self {
        math!(self, fract)
    }

    fn exp(self) -> Self {
        math!(self, exp)
    }

    fn powf(self, exponent: Self) -> Self {
        #[cfg(target_arch = "spirv")]
        return spirv_std::num_traits::Float::powf(self, exponent);
        #[cfg(not(target_arch = "spirv"))]
        return Self::powf(self, exponent);
    }

    fn atan2(self, other: Self) -> Self {
        #[cfg(target_arch = "spirv")]
        return spirv_std::num_traits::Float::atan2(self, other);
        #[cfg(not(target_arch = "spirv"))]
        return Self::atan2(self, other);
    }

    fn sqrt(self) -> Self {
        math!(self, sqrt)
    }

    fn round(self) -> Self {
        math!(self, round)
    }

    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        let factor = ((self - edge0) / (edge1 - edge0)).saturate();
        factor * factor * (3.0 - 2.0 * factor)
    }
}
