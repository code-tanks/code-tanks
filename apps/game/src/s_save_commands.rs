use bevy::prelude::Query;
use std::{fs::OpenOptions, io::Write};

use crate::{
    c_command::{CommandSource, Commands},
    c_position::Position,
    c_velocity::{TankVelocity, Velocity},
};

pub fn save_commands(
    mut query: Query<(
        &mut CommandSource,
        &mut Velocity,
        &mut TankVelocity,
        &Position,
    )>,
) {
    for (mut command_receiver, mut velocity, mut tank_velocity, position) in &mut query {
        let grouped_commands = command_receiver.queue[0];

        println!("save_commands {:?}", grouped_commands);

        let mut f = OpenOptions::new()
            .append(true)
            .open("./sim.ct")
            .expect("Unable to open file");
        f.write_all(format!("{}\n", grouped_commands).to_string().as_bytes())
            .expect("Unable to write data");

        println!("commands remaining {:?}", command_receiver.queue);
    }
}
