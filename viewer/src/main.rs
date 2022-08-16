use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ctsim::{
    s_apply_commands::apply_commands,
    // s_physics::physics,
    s_publish_events::publish_events,
    s_request_commands::request_commands,
    s_walls::setup_walls,
};

use ctviewer::*;
use s_graphics::*;
use s_tanks::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
            "apply_commands",
            SystemStage::single_threaded().with_system(apply_commands),
        )
        .add_stage(
            "publish_events",
            SystemStage::single_threaded().with_system(publish_events),
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
