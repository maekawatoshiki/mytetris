extern crate mytetris;
extern crate termion;

use mytetris::{field::Field, tetrimino::T3};

fn main() {
    let mut field = Field::new();

    field.show();
    field.set_cur_tetrimino(T3);
    field.show();

    field.main_loop();
}
