use std::env;

use ctsim::run_game;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = &args[1..];
    println!("running game: {:?}", args);

    // let mut g = create_game(args);
    run_game(args);
    println!("running game: {:?}", args);

    // if &args[1] == "-f"
}
