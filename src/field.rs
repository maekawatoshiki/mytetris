#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Dot {
    Black,
    None,
}

#[derive(Debug, Clone)]
pub struct Field {
    field: [[Dot; 10]; 20],
}

impl Field {
    pub fn new() -> Self {
        Field {
            field: [[Dot::None; 10]; 20],
        }
    }
}
