use bevy::prelude::*;
use ct_api::CCommands;

use crate::c_command::CommandSource;

pub fn request_debug_commands(mut query: Query<&mut CommandSource>, keys: Res<Input<KeyCode>>) {
    for mut command_source in &mut query {
        let mut grouped_commands = command_source.queue.remove(0);

        if keys.pressed(KeyCode::W) {
            // info!("W pressed");
            grouped_commands = grouped_commands | CCommands::MOVE_FORWARD;
        }
        if keys.pressed(KeyCode::S) {
            // info!("S pressed");
            grouped_commands = grouped_commands | CCommands::MOVE_BACKWARD;
        }
        if keys.pressed(KeyCode::A) {
            // info!("A pressed");
            grouped_commands = grouped_commands | CCommands::ROTATE_TANK_COUNTER_CLOCKWISE;
        }
        if keys.pressed(KeyCode::D) {
            // info!("D pressed");
            grouped_commands = grouped_commands | CCommands::ROTATE_TANK_CLOCKWISE;
        }
        if keys.pressed(KeyCode::Space) {
            // info!("Space pressed");
            grouped_commands = grouped_commands | CCommands::FIRE;
        }
        if keys.pressed(KeyCode::E) {
            // info!("E pressed");
            grouped_commands = grouped_commands | CCommands::ROTATE_GUN_CLOCKWISE;
        }
        if keys.pressed(KeyCode::Q) {
            // info!("Q pressed");
            grouped_commands = grouped_commands | CCommands::ROTATE_GUN_COUNTER_CLOCKWISE;
        }
        if keys.pressed(KeyCode::X) {
            // info!("X pressed");
            grouped_commands = grouped_commands | CCommands::ROTATE_RADAR_CLOCKWISE;
        }
        if keys.pressed(KeyCode::Z) {
            // info!("Z pressed");
            grouped_commands = grouped_commands | CCommands::ROTATE_RADAR_COUNTER_CLOCKWISE;
        }
        if keys.pressed(KeyCode::R) {
            // info!("R pressed");
            grouped_commands = grouped_commands | CCommands::LOCK_GUN;
        }
        if keys.pressed(KeyCode::T) {
            // info!("T pressed");
            grouped_commands = grouped_commands | CCommands::UNLOCK_GUN;
        }
        if keys.pressed(KeyCode::F) {
            // info!("F pressed");
            grouped_commands = grouped_commands | CCommands::LOCK_RADAR;
        }
        if keys.pressed(KeyCode::G) {
            // info!("G pressed");
            grouped_commands = grouped_commands | CCommands::UNLOCK_RADAR;
        }
        command_source.queue.insert(0, grouped_commands);
    }
}
