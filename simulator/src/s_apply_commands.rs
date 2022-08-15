use bevy_ecs::prelude::Query;

use crate::{
    c_command::{CCommands, CommandSource},
    c_position::Position,
    c_velocity::{CVelocity, TankVelocity},
};

pub fn apply_commands(
    mut query: Query<(
        &mut CommandSource,
        &mut CVelocity,
        &mut TankVelocity,
        &Position,
    )>,
) {
    for (mut command_receiver, mut velocity, mut tank_velocity, _position) in &mut query {
        let grouped_commands = command_receiver.queue.remove(0);

        println!("apply_commands {:?}", grouped_commands);

        if CCommands::MOVE_FORWARD & grouped_commands != 0 {
            velocity.velocity = 1.0;
        }
        if CCommands::ROTATE_TANK_CLOCKWISE & grouped_commands != 0 {
            println!("derp");
            tank_velocity.angular_velocity = 1.0;
        }

        println!("commands remaining {:?}", command_receiver.queue);
    }
}
