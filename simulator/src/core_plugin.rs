use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    s_apply_commands::apply_commands, s_physics::*, s_request_commands::request_commands,
    s_request_commands_by_event::request_commands_by_event,
};
pub struct CoreCTPlugin;

impl Plugin for CoreCTPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_stage(
                "request_commands",
                SystemStage::single_threaded().with_system(request_commands),
            )
            .add_stage(
                "apply_commands",
                SystemStage::single_threaded().with_system(apply_commands),
            )
            .add_stage(
                "physics2",
                SystemStage::single_threaded().with_system(physics2),
            )
            .add_stage(
                "physics",
                SystemStage::single_threaded().with_system(physics),
            )
            .add_stage(
                "publish_events",
                SystemStage::single_threaded().with_system(request_commands_by_event),
            );
    }
}
