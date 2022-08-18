use std::env;

use worker_simulator::run_docker_game;

fn main() {
    println!("Running test");
    let args = &env::args().collect::<Vec<String>>()[1..];

    run_docker_game(args);
    // let game_url = args.join("");

    // let tank_ids = args
    //     .iter()
    //     .enumerate()
    //     .map(|(i, url)| run_tank(url, &game_url, i))
    //     .collect::<Vec<String>>();

    // println!("running game with tank_ids {:?}", tank_ids);

    // run_game(&tank_ids);
    // for tank_id in tank_ids {
    //     remove_tank(&tank_id);
    // }
}
