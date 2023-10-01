#[cfg(any(feature = "better_enums", feature = "strum"))]
extern crate strum;
#[cfg(any(feature = "better_enums", feature = "strum_macros"))]
#[macro_use]
extern crate strum_macros;

pub use overworld_dice::difficulty;
pub use overworld_dice::roll;

pub mod race;
