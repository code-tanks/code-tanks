use bevy::prelude::*;
use serde_json::json;
use std::{fs::OpenOptions, io::Write};

use crate::{
    c_command::{CCommands, CommandSource},
    c_tank::*,
    TickState, TankIds, c_health::Health,
};
use bevy::app::AppExit;

pub fn save_commands(
    mut state: ResMut<TickState>,
    tank_ids_state: Res<TankIds>,
    mut exit: EventWriter<AppExit>,
    query: Query<&CommandSource>,
    tanks: Query<&Transform, With<Tank>>,
    radars: Query<&Transform, With<Radar>>,
    guns: Query<&Transform, With<Gun>>,
    healths: Query<&Health, With<Tank>>,
) {
    let tanks: Vec<&Transform> = tanks.iter().collect();
    let radars: Vec<&Transform> = radars.iter().collect();
    let guns: Vec<&Transform> = guns.iter().collect();
    let healths: Vec<&Health> = healths.iter().collect();

    let mut f = OpenOptions::new()
        .append(true)
        .open("./sim.txt")
        .expect("Unable to open file");

    for (i, command_receiver) in query.iter().enumerate() {
        let grouped_commands = if command_receiver.queue.is_empty() {
            CCommands::NONE
        } else {
            command_receiver.queue[0]
        };

        // println!("save_commands {:?}", grouped_commands);

        f.write_all(
            format!(
                "{}|{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                grouped_commands,
                tanks[i].translation.x,
                tanks[i].translation.y,
                tanks[i].rotation.x,
                tanks[i].rotation.y,
                tanks[i].rotation.z,
                tanks[i].rotation.w,
                radars[i].rotation.x,
                radars[i].rotation.y,
                radars[i].rotation.z,
                radars[i].rotation.w,
                guns[i].rotation.x,
                guns[i].rotation.y,
                guns[i].rotation.z,
                guns[i].rotation.w,
            )
            .to_string()
            .as_bytes(),
        )
        .expect("Unable to write data");

        // println!("commands remaining {:?}", command_receiver.queue);
    }
    state.tick = state.tick + 1;

    if state.tick > TickState::MAXIMUM_SIMULATION_TICKS {
        // TODO save results of the simulation (winner, damage given, damage taken, time alive)
        let mut j = json!({});
        for (i, tank_id) in tank_ids_state.tank_ids.iter().enumerate() {
            let ti = &tank_id[tank_id.find("-").unwrap() + 1..];
            j[ti] = json!({
                "tank_id": ti[..ti.find("-").unwrap()],
                "index": i,
                "health": healths[i].val,
            });
        }

        f.write_all(j.to_string().as_bytes())
            .expect("Unable to write data");
        exit.send(AppExit);
    }
}
