#![cfg_attr(target_arch = "spirv", no_std)]

#[cfg(feature = "cpu")]
mod app;
#[cfg(feature = "shader")]
#[doc(hidden)]
pub mod render;
#[cfg(feature = "cpu")]
pub use app::run;
#[cfg(feature = "cpu")]
pub(crate) use app::{
    AppUpdater, CantusApp, MAX_RENDER_INSTANCES, PANEL_OVERFLOW, PANEL_START, PARTICLE_COUNT,
    TRACK_SPACING_MS, Update, config, interaction, openmeteo, platform, send_update, spotify,
};
