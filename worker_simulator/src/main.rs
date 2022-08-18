use core::time;
use std::thread;

use worker_simulator::{create_sim_queue, db::get_client, get_sim_job, run_docker_game};

fn main() {
    println!("Started ctsim");

    create_sim_queue();

    let mut _client = get_client();

    loop {
        let job = get_sim_job();

        if !job.is_empty() {
            let _id = &job[0];
            let args = &job[1]
                .split(" ")
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            run_docker_game(args);
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
