use core::time;
use std::fs::File;
use std::io::Write;
use std::{process::Command, thread};
use std::env;

use bevy::MinimalPlugins;
use bevy::prelude::{App, Startup, Update, IntoSystemConfigs};
use ctsimlib::c_tank::TankInfo;
use ctsimlib::core_plugin::CoreCTPlugin;
use ctsimlib::*;
use ctsimlib::s_apply_commands::apply_commands;
use ctsimlib::s_request_commands::request_commands;
use ctsimlib::s_save_commands::save_commands;
use ctsimlib::s_setup_sim_tanks::setup_sim_tanks;
use ctsimlib::s_setup_walls::setup_walls;
use db::upload_log_to_db;
use postgres::Client;

pub mod db;

pub fn create_sim_queue() {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPUT")
        .arg("-d")
        .arg(r#"{"timeout": "10m"}"#)
        .arg(format!("{}/queue/simulator", env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()))
        .output()
        .expect("failed to communicate with ocypod");
}

pub fn get_sim_job() -> Vec<String> {
    let output_raw = Command::new("bash")
        .arg("-c")
        .arg(format!(r#"curl {}/queue/simulator/job | jq --raw-output '.id,.input'"#, env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()))
        .arg(format!("{}/queue/build/job", env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap()))
        .output()
        .expect("failed to communicate with ocypod");

    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    result_raw
        .to_string()
        .split('\n')
        .map(|f| f.to_string())
        .filter(|f| !f.is_empty())
        .collect::<Vec<String>>()
}

pub fn upload_log(tank_id: &str, client: &mut Client) {
    let output_raw = Command::new("docker")
        .arg("logs")
        .arg("--timestamps")
        .arg(tank_id)
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);
    let err_raw = String::from_utf8_lossy(&output_raw.stderr);

    upload_log_to_db(client, tank_id, &result_raw, &err_raw);
}

// pub fn run_docker_game(tank_hashes: &[String]) {
//     let game_url: String = tank_hashes.join("");

//     let tank_infos = tank_hashes.iter().enumerate().map(|(i, f)| TankInfo{
//         hash: f.to_string(),
//         id: format!("{}-{}", f, i),
//         index: i,
//         container_name: format!("{}-{}-{}", game_url, f, i),
//     }).collect::<Vec<TankInfo>>();
//     // let tank_container_name = format!("{}-{}-{}", game_url, tank_info.hash, tank_info.index);

//     for tank_info in tank_infos {
//         run_tank(&tank_info.container_name, &tank_info.hash);
//     }

//     // let tank_container_names = tank_hashes
//     //     .iter()
//     //     .enumerate()
//     //     .map(|(i, url)| run_tank(url, &game_url, i))
//     //     .collect::<Vec<String>>();
//     thread::sleep(time::Duration::from_millis(5000));

//     let mut f = File::create("./sim.txt").expect("Unable to create file");
//     f.write_all(format!("{}\n", tank_hashes.join(",")).as_bytes())
//         .expect("Unable to write data");

//     App::new()
//         .add_plugins(MinimalPlugins)
//         // .insert_resource(TankInfo { // TODO fix
//         //     tank_ids: tank_hashes.to_vec(),
//         //     tank_container_name: tank_container_names.to_vec(),
//         // })
//         .add_systems(Startup, (setup_walls, setup_sim_tanks).chain())
//         .add_plugins(CoreCTPlugin)
//         .add_systems(
//             Update,
//             save_commands.after(request_commands).before(apply_commands)
//             // "request_commands",
//             // "save_commands",
//             // SystemStage::single_threaded().with_system(save_commands),
//         )
//         .run();
// }

pub fn update_sim_job(id: &str, successful: bool) {
    Command::new("curl")
        .arg("-H")
        .arg("content-type: application/json")
        .arg("-XPATCH")
        .arg("-d")
        .arg(format!(
            r#"{{"status": "{}"}}"#,
            if successful { "completed" } else { "failed" }
        ))
        .arg(format!("{}/job/{}", env::var("OCYPOD_URL").unwrap().parse::<String>().unwrap(), id))
        .output()
        .expect("failed to communicate with ocypod");

    println!("update job, id={}", id);
}
