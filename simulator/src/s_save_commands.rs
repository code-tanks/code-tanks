use bevy::prelude::*;
use std::{fs::OpenOptions, io::Write};

use crate::{c_command::CommandSource, TickState, c_tank::*};
use bevy::app::AppExit;

pub fn save_commands(
    mut state: ResMut<TickState>,
    mut exit: EventWriter<AppExit>,
    query: Query<&CommandSource>,
    tanks: Query<&Transform, With<Tank>>,
    radars: Query<&Transform, With<Radar>>,
    guns: Query<&Transform, With<Gun>>,
) {
    let tanks: Vec<&Transform> = tanks.iter().collect();
    let radars: Vec<&Transform> = radars.iter().collect();
    let guns: Vec<&Transform> = guns.iter().collect();
    for (i, command_receiver) in query.iter().enumerate() {
        let grouped_commands = command_receiver.queue[0];

        // println!("save_commands {:?}", grouped_commands);

        let mut f = OpenOptions::new()
            .append(true)
            .open("./sim.txt")
            .expect("Unable to open file");
        f.write_all(
            format!(
                "{}|{}{}{}{}{}{}{}{}{}{}\n",
                 grouped_commands, 
                 tanks[i].translation.x,
                 tanks[i].translation.y,
                 radars[i].rotation.x,
                 radars[i].rotation.y,
                 radars[i].rotation.z,
                 radars[i].rotation.w,
                 guns[i].rotation.x,
                 guns[i].rotation.y,
                 guns[i].rotation.z,
                 guns[i].rotation.w,
                ).to_string().as_bytes())
            .expect("Unable to write data");

        // println!("commands remaining {:?}", command_receiver.queue);
    }
    state.tick = state.tick + 1;

    if state.tick > 2000 {
        exit.send(AppExit);
    }
}
