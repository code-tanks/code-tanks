use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::collision_mask;

pub fn setup_walls(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 50.0))
        .insert(CollisionGroups::new(
            collision_mask::WALL,
            collision_mask::ALL,
        ))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    // commands
    //     .spawn()
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::ball(50.0))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}
