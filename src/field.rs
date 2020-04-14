use crate::{dot::*, tetrimino::*};
use rand;

const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 20;

#[derive(Debug, Clone)]
pub struct Field {
    field: [[DotColor; FIELD_WIDTH]; FIELD_HEIGHT],
    cur_tetrimino: CurrentTetrimino,
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentTetrimino {
    pub tetrimino: Tetrimino,
    pub pos: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

impl Field {
    pub fn new() -> Self {
        let cur_tetrimino = CurrentTetrimino {
            tetrimino: T2,
            pos: (FIELD_WIDTH / 2, 1),
        };
        let mut field = Field {
            field: [[DotColor::None; FIELD_WIDTH]; FIELD_HEIGHT],
            cur_tetrimino,
        };
        field.set_cur_tetrimino(None).unwrap();
        field
    }

    pub fn set_cur_tetrimino(&mut self, color: Option<DotColor>) -> Result<(), ()> {
        let (x, y) = self.cur_tetrimino.pos;
        let tetrimino = self.cur_tetrimino.tetrimino;
        let color = match color {
            Some(color) => color,
            None => tetrimino.color,
        };
        let mut new_xys = vec![];

        for (s_x, s_y) in &tetrimino.shape {
            let new_x = x as i32 + *s_x;
            let new_y = y as i32 + *s_y;

            if (0 <= new_x && new_x < FIELD_WIDTH as i32)
                && (0 <= new_y && new_y < FIELD_HEIGHT as i32)
            {
                new_xys.push((new_x as usize, new_y as usize));
            } else {
                return Err(());
            }
        }

        self.field[y][x] = color;
        for (x, y) in new_xys {
            self.field[y][x] = color;
        }

        Ok(())
    }

    fn set_next_tetrimino_if_necessary(&mut self) {
        let n: usize = rand::random();
        self.cur_tetrimino = CurrentTetrimino {
            tetrimino: TETRIMINOS[n % 8],
            pos: (FIELD_WIDTH / 2, 1),
        }
    }

    pub fn update_cur_tetrimino_with_key(&mut self, key: Keys) {
        self.set_cur_tetrimino(Some(DotColor::None)).unwrap(); // Clear current tetrimino

        let save_x = self.cur_tetrimino.pos.0;

        match key {
            Keys::Left => {
                let x = &mut self.cur_tetrimino.pos.0;
                *x = x.saturating_sub(1);
            }
            Keys::Right => {
                let x = &mut self.cur_tetrimino.pos.0;
                *x = x.saturating_add(1);
            }
            _ => {}
        }

        if self.set_cur_tetrimino(None).is_err() {
            self.cur_tetrimino.pos.0 = save_x;
            self.set_cur_tetrimino(None).unwrap();
        }
    }

    pub fn down_cur_tetrimino(&mut self) {
        self.set_cur_tetrimino(Some(DotColor::None)).unwrap(); // Clear current tetrimino

        let save_y = self.cur_tetrimino.pos.1;

        let y = &mut self.cur_tetrimino.pos.1;
        *y = y.saturating_add(1);

        if self.set_cur_tetrimino(None).is_err() {
            self.cur_tetrimino.pos.1 = save_y;
            self.set_cur_tetrimino(None).unwrap();
        }

        self.erase_lines();
        self.set_next_tetrimino_if_necessary();
    }

    // fn need_next_tetrimino(&mut self) {
    // }

    fn erase_lines(&mut self) {}

    pub fn show(&self) {
        println!("{}\r", "*".repeat(FIELD_WIDTH + 2));
        for y in 0..FIELD_HEIGHT {
            print!("*");
            for x in 0..FIELD_WIDTH {
                print!("{}", self.field[y][x].dot())
            }
            println!("*\r");
        }
        println!("{}\r", "*".repeat(FIELD_WIDTH + 2));
    }

    pub fn main_loop(&mut self) {
        use std::io::stdout;
        use std::io::{Read, Write};
        use std::{thread, time};
        use termion::async_stdin;
        // use termion::clear;
        // use termion::cursor;
        use termion::raw::IntoRawMode;

        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut astdin = async_stdin();

        // write!(stdout, "{}", clear::All);

        let mut counter = 0usize;

        loop {
            let mut duration = 50;
            let mut key = None;
            let mut buf = [0u8; 1];
            astdin.read(&mut buf).unwrap();
            if buf[0] == 113 {
                key = Some(Keys::Quit)
            } else if buf[0] == 27 {
                astdin.read(&mut buf).unwrap();
                if buf[0] == 91 {
                    astdin.read(&mut buf).unwrap();
                    key = match buf[0] {
                        65 => Some(Keys::Up),
                        68 => Some(Keys::Left),
                        67 => Some(Keys::Right),
                        66 => Some(Keys::Down),
                        _ => None,
                    }
                }
            }

            if let Some(key) = key {
                // println!("{:?}\r", key);
                self.update_cur_tetrimino_with_key(key);
                // self.update();
                stdout.flush().unwrap();
                duration = 0;
            }

            if key == Some(Keys::Quit) {
                break;
            }

            // every 200ms
            if counter % 4 == 0 {
                self.show();
            }
            // every 1000ms
            if counter % 10 == 0 {
                self.down_cur_tetrimino();
            }

            thread::sleep(time::Duration::from_millis(duration));
            counter += 1;
        }
    }
}
