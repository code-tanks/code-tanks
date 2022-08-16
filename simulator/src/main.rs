use core::time;
use std::thread;

use ctsimlib::{create_sim_queue, db::get_client, get_job, run_game};

fn main() {
    println!("Started ctsim");

    create_sim_queue();

    let mut _client = get_client();

    loop {
        let job = get_job();

        if !job.is_empty() {
            let _id = &job[0];
            let args = &job[1]
                .chars()
                .collect::<Vec<char>>()
                .chunks(8)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();

            run_game(args);
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
