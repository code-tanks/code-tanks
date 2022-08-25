use std::env;

use ctdesktop::run_game;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = &args[1..]; 
    run_game(args);
    println!("running game: {:?}", args);
}