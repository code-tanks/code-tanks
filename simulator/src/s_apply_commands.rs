use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::c_command::{CCommands, CommandSource};

pub fn apply_commands(
    mut query: Query<(
        &mut CommandSource,
        &Transform,
        &RigidBody,
        &mut Velocity, // &mut CVelocity,
                       // &mut TankVelocity,
                       // &Position,
    )>,
) {
    for (mut command_receiver, transform, _body, mut velocity) in &mut query {
        let grouped_commands = command_receiver.queue.remove(0);

        println!("apply_commands {:?}", grouped_commands);
        info!("apply_commands {:?}", grouped_commands);
        let mut vel = Vec2::ZERO;
        let mut ang = 0.0;
        if CCommands::MOVE_FORWARD & grouped_commands != 0 {
            let dir = transform.rotation * Vec3::Y;

            vel.x += 100.0 * dir.x;
            vel.y += 100.0 * dir.y;
        }
        if CCommands::MOVE_BACKWARD & grouped_commands != 0 {
            let dir = transform.rotation * Vec3::Y;
            vel.x -= 100.0 * dir.x;
            vel.y -= 100.0 * dir.y;
        }
        if CCommands::ROTATE_TANK_CLOCKWISE & grouped_commands != 0 {
            ang -= 0.125 * std::f32::consts::PI;
        }
        if CCommands::ROTATE_TANK_COUNTER_CLOCKWISE & grouped_commands != 0 {
            ang += 0.125 * std::f32::consts::PI;
        }

        velocity.linvel = vel;

        velocity.angvel = ang;

        println!("commands remaining {:?}", command_receiver.queue);
        info!("commands remaining {:?}", command_receiver.queue);
    }
}
