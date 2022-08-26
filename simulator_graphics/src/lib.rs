use bevy::{
    prelude::{
        default, AssetServer, BuildChildren, Color, Commands, Component, Quat, Res, SpatialBundle,
        Transform, Vec2, Visibility,
    },
    sprite::SpriteBundle,
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
    shapes::{self, RectangleOrigin},
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Ccd, Collider, ColliderMassProperties, CollisionGroups, Damping, GravityScale,
    Restitution, RigidBody, Sleeping, Velocity,
};
use ctsimlib::{
    // c_client::{Client, ClientTrait, ReaderClient},
    // c_collider::CCollider,
    c_command::*,
    c_event::EventSink,
    // c_velocity::{CVelocity, TankVelocity},
    c_health::Health,
    c_healthbar::HealthBar,
    c_tank::Tank,
    collision_mask,
    CCollider,
    CollisionType,
};
pub mod s_graphics;
pub mod s_update_health;

pub fn create_tank(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    client: impl Component,
    x: f32,
    y: f32,
) {
    commands
        .spawn()
        // .insert(Render::as_tank())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CCollider {
            collision_type: CollisionType::Tank,
        })
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Tank { cooldown: 0 })
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
        // .insert_bundle(SpriteBundle {
        //     // texture: asset_server.load("tankBody_red.png"),
        //     sprite
        //     transform: Transform::from_xyz(150.0 * (n as f32) + 10.0, 300.0, 0.0),
        //     ..Default::default()
        // })
        .insert(client)
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            // transform: Transform::from_xyz(10.0, 20.0, 30.0),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                texture: asset_server.load("tank_red.png"),
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
