use bevy::prelude::*;

use crate::{
    c_client::{
        Client,
        DockerClient, // , DummyClient
    },
    c_tank::Gun,
    c_tank::Radar,
    c_tank::Tank,
    CCollider, CollisionType, TankIds,
};
use bevy_rapier2d::prelude::*;

use crate::{c_command::CommandSource, c_event::EventSink, c_health::Health, CollisionMask};

pub fn create_gun(commands: &mut Commands, x: f32, y: f32) -> Entity {
    commands
        .spawn((
            Gun { locked: true },
            SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                visibility: Visibility { is_visible: true },
                ..default()
            },
            Sensor,
            GravityScale(0.0),
            RigidBody::Dynamic,
            // ColliderMassProperties::Mass(0.0),
            ColliderMassProperties::Density(1.0),
            Collider::ball(5.0),
            Restitution::coefficient(0.0),
            CollisionGroups::new(
                Group::from_bits_truncate(CollisionMask::NONE),
                Group::from_bits_truncate(CollisionMask::NONE),
            ),
            // Damping {
            //     linear_damping: 0.0,
            //     angular_damping: 0.0,
            // },
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
        ))
        .id()
}

pub fn create_radar(commands: &mut Commands, x: f32, y: f32) -> Entity {
    commands
        .spawn((
            CCollider {
                collision_type: CollisionType::Radar,
            },
            Radar { locked: true },
            SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                visibility: Visibility { is_visible: true },
                ..default()
            },
            Sensor,
            GravityScale(0.0),
            RigidBody::Dynamic,
            // ColliderMassProperties::Mass(0.0),
            ColliderMassProperties::Density(1.0),
            Collider::triangle(
                Vec2::new(0.0, 0.0),
                Vec2::new(-25.0, 50.0),
                Vec2::new(25.0, 50.0),
            ),
            Restitution::coefficient(0.0),
            CollisionGroups::new(
                Group::from_bits_truncate(CollisionMask::RADAR),
                Group::from_bits_truncate(
                    CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::WALL,
                ),
            ),
            // Damping {
            //     linear_damping: 0.0,
            //     angular_damping: 0.0,
            // },
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
        ))
        .id()
}

pub fn create_base_tank(
    commands: &mut Commands,
    gun: Entity,
    radar: Entity,
    x: f32,
    y: f32,
    client: impl Component,
) -> Entity {
    commands
        .spawn((
            ActiveEvents::COLLISION_EVENTS,
            CCollider {
                collision_type: CollisionType::Tank,
            },
            // Sleeping::disabled(),
            // Ccd::enabled(),
            Tank {
                cooldown: 0,
                gun,
                radar,
            },
            Health {
                val: Health::MAX_HEALTH,
            },
            CommandSource::default(),
            EventSink::default(),
            GravityScale(0.0),
            RigidBody::Dynamic,
            // ColliderMassProperties::Mass(1.0),
            ColliderMassProperties::Density(1.0),
            Collider::cuboid(Tank::WIDTH, Tank::HEIGHT),
            Restitution::coefficient(0.0),
            CollisionGroups::new(
                Group::from_bits_truncate(CollisionMask::TANK),
                Group::from_bits_truncate(
                    CollisionMask::TANK
                        | CollisionMask::BULLET
                        | CollisionMask::WALL
                        | CollisionMask::RADAR,
                ),
            ),
            // Damping {
            //     linear_damping: 0.0,
            //     angular_damping: 0.0,
            // },
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            client,
            SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                visibility: Visibility { is_visible: true },
                ..default()
            },
        ))
        .id()
}

pub fn create_basic_tank(i: usize, client: impl Component, commands: &mut Commands) {
    let x = 150.0 * (i as f32) + 10.0;
    let y = 0.0;

    let gun = create_gun(commands, x, y);

    let radar = create_radar(commands, x, y);

    create_base_tank(commands, gun, radar, x, y, client);
}

pub fn setup_sim_tanks(state: Res<TankIds>, mut commands: Commands) {
    for (i, tank_id) in state.tank_ids.iter().enumerate() {
        create_basic_tank(
            i,
            Client {
                client: Box::new(DockerClient {
                    tank_id: tank_id.to_string(),
                }),
            },
            &mut commands,
        );

        // let gun = commands
        //     .spawn()
        //     .insert(Gun { locked: true })
        //     .insert_bundle(SpatialBundle {
        //         transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
        //         visibility: Visibility { is_visible: true },
        //         ..default()
        //     })
        //     .insert(Sensor)
        //     .insert(GravityScale(0.0))
        //     .insert(RigidBody::Dynamic)
        //     .insert(ColliderMassProperties::Mass(1.0))
        //     .insert(ColliderMassProperties::Density(1.0))
        //     .insert(Collider::ball(5.0))
        //     .insert(Restitution::coefficient(0.1))
        //     .insert(CollisionGroups::new(
        //         collision_mask::NONE,
        //         collision_mask::NONE,
        //     ))
        //     .insert(Damping {
        //         linear_damping: 0.0,
        //         angular_damping: 0.0,
        //     })
        //     .insert(Velocity {
        //         linvel: Vec2::new(0.0, 0.0),
        //         angvel: 0.0,
        //     })
        //     .id();

        // let radar = commands
        //     .spawn()
        //     .insert(CCollider {
        //         collision_type: CollisionType::Radar,
        //     })
        //     .insert(Radar { locked: true })
        //     .insert_bundle(SpatialBundle {
        //         transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
        //         visibility: Visibility { is_visible: true },
        //         ..default()
        //     })
        //     .insert(Sensor)
        //     .insert(ActiveEvents::COLLISION_EVENTS)
        //     .insert(GravityScale(0.0))
        //     .insert(RigidBody::Dynamic)
        //     .insert(ColliderMassProperties::Mass(0.0))
        //     .insert(ColliderMassProperties::Density(0.0))
        //     .insert(Collider::triangle(
        //         Vec2::new(0.0, 0.0),
        //         Vec2::new(-25.0, 500.0),
        //         Vec2::new(25.0, 500.0),
        //     ))
        //     .insert(Restitution::coefficient(0.0))
        //     .insert(CollisionGroups::new(
        //         collision_mask::RADAR,
        //         collision_mask::TANK | collision_mask::BULLET | collision_mask::WALL,
        //     ))
        //     .insert(Damping {
        //         linear_damping: 0.0,
        //         angular_damping: 0.0,
        //     })
        //     .insert(Velocity {
        //         linvel: Vec2::new(0.0, 0.0),
        //         angvel: 0.0,
        //     })
        //     .id();

        // commands
        //     .spawn()
        //     .insert(ActiveEvents::COLLISION_EVENTS)
        //     .insert(CCollider {
        //         collision_type: CollisionType::Tank,
        //     })
        //     .insert(Sleeping::disabled())
        //     .insert(Ccd::enabled())
        //     .insert(Tank {
        //         cooldown: 0,
        //         gun: gun,
        //         radar: radar,
        //     })
        //     .insert(Health {
        //         val: Health::MAX_HEALTH,
        //     })
        //     .insert(CommandSource::default())
        //     .insert(EventSink::default())
        //     .insert(GravityScale(0.0))
        //     .insert(RigidBody::Dynamic)
        //     .insert(ColliderMassProperties::Mass(1.0))
        //     .insert(ColliderMassProperties::Density(1.0))
        //     .insert(Collider::cuboid(19.0, 23.0))
        //     .insert(Restitution::coefficient(0.1))
        //     .insert(CollisionGroups::new(
        //         collision_mask::TANK,
        //         collision_mask::TANK
        //             | collision_mask::BULLET
        //             | collision_mask::WALL
        //             | collision_mask::RADAR,
        //     ))
        //     .insert(Damping {
        //         linear_damping: 0.5,
        //         angular_damping: 0.5,
        //     })
        //     .insert(Velocity {
        //         linvel: Vec2::new(0.0, 0.0),
        //         angvel: 0.0,
        //     })
        //     .insert(Client {
        //         client: Box::new(DockerClient {
        //             tank_id: tank_id.to_string(),
        //         }),
        //     })
        //     .insert_bundle(SpatialBundle {
        //         transform: Transform::from_xyz(x, 0.0, 0.0),
        //         // transform: Transform::from_xyz(10.0, 20.0, 30.0),
        //         visibility: Visibility { is_visible: true },
        //         ..default()
        //     });
    }
}
