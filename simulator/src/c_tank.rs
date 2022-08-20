use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    c_client::Client, c_command::CommandSource, c_event::EventSink, c_health::Health,
    c_render::Render, collision_mask,
};

#[derive(Component)]
pub struct Tank {}

pub fn create_tank(
    commands: &mut Commands,
    n: usize,
    client: Client,
    asset_server: &Res<AssetServer>,
    // texture_atlas_handle: &Handle<TextureAtlas>,
) {
    commands
        .spawn()
        .insert(Render::as_tank())
        .insert(Health {})
        .insert(CommandSource::default())
        .insert(EventSink::default())
        .insert(GravityScale(0.0))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Collider::cuboid(30.0, 50.0))
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
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("tankBody_red.png"),
            transform: Transform::from_xyz(150.0 * (n as f32) + 10.0, 300.0, 0.0),
            ..Default::default()
        })
        // .insert_bundle(TransformBundle::from(Transform::from_xyz(
        //     150.0 * (n as f32) + 10.0,
        //     300.0,
        //     0.0,
        // )))
        .insert(client);
}
