#![allow(clippy::missing_panics_doc)]

#[cfg(feature = "cpu")]
pub mod art;
#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "cpu")]
pub mod frame;

pub mod particles;
pub mod playhead;
pub(crate) mod shader;
pub(crate) mod shared;
pub mod status;
pub mod tempo;
pub mod text;
pub mod track;

#[cfg(feature = "cpu")]
pub(crate) use cpu::Passes;
#[cfg(feature = "cpu")]
pub use cpu::{RenderState, Systems};
#[cfg(feature = "cpu")]
pub(crate) use frame::approach;
