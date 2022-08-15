use std::env;

use ctsim::{create_game, create_view_game, run_game};

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "run" {
        let args = &args[2..];
        println!("running game: {:?}", args);

        let mut g = create_game(args);
        run_game(&mut g);
        println!("running game: {:?}", args);
    } else if &args[1] == "view" {
        let arg = &args[2];
        let mut g = create_view_game(arg);
        run_game(&mut g);
        println!("viewing game: {}", arg);
    } else {
        println!("unexpected command: {}", &args[1]);
    }

    // if &args[1] == "-f"
}
