use crate::{dot::*, tetrimino::*};

const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 20;

#[derive(Debug, Clone)]
pub struct Field {
    field: [[DotColor; FIELD_HEIGHT]; FIELD_WIDTH],
    cur_tetrimino: Option<CurrentTetrimino>,
}

#[derive(Debug, Clone)]
pub struct CurrentTetrimino {
    tetrimino: Tetrimino,
    pos: (usize, usize),
}

impl Field {
    pub fn new() -> Self {
        Field {
            field: [[DotColor::None; FIELD_HEIGHT]; FIELD_WIDTH],
            cur_tetrimino: None,
        }
    }

    pub fn set_cur_tetrimino(&mut self, tetrimino: Tetrimino) {
        let init_x = FIELD_WIDTH / 2;
        let init_y = 0;

        self.cur_tetrimino = Some(CurrentTetrimino {
            tetrimino,
            pos: (init_x, init_y),
        });

        self.draw_tetrimino(tetrimino, (init_x, init_y));
    }

    pub fn draw_tetrimino(&mut self, tetrimino: Tetrimino, (x, y): (usize, usize)) {
        self.field[x][y] = tetrimino.color;
        for (s_x, s_y) in &tetrimino.shape {
            self.field[(x as i32 + *s_x) as usize][(y as i32 + *s_y) as usize] = tetrimino.color;
        }
    }

    pub fn show(&self) {
        println!("{}", "*".repeat(FIELD_WIDTH + 2));
        for y in 0..FIELD_HEIGHT {
            print!("*");
            for x in 0..FIELD_WIDTH {
                print!("{}", self.field[x][y].dot())
            }
            println!("*");
        }
        println!("{}", "*".repeat(FIELD_WIDTH + 2));
    }

    pub fn main_loop(&self) {
        use std::io::Read;
        use std::io::{stdin, stdout, Write};
        use std::{thread, time};
        use termion::async_stdin;
        use termion::clear;
        use termion::cursor;
        use termion::raw::IntoRawMode;

        let mut stdout = stdout().into_raw_mode().unwrap();

        // write!(stdout, "{}", clear::All);

        let mut astdin = async_stdin();

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Keys {
            Up,
            Down,
            Left,
            Right,
            Quit,
        }

        loop {
            let mut buf = [0u8; 1];
            let r = astdin.read(&mut buf);
            let mut key = None;
            if buf[0] == 113 {
                key = Some(Keys::Quit)
            } else if buf[0] == 27 {
                astdin.read(&mut buf);
                if buf[0] == 91 {
                    astdin.read(&mut buf);
                    key = match buf[0] {
                        65 => Some(Keys::Up),
                        68 => Some(Keys::Left),
                        67 => Some(Keys::Right),
                        66 => Some(Keys::Down),
                        _ => None,
                    }
                }
            } else {
                key = None;
            }

            if key.is_some() {
                println!("{:?}\r", key);
                stdout.flush();
            }

            if key == Some(Keys::Quit) {
                break;
            }

            thread::sleep(time::Duration::from_millis(50));
        }
    }
}
