use bevy::{
    prelude::{
        default, App, AssetServer, BuildChildren, Color, Commands, Component, Msaa, Plugin, Quat,
        Res, Transform, Vec2,
    },
    sprite::SpriteBundle,
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
    shapes::{self, RectangleOrigin},
};
use ctsimlib::c_healthbar::HealthBar;
pub mod s_graphics;
pub mod s_update_health;
use crate::s_graphics::setup_graphics;
use crate::s_update_health::update_health;
use bevy::ecs::schedule::SystemStage;
use bevy::DefaultPlugins;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use ctsimlib::s_request_debug_commands::request_debug_commands;
use ctsimlib::s_setup_sim_tanks::{create_gun, create_radar, create_base_tank};

pub fn create_graphics_tank(
    commands: &mut Commands,
    i: usize,
    client: impl Component,
    asset_server: &Res<AssetServer>,
) {
    let x = 150.0 * (i as f32) + 10.0;
    let y = 0.0;

    let gun = create_gun(commands);
    let mut gun = commands.entity(gun);
    gun.insert_bundle(SpriteBundle {
        transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
        texture: asset_server.load("tankRed_barrel1.png"),
        ..default()
    });
    let gun = gun.id();

    let radar = create_radar(commands);
    let mut radar = commands.entity(radar);
    radar.insert_bundle(SpriteBundle {
        transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
        texture: asset_server.load("shotLarge.png"),
        ..default()
    });
    let radar = radar.id();

    let tank = create_base_tank(commands, gun, radar, x, y, client);
    commands.entity(tank).with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
            transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
            texture: asset_server.load("tankBody_red.png"),
            ..default()
        });
        let shape = shapes::Rectangle {
            extents: Vec2::new(38.0, 3.0),
            origin: RectangleOrigin::BottomLeft,
        };

        parent
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform::from_xyz(-19.0, -23.0, 1.0),
            ))
            .insert(HealthBar {});
    });
}

// pub fn create_tank(
//     commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
//     client: impl Component,
//     x: f32,
//     y: f32,
// ) {
//     let gun = commands
//         .spawn()
//         .insert(Gun { locked: true })
//         .insert_bundle(SpriteBundle {
//             transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
//             texture: asset_server.load("tankRed_barrel1.png"),
//             ..default()
//         })
//         .insert(Sensor)
//         .insert(GravityScale(0.0))
//         .insert(RigidBody::Dynamic)
//         .insert(ColliderMassProperties::Mass(1.0))
//         .insert(ColliderMassProperties::Density(1.0))
//         .insert(Collider::ball(5.0))
//         .insert(Restitution::coefficient(0.1))
//         .insert(CollisionGroups::new(
//             collision_mask::NONE,
//             collision_mask::NONE,
//         ))
//         .insert(Damping {
//             linear_damping: 0.0,
//             angular_damping: 0.0,
//         })
//         .insert(Velocity {
//             linvel: Vec2::new(0.0, 0.0),
//             angvel: 0.0,
//         })
//         .id();

//     let radar = commands
//         .spawn()
//         .insert(CCollider {
//             collision_type: CollisionType::Radar,
//         })
//         .insert(Radar { locked: true })
//         .insert_bundle(SpriteBundle {
//             transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
//             texture: asset_server.load("shotLarge.png"),
//             ..default()
//         })
//         .insert(Sensor)
//         .insert(ActiveEvents::COLLISION_EVENTS)
//         .insert(GravityScale(0.0))
//         .insert(RigidBody::Dynamic)
//         .insert(ColliderMassProperties::Mass(0.0))
//         .insert(ColliderMassProperties::Density(0.0))
//         .insert(Collider::triangle(
//             Vec2::new(0.0, 0.0),
//             Vec2::new(-25.0, 500.0),
//             Vec2::new(25.0, 500.0),
//         ))
//         .insert(Restitution::coefficient(0.0))
//         .insert(CollisionGroups::new(
//             collision_mask::RADAR,
//             collision_mask::TANK | collision_mask::BULLET | collision_mask::WALL,
//         ))
//         .insert(Damping {
//             linear_damping: 0.0,
//             angular_damping: 0.0,
//         })
//         .insert(Velocity {
//             linvel: Vec2::new(0.0, 0.0),
//             angvel: 0.0,
//         })
//         .id();

//     commands
//         .spawn()
//         .insert(ActiveEvents::COLLISION_EVENTS)
//         .insert(CCollider {
//             collision_type: CollisionType::Tank,
//         })
//         .insert(Sleeping::disabled())
//         .insert(Ccd::enabled())
//         .insert(Tank {
//             cooldown: 0,
//             gun: gun,
//             radar: radar,
//         })
//         .insert(Health {
//             val: Health::MAX_HEALTH,
//         })
//         .insert(CommandSource::default())
//         .insert(EventSink::default())
//         .insert(GravityScale(0.0))
//         .insert(RigidBody::Dynamic)
//         .insert(ColliderMassProperties::Mass(0.0))
//         .insert(ColliderMassProperties::Density(0.0))
//         .insert(Collider::cuboid(19.0, 23.0))
//         .insert(Restitution::coefficient(0.0))
//         .insert(CollisionGroups::new(
//             collision_mask::TANK,
//             collision_mask::TANK
//                 | collision_mask::BULLET
//                 | collision_mask::WALL
//                 | collision_mask::RADAR,
//         ))
//         .insert(Damping {
//             linear_damping: 0.5,
//             angular_damping: 0.5,
//         })
//         .insert(Velocity {
//             linvel: Vec2::new(0.0, 0.0),
//             angvel: 0.0,
//         })
//         .insert(client)
//         .insert_bundle(SpatialBundle {
//             transform: Transform::from_xyz(x, y, 0.0),
//             visibility: Visibility { is_visible: true },
//             ..default()
//         })
//         .with_children(|parent| {
//             parent.spawn_bundle(SpriteBundle {
//                 transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
//                 texture: asset_server.load("tankBody_red.png"),
//                 ..default()
//             });
//             let shape = shapes::Rectangle {
//                 extents: Vec2::new(38.0, 3.0),
//                 origin: RectangleOrigin::BottomLeft,
//             };

//             parent
//                 .spawn_bundle(GeometryBuilder::build_as(
//                     &shape,
//                     DrawMode::Outlined {
//                         fill_mode: FillMode::color(Color::GREEN),
//                         outline_mode: StrokeMode::new(Color::BLACK, 1.0),
//                     },
//                     Transform::from_xyz(-19.0, -23.0, 1.0),
//                 ))
//                 .insert(HealthBar {});
//         });
// }

pub struct CoreCTGraphicsPlugin;

impl Plugin for CoreCTGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .add_plugins(DefaultPlugins)
            .add_plugin(ShapePlugin)
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_graphics)
            .add_stage_after(
                "request_commands",
                "request_debug_commands",
                SystemStage::single_threaded().with_system(request_debug_commands),
            )
            .add_stage(
                "update_health",
                SystemStage::single_threaded().with_system(update_health),
            );
    }
}
