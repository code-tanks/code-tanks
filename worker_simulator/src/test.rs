use std::env;

use worker_simulator::{remove_tank, run_docker_game};

fn main() {
    println!("Running test");
    let args = &env::args().collect::<Vec<String>>()[1..];

    let tank_ids = run_docker_game(args);
    for tank_id in tank_ids.iter() {
        remove_tank(tank_id);
    }
}
