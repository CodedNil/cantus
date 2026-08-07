#![allow(clippy::missing_panics_doc)]

#[cfg(feature = "cpu")]
pub mod art;
#[cfg(feature = "cpu")]
mod cpu;

pub mod particles;
pub mod playhead;
pub(crate) mod shader;
pub(crate) mod shared;
pub mod status;
pub mod tempestas;
pub mod text;
pub mod track;

#[cfg(feature = "cpu")]
pub(crate) use cpu::{Frame, Passes, approach};
#[cfg(feature = "cpu")]
pub use cpu::{RenderState, Systems};
