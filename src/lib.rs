#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate image;
extern crate rusttype;
extern crate imageproc;

mod bingo;
mod seed_random;
mod template;
mod generator;
mod renderer;

pub use template::{Template, Goal};
pub use bingo::{Mode, Bingo};
pub use renderer::render;
