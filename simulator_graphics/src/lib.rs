use bevy::{
    prelude::{
        default, App, AssetServer, BuildChildren, Color, Commands, Component, Msaa, Plugin, Quat,
        Res, SpatialBundle, Transform, Vec2, Visibility,
    },
    sprite::SpriteBundle,
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
    shapes::{self, RectangleOrigin},
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Ccd, Collider, ColliderMassProperties, CollisionGroups, Damping, GravityScale,
    Restitution, RigidBody, Sleeping, Velocity, Sensor,
};
use ctsimlib::{
    c_command::*, c_event::EventSink, c_health::Health, c_healthbar::HealthBar, c_tank::Tank,
    collision_mask, CCollider, CollisionType,
};
pub mod s_graphics;
pub mod s_update_health;
use bevy::DefaultPlugins;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use bevy::ecs::schedule::SystemStage;
use crate::s_graphics::setup_graphics;
use ctsimlib::s_request_debug_commands::request_debug_commands;
use crate::s_update_health::update_health;
use ctsimlib::c_tank::Gun;
use ctsimlib::c_tank::Radar;

pub fn create_tank(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    client: impl Component,
    x: f32,
    y: f32,
) {
  

    let gun = commands
        .spawn()
        .insert(Gun { locked: true })
        .insert_bundle(SpriteBundle {
            transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
            texture: asset_server.load("tankRed_barrel1.png"),
            ..default()
        })
        .insert(Sensor)
        .insert(GravityScale(0.0))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Collider::ball(5.0))
        .insert(Restitution::coefficient(0.1))
        .insert(CollisionGroups::new(
            collision_mask::NONE,
            collision_mask::NONE,
        ))
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .id();

    let radar = commands
        .spawn()
        .insert(Radar { locked: true })
        .insert_bundle(SpriteBundle {
            transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
            texture: asset_server.load("shotLarge.png"),
            ..default()
        })
        .insert(Sensor)
        .insert(GravityScale(0.0))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Collider::ball(5.0))
        .insert(Restitution::coefficient(0.1))
        .insert(CollisionGroups::new(
            collision_mask::NONE,
            collision_mask::NONE,
        ))
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .id(); 

    commands
        .spawn()
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CCollider {
            collision_type: CollisionType::Tank,
        })
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Tank { cooldown: 0, gun: gun, radar: radar })
        .insert(Health {
            val: Health::MAX_HEALTH,
        })
        .insert(CommandSource::default())
        .insert(EventSink::default())
        .insert(GravityScale(0.0))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Collider::cuboid(19.0, 23.0))
        .insert(Restitution::coefficient(0.1))
        .insert(CollisionGroups::new(
            collision_mask::TANK,
            collision_mask::ALL,
        ))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(client)
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
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
