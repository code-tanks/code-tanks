use std::env;

use ctsimlib::run_game;
use worker_simulator::run_tank;

fn main() {
    let args = &env::args().collect::<Vec<String>>()[1..];

    let game_url = args.join("");

    let tank_ids = args
        .iter()
        .enumerate()
        .map(|i, url| run_tank(url, &game_url, i))
        .collect();

    println!("running game with tank_ids {}", tank_ids);

    run_game(tank_ids);
}
