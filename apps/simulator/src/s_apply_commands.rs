use bevy_ecs::system::Query;

use crate::{
    c_command::{CommandSource, CommandType, COMMAND_TYPES_LENGTH},
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

        for command_type_as_usize in 0..COMMAND_TYPES_LENGTH {
            if grouped_commands.command_array[command_type_as_usize] > 0 {
                grouped_commands.command_array[command_type_as_usize] =
                    grouped_commands.command_array[command_type_as_usize] - 1;

                let command_type: CommandType =
                    unsafe { ::std::mem::transmute(command_type_as_usize) };

                match command_type {
                    CommandType::None => {}
                    CommandType::MoveForward => {
                        println!("AheadBy");

                        velocity.velocity = 1.0;
                    }
                    CommandType::MoveBackward => {
                        velocity.velocity = -1.0;
                    }
                    CommandType::RotateTankClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::RotateTankCounterClockwise => {
                        tank_velocity.angular_velocity = -1.0;
                    }
                    CommandType::RotateGunClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::RotateGunCounterClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::RotateRaderClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::RotateRaderCounterClockwise => {
                        tank_velocity.angular_velocity = 1.0;
                    }
                    CommandType::FireWithPower => {}
                }
            }
        }

        if !grouped_commands.command_array.iter().any(|x| x > &0) {
            command_receiver.queue.remove(0);
        }
        println!("commands remaining {:?}", command_receiver.queue);
    }
}
