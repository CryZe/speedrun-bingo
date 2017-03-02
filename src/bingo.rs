use {generator, Template};

#[derive(Debug, Eq, PartialEq)]
pub struct Bingo {
    pub cells: [[String; 5]; 5],
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Short,
    Normal,
    Long,
    Special,
}

impl Bingo {
    pub fn new(seed: u32, mode: Mode, template: &Template) -> Self {
        generator::generate(seed, mode, template)
    }
}
