use crate::tetrimino::*;

const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 20;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DotKind {
    Black,
    None,
}

#[derive(Debug, Clone)]
pub struct Field {
    field: [[DotKind; FIELD_HEIGHT]; FIELD_WIDTH],
    cur_tetrimino: Option<CurrentTetrimino>,
}

#[derive(Debug, Clone)]
pub struct CurrentTetrimino {
    tetrimino: Tetrimino,
    pos: (i32, i32),
}

impl Field {
    pub fn new() -> Self {
        Field {
            field: [[DotKind::None; FIELD_HEIGHT]; FIELD_WIDTH],
            cur_tetrimino: None,
        }
    }

    pub fn show(&self) {
        for x in 0..FIELD_WIDTH {
            for y in 0..FIELD_HEIGHT {
                print!("{}", self.field[x][y].dot())
            }
            println!();
        }
    }
}

impl DotKind {
    pub fn dot(&self) -> char {
        match self {
            Self::Black => '*',
            Self::None => ' ',
        }
    }
}
