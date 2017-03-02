#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod bingo;
mod seed_random;
mod template;
mod generator;

pub use template::{Template, Goal};
pub use bingo::{Mode, Bingo};
