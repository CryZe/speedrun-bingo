#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

extern crate arrayvec;

#[cfg(feature = "std")]
extern crate image;
#[cfg(feature = "std")]
extern crate imageproc;
#[cfg(feature = "std")]
extern crate rusttype;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "std")]
extern crate serde_json;
#[cfg(all(not(feature = "std"), feature = "json_core"))]
extern crate serde_json_core as serde_json;

mod bingo;
mod seed_random;
mod template;
mod generator;
#[cfg(feature = "std")]
mod renderer;

pub use template::{Goal, Template};
pub use bingo::{Bingo, Mode};
#[cfg(feature = "std")]
pub use renderer::render;
