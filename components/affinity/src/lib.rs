mod affinity;
pub use affinity::Affinity;
pub mod common;

#[cfg(feature = "color_hints")]
mod color_hints;
#[cfg(feature = "color_hints")]
pub use color_hints::ColorHint;
