use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{game, CCollider, CollisionMask, CollisionType};

pub fn setup_walls(mut commands: Commands) {
    /* Create the ground. */

    const WALL_THICKNESS: f32 = 10.;

    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(game::WIDTH / 2., WALL_THICKNESS),
        CollisionGroups::new(
            Group::from_bits_truncate(CollisionMask::WALL),
            Group::from_bits_truncate(
                CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(0.0, -300.0 - WALL_THICKNESS, 0.0)),
    ));
    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(game::WIDTH / 2., WALL_THICKNESS),
        CollisionGroups::new(
            Group::from_bits_truncate(CollisionMask::WALL),
            Group::from_bits_truncate(
                CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(0.0, 300.0 + WALL_THICKNESS, 0.0)),
    ));

    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS, game::HEIGHT / 2.),
        CollisionGroups::new(
            Group::from_bits_truncate(CollisionMask::WALL),
            Group::from_bits_truncate(
                CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(500.0 + WALL_THICKNESS, 0.0, 0.0)),
    ));
    commands.spawn((
        CCollider {
            collision_type: CollisionType::Wall,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS, game::HEIGHT / 2.),
        CollisionGroups::new(
            Group::from_bits_truncate(CollisionMask::WALL),
            Group::from_bits_truncate(
                CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::RADAR,
            ),
        ),
        TransformBundle::from(Transform::from_xyz(-500.0 - WALL_THICKNESS, 0.0, 0.0)),
    ));
}
