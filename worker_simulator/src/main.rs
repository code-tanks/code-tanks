use core::time;
use std::{fs::{self, File}, thread, io::Write};

use ctsimlib::{remove_tank, c_tank::{TankInfo, AllTankInfo}};
use worker_simulator::{
    create_sim_queue,
    db::{get_client, upload_sim},
    get_sim_job, update_sim_job, upload_log,
};
use bevy::MinimalPlugins;
use bevy::prelude::{App, Startup, Update, IntoSystemConfigs};
// use ctsimlib::c_tank::TankInfo;
use ctsimlib::core_plugin::CoreCTPlugin;
use ctsimlib::*;
use ctsimlib::s_apply_commands::apply_commands;
use ctsimlib::s_request_commands::request_commands;
use ctsimlib::s_save_commands::save_commands;
use ctsimlib::s_setup_sim_tanks::setup_sim_tanks;
use ctsimlib::s_setup_walls::setup_walls;

fn main() {
    println!("Started ctsim");

    create_sim_queue();

    let mut client = get_client();

    loop {
        println!("getting sim job");
        let job = get_sim_job();

        if !job.is_empty() {
            println!("got {:?}", job);
            let id = &job[0];
            let tank_hashes = &job[1]
                .split(' ')
                .map(|f| f.to_string())
                .collect::<Vec<String>>();

            // let tank_container_names = run_docker_game(args);
            // TODO common
            let game_url: String = tank_hashes.join("");
            let tank_infos = &tank_hashes.iter().enumerate().map(|(i, f)| TankInfo{
                hash: f.to_string(),
                id: format!("{}-{}", f, i),
                index: i,
                container_name: format!("{}-{}-{}", game_url, f, i),
            }).collect::<Vec<TankInfo>>();

            for tank_info in tank_infos {
                // TODO fix
                let tank_image_name = format!("localhost:5001/{}", &tank_info.hash);
                run_tank(&tank_info.container_name, &tank_image_name, "8080");
            }
        
            // let tank_container_names = tank_hashes
            //     .iter()
            //     .enumerate()
            //     .map(|(i, url)| run_tank(url, &game_url, i))
            //     .collect::<Vec<String>>();
            thread::sleep(time::Duration::from_millis(5000));
        
            let mut f = File::create("./sim.txt").expect("Unable to create file");
            f.write_all(format!("{}\n", tank_hashes.join(",")).as_bytes())
                .expect("Unable to write data");
            
            App::new()
                .add_plugins(MinimalPlugins)
                .insert_resource(AllTankInfo{
                    all: tank_infos.to_vec()
                })
                .add_systems(Startup, (setup_walls, setup_sim_tanks).chain())
                .add_plugins(CoreCTPlugin)
                .add_systems(
                    Update,
                    save_commands.after(request_commands).before(apply_commands),
                )
                .run();

            // let game_id = &tank_hashes.join("-");
            for tank_info in tank_infos {
                upload_log(&tank_info.container_name, &mut client);
                remove_tank(&tank_info.container_name);
            }
            let sim = fs::read_to_string("./sim.txt").expect("Unable to read file");
            let uploaded_sim = upload_sim(&mut client, &game_url, &sim, true);
            update_sim_job(id, uploaded_sim);
        }
        println!("no jobs found. sleeping for 1 second");

        thread::sleep(time::Duration::from_millis(1000));
    }
}
