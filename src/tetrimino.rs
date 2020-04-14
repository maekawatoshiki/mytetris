use crate::dot::DotColor;

/*
 * **** No1
 *
 *   *
 * ***  No2
 *
 * **
 * **   No3
 *
 *  **
 * **   No4
 *
 * **
 *  **  No5
 *
 * *
 * ***  No6
 *
 *   *
 * ***  No7
 *
 *  *
 * ***  No8
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tetrimino {
    pub color: DotColor,
    pub shape: [(i32, i32); 3],
}

impl Tetrimino {
    pub const fn new(color: DotColor, shape: [(i32, i32); 3]) -> Self {
        Self { color, shape }
    }
}

pub const T1: Tetrimino = Tetrimino::new(DotColor::Black, [(-1, 0), (1, 0), (2, 0)]);
pub const T2: Tetrimino = Tetrimino::new(DotColor::Black, [(-1, 0), (1, 0), (1, -1)]);
pub const T3: Tetrimino = Tetrimino::new(DotColor::Black, [(-1, 0), (-1, 1), (0, -1)]);
pub const T4: Tetrimino = Tetrimino::new(DotColor::Black, [(-1, 0), (0, 1), (1, -1)]);
pub const T5: Tetrimino = Tetrimino::new(DotColor::Black, [(-1, 1), (0, 1), (1, 0)]);
pub const T6: Tetrimino = Tetrimino::new(DotColor::Black, [(-1, 0), (-1, 1), (1, 0)]);
pub const T7: Tetrimino = Tetrimino::new(DotColor::Black, [(-1, 0), (1, 0), (1, -1)]);
pub const T8: Tetrimino = Tetrimino::new(DotColor::Black, [(-1, 0), (1, 0), (0, -1)]);
pub const TETRIMINOS: [Tetrimino; 8] = [T1, T2, T3, T4, T5, T6, T7, T8];
