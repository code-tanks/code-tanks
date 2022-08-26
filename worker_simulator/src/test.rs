use std::env;

use worker_simulator::run_docker_game;

fn main() {
    println!("Running test");
    let args = &env::args().collect::<Vec<String>>()[1..];

    run_docker_game(args);
}
