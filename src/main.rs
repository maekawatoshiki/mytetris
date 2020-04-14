extern crate mytetris;
extern crate termion;

use mytetris::field::Field;

fn main() {
    let mut field = Field::new();

    // field.set_new_cur_tetrimino(T3);
    field.show();
    field.show();

    field.main_loop();
}
