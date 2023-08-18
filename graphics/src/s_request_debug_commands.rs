use bevy::prelude::*;
use ct_api::Commands;
use ctengine::c_command_source::CommandSource;

use crate::DebugToggle;

pub fn request_debug_commands(
    mut debug_toggle: ResMut<DebugToggle>,
    mut query: Query<&mut CommandSource>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Grave) {
        debug_toggle.is_on = !debug_toggle.is_on;
    }

    if !debug_toggle.is_on {
        return;
    }

    if keys.just_pressed(KeyCode::Key1) {
        debug_toggle.index = 0;
    } else if keys.just_pressed(KeyCode::Key2) {
        debug_toggle.index = 1;
    } else if keys.just_pressed(KeyCode::Key3) {
        debug_toggle.index = 2;
    } else if keys.just_pressed(KeyCode::Key4) {
        debug_toggle.index = 3;
    }

    if keys.just_pressed(KeyCode::Grave) && debug_toggle.is_on {
        println!("debug target {}", debug_toggle.index);
    }

    let mut command_sources = query.iter_mut().collect::<Vec<_>>();

    if debug_toggle.index >= command_sources.len() {
        return;
    }

    let command_source = &mut command_sources[debug_toggle.index];

    let mut grouped_commands = command_source.queue[0];

    if keys.pressed(KeyCode::W) {
        // info!("W pressed");
        grouped_commands |= Commands::MOVE_FORWARD;
    }
    if keys.pressed(KeyCode::S) {
        // info!("S pressed");
        grouped_commands |= Commands::MOVE_BACKWARD;
    }
    if keys.pressed(KeyCode::A) {
        // info!("A pressed");
        grouped_commands |= Commands::ROTATE_TANK_COUNTER_CLOCKWISE;
    }
    if keys.pressed(KeyCode::D) {
        // info!("D pressed");
        grouped_commands |= Commands::ROTATE_TANK_CLOCKWISE;
    }
    if keys.pressed(KeyCode::Space) {
        // info!("Space pressed");
        grouped_commands |= Commands::FIRE;
    }
    if keys.pressed(KeyCode::E) {
        // info!("E pressed");
        grouped_commands |= Commands::ROTATE_GUN_CLOCKWISE;
    }
    if keys.pressed(KeyCode::Q) {
        // info!("Q pressed");
        grouped_commands |= Commands::ROTATE_GUN_COUNTER_CLOCKWISE;
    }
    if keys.pressed(KeyCode::X) {
        // info!("X pressed");
        grouped_commands |= Commands::ROTATE_RADAR_CLOCKWISE;
    }
    if keys.pressed(KeyCode::Z) {
        // info!("Z pressed");
        grouped_commands |= Commands::ROTATE_RADAR_COUNTER_CLOCKWISE;
    }
    if keys.pressed(KeyCode::R) {
        // info!("R pressed");
        grouped_commands |= Commands::LOCK_GUN;
    }
    if keys.pressed(KeyCode::T) {
        // info!("T pressed");
        grouped_commands |= Commands::UNLOCK_GUN;
    }
    if keys.pressed(KeyCode::F) {
        // info!("F pressed");
        grouped_commands |= Commands::LOCK_RADAR;
    }
    if keys.pressed(KeyCode::G) {
        // info!("G pressed");
        grouped_commands |= Commands::UNLOCK_RADAR;
    }
    command_source.queue[0] = grouped_commands;
}
