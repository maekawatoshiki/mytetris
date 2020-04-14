#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DotColor {
    Black,
    None,
}

impl DotColor {
    pub fn dot(&self) -> char {
        match self {
            Self::Black => '*',
            Self::None => ' ',
        }
    }
}
