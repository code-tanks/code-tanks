use crate::{CState, CustomAsset};
use bevy::{
    prelude::{
        default, info, AssetServer, Assets, BuildChildren, Camera2dBundle, Children, Color,
        Commands, Component, Quat, Query, Res, ResMut, SpatialBundle, Transform, Vec2, Visibility,
        With,
    },
    sprite::SpriteBundle,
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, Path, ShapePath, StrokeMode},
    shapes::{self, RectangleOrigin},
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Ccd, Collider, ColliderMassProperties, CollisionGroups, Damping, GravityScale,
    Restitution, RigidBody, Sleeping, Velocity,
};
use ctsimlib::{
    c_client::{Client, ReaderClient},
    // c_collider::CCollider,
    c_command::*,
    c_event::EventSink,
    // c_velocity::{CVelocity, TankVelocity},
    c_health::Health,
    c_tank::Tank,
    collision_mask,
    CCollider,
    CollisionType,
};

pub fn update_health(q_parent: Query<(&Health, &Children)>, mut q_child: Query<&mut Path>) {
    for (health, children) in q_parent.iter() {
        // `children` is a collection of Entity IDs
        for &child in children.iter() {
            // get the health of each child unit
            let mut path = q_child.get_mut(child).unwrap();

            let polygon = shapes::Rectangle {
                extents: Vec2::new(
                    50.0 * (health.val as f32) / (Health::MAX_HEALTH as f32),
                    3.0,
                ),
                origin: RectangleOrigin::default(),
            };

            *path = ShapePath::build_as(&polygon);
            // do something
        }
    }
}

// use bevy_prototype_lyon::prelude::*;

pub fn setup_tanks(
    mut state: ResMut<CState>,
    mut commands: Commands,
    custom_assets: ResMut<Assets<CustomAsset>>,
    asset_server: Res<AssetServer>,
) {
    let custom_asset = custom_assets.get(&state.handle);
    if state.printed || custom_asset.is_none() {
        return;
    }

    let custom_asset = custom_asset.unwrap();

    // info!("Custom asset loaded: {:?}", custom_asset);
    let lines: Vec<String> = custom_asset.0.lines().map(|l| l.to_string()).collect();
    // while let Some(line) = reader.read_line(&mut buffer) {
    //     info!("{}", line?.trim());
    // }

    let tank_ids = lines[0]
        .split(",")
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    info!("players: {:?}", tank_ids);

    let mut n_commands = 0;

    commands.spawn_bundle(Camera2dBundle::default());

    // let mut world = World::default();
    for n in 0..tank_ids.len() {
        let c_lines: Vec<CCommand> = lines[(1 + n)..]
            .iter()
            .step_by(tank_ids.len())
            .map(|f| f.to_string().parse::<CCommand>().unwrap())
            .collect();
        // info!("{} lines: {:?}", n + 1, c_lines);
        if n_commands == 0 && c_lines.len() > 0 {
            n_commands = c_lines.len();
        }
        assert!(n_commands == c_lines.len());

        // commands
        //     .spawn()
        //     .insert(Render::as_tank())
        //     .insert(Health {})
        //     // .insert(Position {
        //     //     x: 0.0,
        //     //     y: 0.0,
        //     //     rotation: 0.0,
        //     // })
        //     // .insert(CVelocity { velocity: 0.0 })
        //     // .insert(TankVelocity {
        //     //     angular_velocity: 0.0,
        //     //     gun_angular_velocity: 0.0,
        //     //     radar_angular_velocity: 0.0,
        //     // })
        //     // .insert(CCollider::tank())
        //     .insert(CommandSource::default())
        //     .insert(Client {
        //         client: Box::new(ReaderClient { lines: c_lines }),
        //     })
        //     .insert(Scanner {})
        //     .insert(EventSink::default())
        //     .insert(GravityScale(0.0))
        //     .insert(RigidBody::Dynamic)
        //     .insert(Collider::cuboid(30.0, 50.0))
        //     .insert(Restitution::coefficient(0.1))
        //     .insert(Damping {
        //         linear_damping: 0.5,
        //         angular_damping: 0.5,
        //     })
        //     .insert(Velocity {
        //         linvel: Vec2::new(0.0, 0.0),
        //         angvel: 0.0,
        //     })
        //     .insert_bundle(TransformBundle::from(Transform::from_xyz(
        //         150.0 * (n as f32) + 10.0,
        //         300.0,
        //         0.0,
        //     )))
        //     .insert(ColliderMassProperties::Mass(1.0))
        //     .insert(ColliderMassProperties::Density(1.0));

        // create_tank(
        //     &mut commands,
        //     n,
        //     Client {
        //         client: Box::new(ReaderClient { lines: c_lines }),
        //     },
        //     &asset_server,
        // );
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
                val: 50,
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
            .insert(Client {
                client: Box::new(ReaderClient { lines: c_lines }),
            })
            .insert_bundle(SpatialBundle {
                transform: Transform::from_xyz(150.0 * (n as f32) + 10.0, 0.0, 0.0),
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
                    extents: Vec2::new(50.0, 3.0),
                    origin: RectangleOrigin::default(),
                };

                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(Color::CYAN),
                            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                        },
                        Transform::from_xyz(0.0, 60.0 * (n as f32), 1.0),
                    ))
                    .insert(HealthBar {});
                // let vertices = vec![
                //     [-0.8660, 0.5000, 0f32],
                //     [0.8660, 0.5000, 0f32],
                //     [-1.0000, 0.0000, 0f32],
                //     [1.0000, 0.0000, 0f32],
                //     [-0.8660, -0.5000, 0f32],
                //     [0.8660, -0.5000, 0f32],
                // ];
                // let normals = vec![
                //     [0f32, 0f32, 1f32],
                //     [0f32, 0f32, 1f32],
                //     [0f32, 0f32, 1f32],
                //     [0f32, 0f32, 1f32],
                //     [0f32, 0f32, 1f32],
                //     [0f32, 0f32, 1f32],
                // ];
                // let uvs = vec![
                //     [0.0000, 0.0000],
                //     [0.0000, 0.0000],
                //     [0.0000, 0.0000],
                //     [0.0000, 0.0000],
                //     [0.0000, 0.0000],
                //     [0.0000, 0.0000],
                // ];
                // let indices = Indices::U16(vec![1, 0, 2, 3, 1, 2, 3, 2, 4, 3, 4, 5]);

                // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                // mesh.set_indices(Some(indices));

                // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                // mesh.insert_attribute(
                //     Mesh::ATTRIBUTE_POSITION,
                //     vec![[0.0, 0.0, 0.0], [0.0, 50.0, 0.0], [50.0, 50.0, 0.0]],
                // );
                // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]);
                // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[1.0, 1.0]; 3]);
                // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[1.0, 1.0, 1.0, 1.0]; 3]);
                // mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));

                // parent.spawn_bundle(MaterialMesh2dBundle {
                //     mesh: meshes.add(mesh).into(),
                //     transform: Transform::default().with_scale(Vec3::splat(128.)),
                //     material: materials.add(Color::GREEN.into()),
                //     ..default()
                // });

                // parent.spawn_bundle(Camera2dBundle::default());
            });
    }

    state.printed = true;
}

#[derive(Component)]
pub struct HealthBar {}
