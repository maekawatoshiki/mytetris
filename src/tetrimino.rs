use crate::field::DotKind;

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
    kind: DotKind,
    shape: [(i32, i32); 3],
}

impl Tetrimino {
    pub const fn new(kind: DotKind, shape: [(i32, i32); 3]) -> Self {
        Self { kind, shape }
    }
}

pub const T1: Tetrimino = Tetrimino::new(DotKind::Black, [(-1, 0), (1, 0), (2, 0)]);
pub const T2: Tetrimino = Tetrimino::new(DotKind::Black, [(-1, 0), (1, 0), (1, 1)]);
pub const T3: Tetrimino = Tetrimino::new(DotKind::Black, [(-1, 0), (-1, 1), (1, 1)]);
pub const T4: Tetrimino = Tetrimino::new(DotKind::Black, [(-1, 0), (0, 1), (1, 1)]);
pub const T5: Tetrimino = Tetrimino::new(DotKind::Black, [(-1, 1), (0, 1), (1, 0)]);
pub const T6: Tetrimino = Tetrimino::new(DotKind::Black, [(-1, 0), (-1, 1), (1, 0)]);
pub const T7: Tetrimino = Tetrimino::new(DotKind::Black, [(-1, 0), (1, 0), (1, 1)]);
pub const T8: Tetrimino = Tetrimino::new(DotKind::Black, [(-1, 0), (1, 0), (0, 1)]);
