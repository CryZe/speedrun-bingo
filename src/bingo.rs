use {generator, Template};

#[derive(Debug, Eq, PartialEq)]
pub struct Bingo<'a> {
    pub cells: [[&'a str; 5]; 5],
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Short,
    Normal,
    Long,
    Special,
}

impl<'a> Bingo<'a> {
    pub fn new(seed: u32, mode: Mode, template: &'a Template) -> Self {
        generator::generate(seed, mode, template)
    }
}
