use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ctsimlib::{
    s_apply_commands::apply_commands,
    s_physics::{physics, physics2},
    s_request_commands::request_commands,
    s_request_commands_by_event::request_commands_by_event,
    s_walls::setup_walls,
};

use ctviewer::*;
use s_graphics::*;
use s_load_tanks::*;
use s_request_debug_commands::*;
use s_tanks::*;

use bevy_prototype_lyon::prelude::*;


fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .init_resource::<CState>()
        .add_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .add_startup_system(load_tanks)
        .add_system(setup_tanks)
        // .add_system(print_on_load)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_walls)
        .add_stage(
            "request_commands",
            SystemStage::single_threaded().with_system(request_commands),
        )
        .add_stage(
            "request_debug_commands",
            SystemStage::single_threaded().with_system(request_debug_commands),
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
        )
        .add_stage(
            "update_health",
            SystemStage::single_threaded().with_system(update_health)
        )
        .run();
}

// fn print_ball_altitude(
//     mut positions: Query<(&Transform, &RigidBody, &mut Velocity)>,
//     keys: Res<Input<KeyCode>>,
// ) {
//     for (transform, _body, mut velocity) in &mut positions {
//         // info!("Ball altitude: {}", transform.translation.y);

//         let mut vel = Vec2::ZERO;
//         let mut ang = 0.0;
//         if keys.pressed(KeyCode::W) {
//             let dir = transform.rotation * Vec3::X;

//             vel.x += 100.0 * dir.x;
//             vel.y += 100.0 * dir.y;
//         }
//         if keys.pressed(KeyCode::S) {
//             let dir = transform.rotation * Vec3::X;
//             vel.x -= 100.0 * dir.x;
//             vel.y -= 100.0 * dir.y;
//         }
//         if keys.pressed(KeyCode::A) {
//             ang += 0.125 * std::f32::consts::PI;
//         }
//         if keys.pressed(KeyCode::D) {
//             ang -= 0.125 * std::f32::consts::PI;
//         }
//         velocity.linvel = vel;

//         velocity.angvel = ang;
//     }
// }
