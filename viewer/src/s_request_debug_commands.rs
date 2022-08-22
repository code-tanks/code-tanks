use bevy::prelude::*;
use ctsimlib::c_command::{CCommands, CommandSource};

pub fn request_debug_commands(mut query: Query<&mut CommandSource>, keys: Res<Input<KeyCode>>) {
    for mut command_source in &mut query {
        // info!("Ball altitude: {}", transform.translation.y);
        let mut grouped_commands = command_source.queue.remove(0);

        // let mut vel = Vec2::ZERO;
        // let mut ang = 0.0;
        if keys.pressed(KeyCode::W) {
            info!("W pressed");
            grouped_commands = grouped_commands | CCommands::MOVE_FORWARD;
        }
        if keys.pressed(KeyCode::S) {
            info!("S pressed");
            grouped_commands = grouped_commands | CCommands::MOVE_BACKWARD;
        }
        if keys.pressed(KeyCode::A) {
            info!("A pressed");
            grouped_commands = grouped_commands | CCommands::ROTATE_TANK_COUNTER_CLOCKWISE;
        }
        if keys.pressed(KeyCode::D) {
            info!("D pressed");
            grouped_commands = grouped_commands | CCommands::ROTATE_TANK_CLOCKWISE;
        }

        command_source.queue.insert(0, grouped_commands);
        // velocity.linvel = vel;

        // velocity.angvel = ang;
    }
}
