use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{collision_mask, CCollider, CollisionType};

pub fn setup_walls(mut commands: Commands) {
    /* Create the ground. */

    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(500.0, 50.0),
        CollisionGroups::new(
            Group::from_bits_truncate(collision_mask::WALL),
            Group::from_bits_truncate(
                collision_mask::TANK | collision_mask::BULLET | collision_mask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)),
    ));

    /* Create the bouncing ball. */
    // commands
    //     .spawn()
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::ball(50.0))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}
