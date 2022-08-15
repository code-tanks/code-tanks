use bevy::prelude::Query;

use crate::{
    c_command::{CommandSource, Commands},
    c_position::Position,
    c_velocity::{TankVelocity, Velocity},
};

pub fn apply_commands(
    mut query: Query<(
        &mut CommandSource,
        &mut Velocity,
        &mut TankVelocity,
        &Position,
    )>,
) {
    for (mut command_receiver, mut velocity, mut tank_velocity, position) in &mut query {
        let grouped_commands = command_receiver.queue.remove(0);

        println!("apply_commands {:?}", grouped_commands);

        if Commands::MOVE_FORWARD & grouped_commands != 0 {
            velocity.velocity = 1.0;
        }
        if Commands::ROTATE_TANK_CLOCKWISE & grouped_commands != 0 {
            println!("derp");
            tank_velocity.angular_velocity = 1.0;
        }

        println!("commands remaining {:?}", command_receiver.queue);
    }
}
