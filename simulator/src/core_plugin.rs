use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    s_apply_commands::apply_commands, s_physics::*, s_request_commands::request_commands,
    s_request_commands_by_event::request_commands_by_event, TickState,
};
pub struct CoreCTPlugin;

impl Plugin for CoreCTPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickState {
                tick: 0,
            })
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_stage(
                "request_commands",
                SystemStage::single_threaded().with_system(request_commands),
            )
            .add_stage(
                "apply_commands",
                SystemStage::single_threaded().with_system(apply_commands),
            )
            .add_stage(
                "tank_physics",
                SystemStage::single_threaded().with_system(tank_physics),
            )
            .add_stage(
                "radar_physics",
                SystemStage::single_threaded().with_system(radar_physics),
            )
            .add_stage(
                "bullet_physics",
                SystemStage::single_threaded().with_system(bullet_physics),
            )
            .add_stage(
                "publish_events",
                SystemStage::single_threaded().with_system(request_commands_by_event),
            );
    }
}
