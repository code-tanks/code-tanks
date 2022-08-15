use bevy_ecs::system::Query;

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
        let grouped_commands = &mut command_receiver.queue[0];

        println!("apply_commands {:?}", grouped_commands);

        for command in Commands::ALL_COMMANDS {
            if command & Commands::MOVE_FORWARD != 0 {
                velocity.velocity = 1.0;
            }
        }

        command_receiver.queue.remove(0);

        println!("commands remaining {:?}", command_receiver.queue);
    }
}
