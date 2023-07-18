use bevy::{ecs::schedule::ScheduleLabel, prelude::*, app::RunFixedUpdateLoop};
use bevy_rapier2d::prelude::*;

use crate::{
    s_apply_commands::apply_commands, s_bullet_physics::bullet_physics,
    s_radar_physics::radar_physics, s_request_commands::request_commands,
    s_request_commands_by_event::request_commands_by_event, s_setup_physics::setup_physics,
    s_tank_physics::*, TickState,
};
pub struct CoreCTPlugin;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestCommands;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ApplyCommands;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TankPhysics;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RadarPhysics;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BulletPhysics;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestCommandsByEvent;

impl Plugin for CoreCTPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickState { tick: 0 })
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // .edit_schedule(RunFixedUpdateLoop, |schedule| {
            //     schedule.configure_sets(
            //         (
            //             PhysicsSet::SyncBackend,
            //             PhysicsSet::SyncBackendFlush,
            //             PhysicsSet::StepSimulation,
            //             PhysicsSet::Writeback,
            //         )
            //             .chain(),
            //     );
            // })
            .add_systems(Startup, setup_physics)
            .add_systems(Update, (
                request_commands, apply_commands, tank_physics, radar_physics, bullet_physics).chain()
            );
            // .add_systems(
            //     // "request_commands",
            //     RequestCommands,
            //     request_commands, // SystemStage::single_threaded().with_system(request_commands),
            // )
            // .add_systems(
            //     ApplyCommands,
            //     apply_commands, // "apply_commands",
            //                     // SystemStage::single_threaded().with_system(apply_commands),
            // )
            // .add_systems(
            //     TankPhysics,
            //     tank_physics, // "tank_physics",
            //                   // SystemStage::single_threaded().with_system(tank_physics),
            // )
            // .add_systems(
            //     RadarPhysics,
            //     radar_physics, // "radar_physics",
            //                    // SystemStage::single_threaded().with_system(radar_physics),
            // )
            // .add_systems(
            //     BulletPhysics,
            //     bullet_physics, // "bullet_physics",
            //                     // SystemStage::single_threaded().with_system(bullet_physics),
            // )
            // .add_systems(
            //     RequestCommandsByEvent,
            //     request_commands_by_event, // "publish_events",
            //                                // SystemStage::single_threaded().with_system(request_commands_by_event),
            // );
    }
}
