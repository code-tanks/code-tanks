use bevy::{prelude::{Res, Commands, AssetServer, Vec2, SpatialBundle, Transform, Visibility, default, BuildChildren, Quat, Color}, sprite::SpriteBundle};
use bevy_prototype_lyon::{
    prelude::{DrawMode, GeometryBuilder, FillMode, StrokeMode},
    shapes::{self, RectangleOrigin},
};
use bevy_rapier2d::prelude::{ActiveEvents, Sleeping, Ccd, GravityScale, RigidBody, ColliderMassProperties, Collider, Restitution, CollisionGroups, Damping, Velocity};
use ctsimlib::c_healthbar::HealthBar;

use crate::{
    c_client::{
        Client,
        DockerClient, // , DummyClient
    },
    c_tank::Tank,
    CCollider, CState, CollisionType,
};

use crate::{c_command::CommandSource, c_event::EventSink, c_health::Health, collision_mask};

pub fn setup_tanks(
    state: Res<CState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // for tank_id in state.tank_ids.iter() {
    //     commands
    //         .spawn()
    //         .insert(Render::as_tank())
    //         .insert(Health {})
    //         // .insert(Position {
    //         //     x: 0.0,
    //         //     y: 0.0,
    //         //     rotation: 0.0,
    //         // })
    //         // .insert(CVelocity { velocity: 0.0 })
    //         // .insert(TankVelocity {
    //         //     angular_velocity: 0.0,
    //         //     gun_angular_velocity: 0.0,
    //         //     radar_angular_velocity: 0.0,
    //         // })
    //         // .insert(CCollider::tank())
    //         .insert(CommandSource::default())
    //         .insert(Client {
    //             // client: Box::new(DummyClient {}),
    //             client: Box::new(DockerClient {
    //                 tank_id: tank_id.to_string(),
    //             }),
    //         })
    //         .insert(Scanner {})
    //         .insert(EventSink::default());
    //     // .insert(TankUtilities {})
    // }
    // let texture_handle = asset_server.load("spritesheet.png");
    // let texture_atlas = TextureAtlas:: //::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 1);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (i, tank_id) in state.tank_ids.iter().enumerate() {
        // create_tank(
        //     &mut commands,
        //     i,
        //     Client {
        //         // client: Box::new(DummyClient {}),
        //         client: Box::new(DockerClient {
        //             tank_id: tank_id.to_string(),
        //         }),
        //     },
        //     &asset_server, // &texture_atlas_handle,
        // );
        commands
            .spawn()
            // .insert(Render::as_tank())
            .insert(CCollider {
                collision_type: CollisionType::Tank,
            })
            .insert(ActiveEvents::COLLISION_EVENTS)
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
            // .insert_bundle(SpriteBundle {
            //     // texture: asset_server.load("tankBody_red.png"),
            //     sprite
            //     transform: Transform::from_xyz(150.0 * (n as f32) + 10.0, 300.0, 0.0),
            //     ..Default::default()
            // })
            .insert(Client {
                client: Box::new(DockerClient {
                    tank_id: tank_id.to_string(),
                }),
            })
            .insert_bundle(SpatialBundle {
                transform: Transform::from_xyz(150.0 * (i as f32) + 10.0, 0.0, 0.0),
                // transform: Transform::from_xyz(10.0, 20.0, 30.0),
                visibility: Visibility { is_visible: true },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    transform: Transform::from_rotation(Quat::from_rotation_z(
                        std::f32::consts::PI,
                    )),
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
}
