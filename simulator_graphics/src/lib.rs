use bevy::{app::RunFixedUpdateLoop, ecs::schedule::ScheduleLabel, prelude::*};
use bevy::{
    prelude::{
        default, shape, App, AssetServer, Assets, BuildChildren, Color, Commands, Component,
        IntoSystemConfigs, Mesh, Msaa, Plugin, PluginGroup, Quat, Res, ResMut, Resource, Startup,
        Transform, Vec2, Vec3,
    },
    render::render_resource::PrimitiveTopology,
    sprite::{Anchor, ColorMaterial, MaterialMesh2dBundle, Sprite, SpriteBundle},
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
    window::{PresentMode, Window, WindowPlugin},
};
// use bevy_prototype_lyon::{
//     prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
//     shapes::{self, RectangleOrigin},
// };
use c_healthbar::HealthBar;
use c_nametag::NameTag;
use c_tracks::Tracks;
use ctsimlib::{c_tank::Tank, s_request_commands::request_commands};
use s_request_debug_commands::request_debug_commands;
use s_spawn_tracks::spawn_tracks;
// use s_update_healthbar::update_healthbar;
// use s_spawn_tracks::spawn_tracks;
use s_update_nametag::update_nametag;
pub mod c_healthbar;
pub mod c_nametag;
pub mod c_particle;
pub mod c_tracks;
pub mod s_spawn_tracks;
pub mod s_update_tracks;
use ctsimlib::Game;
use s_on_added_bullet::on_added_bullet;
use s_update_tracks::update_tracks;
// use s_update_tracks::update_tracks;
pub mod s_on_added_bullet;
pub mod s_setup_graphics;
pub mod s_update_healthbar;
pub mod s_update_nametag;
use crate::s_setup_graphics::setup_graphics;
// use crate::s_update_healthbar::update_healthbar;
use bevy::ecs::entity::Entity;
use bevy::DefaultPlugins;
// use bevy_prototype_lyon::prelude::ShapePlugin;
// use bevy_rapier2d::prelude::RapierDebugRenderPlugin;

pub mod s_request_debug_commands;

use ctsimlib::s_setup_sim_tanks::{create_base_tank, create_gun, create_radar};

const TANK_BODY_IMAGES: &[&str] = &[
    "tankBody_red.png",
    "tankBody_green.png",
    "tankBody_blue.png",
    "tankBody_dark.png",
];

const TANK_BARREL_IMAGES: &[&str] = &[
    "tankRed_barrel1.png",
    "tankGreen_barrel1.png",
    "tankBlue_barrel1.png",
    "tankDark_barrel1.png",
];

pub fn create_environment(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    for x in 0..(Game::WIDTH as i32 / 64) {
        for y in 0..(Game::HEIGHT as i32 / 64) {
            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(
                    -(Game::WIDTH / 2.) + x as f32 * 64.,
                    (Game::HEIGHT / 2.) - y as f32 * 64.,
                    0.,
                ),
                sprite: Sprite {
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture: asset_server.load("tileSand1.png"),
                ..default()
            });
        }
    }
}

pub fn create_graphics_tank(
    commands: &mut Commands,
    tank_index: usize,
    client: impl Component,
    asset_server: &Res<AssetServer>,
    tank_id: String,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let x = 150.0 * (tank_index as f32) + 10.0;
    let y = 0.0;

    let gun = create_gun(commands, x, y);
    let mut gun = commands.entity(gun);
    let mut t = Transform::from_xyz(x, y, 1.0);
    t.rotate_local_z((Tank::INITIAL_ROTATION).to_radians());

    // let mut t2 = t.clone();
    gun.insert(SpriteBundle {
        transform: {
            let mut j = t;
            j.translation.z = 2.1;
            j
        },
        texture: asset_server.load(TANK_BARREL_IMAGES[tank_index % TANK_BARREL_IMAGES.len()]),
        sprite: Sprite {
            anchor: Anchor::Custom(Vec2::new(0.0, -0.35)),
            flip_y: true,
            ..default()
        },
        ..default()
    });
    let gun = gun.id();

    let radar = create_radar(commands, x, y);
    let mut radar = commands.entity(radar);
    // radar.insert(GeometryBuilder::build_as(
    //     &shapes::Polygon {
    //         points: vec![
    //             Vec2::new(0.0, 0.0),
    //             Vec2::new(25.0, Game::WIDTH + Game::HEIGHT),
    //             Vec2::new(-25.0, Game::WIDTH + Game::HEIGHT),
    //         ],
    //         closed: true,
    //     },
    //     DrawMode::Fill(FillMode::color(Color::rgba(1., 1., 1., 0.1))),
    //     t,
    // ));
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.0, 0.],
            [25.0, Game::WIDTH + Game::HEIGHT, 0.],
            [-25.0, Game::WIDTH + Game::HEIGHT, 0.],
        ],
    );
    radar.insert(MaterialMesh2dBundle {
        // mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: t,
        ..default()
    });
    let radar = radar.id();

    let mut k = Transform::from_rotation(Quat::from_rotation_z(0.0));
    k.translation.z = 1.;

    let tank = create_base_tank(
        tank_id.to_string(),
        tank_index,
        commands,
        gun,
        radar,
        x,
        y,
        client,
    );
    let tank = commands
        .entity(tank)
        .insert(Tracks {
            current_distant: Tracks::MAX_DISTANCE,
            last_pos: t,
        })
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: k,
                texture: asset_server.load(TANK_BODY_IMAGES[tank_index % TANK_BODY_IMAGES.len()]),
                ..default()
            });
        })
        .id();

    // commands.spawn((
    //     GeometryBuilder::build_as(
    //         &shapes::Rectangle {
    //             extents: Vec2::new(HealthBar::MAX_WIDTH, HealthBar::MAX_HEIGHT),
    //             origin: RectangleOrigin::BottomLeft,
    //         },
    //         DrawMode::Outlined {
    //             fill_mode: FillMode::color(Color::GREEN),
    //             outline_mode: StrokeMode::new(Color::BLACK, 1.0),
    //         },
    //         Transform::from_xyz(x - HealthBar::MAX_WIDTH / 2.0, y - Tank::RADIUS - 10.0, 1.0),
    //     ),
    //     HealthBar {
    //         tank,
    //         is_backdrop: true,
    //     },
    // ));
    // commands.spawn((
    //     GeometryBuilder::build_as(
    //         &shapes::Rectangle {
    //             extents: Vec2::new(HealthBar::MAX_WIDTH, HealthBar::MAX_HEIGHT),
    //             origin: RectangleOrigin::BottomLeft,
    //         },
    //         DrawMode::Outlined {
    //             fill_mode: FillMode::color(Color::GREEN),
    //             outline_mode: StrokeMode::new(Color::BLACK, 1.0),
    //         },
    //         Transform::from_xyz(x - HealthBar::MAX_WIDTH / 2.0, y - Tank::RADIUS - 10.0, 1.0),
    //     ),
    //     HealthBar {
    //         tank,
    //         is_backdrop: false,
    //     },
    // ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("{}-{}", tank_id, tank_index),
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 12.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::Center),
            // We align text to the top-left, so this transform is the top-left corner of our text. The
            // box is centered at box_position, so it is necessary to move by half of the box size to
            // keep the text in the box.
            transform: Transform::from_xyz(x, y - Tank::RADIUS - 10.0, 1.0),
            ..default()
        },
        NameTag { tank },
    ));
    tank
}

#[derive(Resource, Default, Debug)]
pub struct DebugToggle {
    is_on: bool,
    index: usize,
}

pub struct CoreCTGraphicsPlugin;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestDebugCommands;

impl Plugin for CoreCTGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            // .insert_resource(Msaa { samples: 4 })
            .insert_resource(DebugToggle {
                is_on: false,
                index: 0,
            })
            // .insert_resource( Window {
            //         title: "Code Tanks".to_string(),
            //         width: Game::WIDTH,
            //         height: Game::HEIGHT,
            //         resizable: false,
            //         present_mode: PresentMode::AutoVsync,
            //         ..default()
            //     },
            //     ..default()
            // }))
            // .add_plugins(ShapePlugin)
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_graphics)
            .add_systems(
                Update,
                request_debug_commands.after(request_commands), // "request_commands",
                                                                // "request_debug_commands",
                                                                // SystemStage::single_threaded().with_system(request_debug_commands),
            )
            // .add_systems(
            //     UpdateHealthbar,
            //     update_healthbar
            //     // "update_healthbar",
            //     // SystemStage::single_threaded().with_system(update_healthbar),
            // )
            .add_systems(
                Update,
                // update_healthbar,
                (update_nametag, on_added_bullet, spawn_tracks, update_tracks).chain(), // "on_added_bullet",
                                                                                        // SystemStage::single_threaded().with_system(on_added_bullet),
            );
        // .add_systems(
        //     SpawnTracks,
        //     spawn_tracks
        // //     "spawn_tracks",
        // //     SystemStage::single_threaded().with_system(spawn_tracks),
        // );
        // .add_systems(
        //     UpdateTracks,
        //     update_tracks
        //     // "update_tracks",
        //     // SystemStage::single_threaded().with_system(update_tracks),
        // );
    }
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateHealthbar;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateNametag;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnAddedBullet;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpawnTracks;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateTracks;
