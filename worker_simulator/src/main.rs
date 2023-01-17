use core::time;
use std::{fs, thread};

use worker_simulator::{
    create_sim_queue,
    db::{get_client, upload_sim},
    get_sim_job, remove_tank, run_docker_game, update_sim_job, upload_log,
};

fn main() {
    println!("Started ctsim");

    create_sim_queue();

    let mut client = get_client();

    loop {
        let job = get_sim_job();

        if !job.is_empty() {
            let id = &job[0];
            let args = &job[1]
                .split(" ")
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let tank_ids = run_docker_game(args);
            let game_id = &args.join("-");
            for tank_id in tank_ids.iter() {
                upload_log(tank_id, &mut client);
                remove_tank(tank_id);
            }
            let sim = fs::read_to_string("./sim.txt").expect("Unable to read file");
            let uploaded_sim = upload_sim(&mut client, game_id, &sim, true);
            update_sim_job(id, uploaded_sim);
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
