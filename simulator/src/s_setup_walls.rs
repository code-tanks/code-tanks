use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{CCollider, CollisionMask, CollisionType};

pub fn setup_walls(mut commands: Commands) {
    /* Create the ground. */

    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(500.0, 10.0),
        CollisionGroups::new(
            Group::from_bits_truncate(CollisionMask::WALL),
            Group::from_bits_truncate(
                CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)),
    ));
    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(500.0, 10.0),
        CollisionGroups::new(
            Group::from_bits_truncate(CollisionMask::WALL),
            Group::from_bits_truncate(
                CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(0.0, 300.0, 0.0)),
    ));

    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(10.0, 300.0),
        CollisionGroups::new(
            Group::from_bits_truncate(CollisionMask::WALL),
            Group::from_bits_truncate(
                CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(500.0, 0.0, 0.0)),
    ));
    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(10.0, 300.0),
        CollisionGroups::new(
            Group::from_bits_truncate(CollisionMask::WALL),
            Group::from_bits_truncate(
                CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(-500.0, 0.0, 0.0)),
    ));
}
