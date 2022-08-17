use std::env;

use ctsimlib::run_game;

fn main() {
    let args = &env::args().collect::<Vec<String>>()[1..];

    run_game(args);
}
