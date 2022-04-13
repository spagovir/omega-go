extern crate omega_go;
use omega_go::board;

fn main() {
    let current_board = board::init_board(19);
    print!("{}", board::show_board(&current_board));
}
