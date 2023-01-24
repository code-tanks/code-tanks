use bevy::{
    prelude::{
        default, App, AssetServer, BuildChildren, Color, Commands, Component, Msaa, Plugin, Quat,
        Res, Transform, Vec2, PluginGroup,
    },
    sprite::SpriteBundle,
    text::{Text, Text2dBundle, TextAlignment, TextStyle}, window::{WindowDescriptor, WindowPlugin, PresentMode},
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
    shapes::{self, RectangleOrigin},
};
use c_healthbar::HealthBar;
use c_nametag::NameTag;
use ctsimlib::c_tank::Tank;
use s_update_nametag::update_nametag;
pub mod c_healthbar;
pub mod c_nametag;
pub mod s_graphics;
pub mod s_update_healthbar;
pub mod s_update_nametag;
use crate::s_graphics::setup_graphics;
use crate::s_update_healthbar::update_healthbar;
use bevy::ecs::entity::Entity;
use bevy::ecs::schedule::SystemStage;
use bevy::DefaultPlugins;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use ctsimlib::s_request_debug_commands::request_debug_commands;
use ctsimlib::s_setup_sim_tanks::{create_base_tank, create_gun, create_radar};

pub fn create_graphics_tank(
    commands: &mut Commands,
    i: usize,
    client: impl Component,
    asset_server: &Res<AssetServer>,
    tank_id: String,
) -> Entity {
    let x = 150.0 * (i as f32) + 10.0;
    let y = 0.0;

    let gun = create_gun(commands, x, y);
    let mut gun = commands.entity(gun);
    gun.insert(SpriteBundle {
        transform: Transform::from_xyz(x, y, 0.0),
        texture: asset_server.load("tankRed_barrel1.png"),
        ..default()
    });
    let gun = gun.id();

    let radar = create_radar(commands, x, y);
    let mut radar = commands.entity(radar);
    radar.insert(SpriteBundle {
        transform: Transform::from_xyz(x, y, 0.0),
        texture: asset_server.load("shotLarge.png"),
        ..default()
    });
    let radar = radar.id();

    let tank = create_base_tank(commands, gun, radar, x, y, client);
    let tank = commands
        .entity(tank)
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
                texture: asset_server.load("tankBody_red.png"),
                ..default()
            });
        })
        .id();

    commands.spawn((
        GeometryBuilder::build_as(
            &shapes::Rectangle {
                extents: Vec2::new(HealthBar::MAX_WIDTH, HealthBar::MAX_HEIGHT),
                origin: RectangleOrigin::BottomLeft,
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GREEN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::from_xyz(x - HealthBar::MAX_WIDTH / 2.0, y - Tank::RADIUS, 1.0),
        ),
        HealthBar {
            tank,
            is_backdrop: true,
        },
    ));
    commands.spawn((
        GeometryBuilder::build_as(
            &shapes::Rectangle {
                extents: Vec2::new(HealthBar::MAX_WIDTH, HealthBar::MAX_HEIGHT),
                origin: RectangleOrigin::BottomLeft,
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GREEN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::from_xyz(x - HealthBar::MAX_WIDTH / 2.0, y - Tank::RADIUS, 1.0),
        ),
        HealthBar {
            tank,
            is_backdrop: false,
        },
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                &tank_id[tank_id.find("-").unwrap() + 1..],
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 12.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::TOP_CENTER),
            // We align text to the top-left, so this transform is the top-left corner of our text. The
            // box is centered at box_position, so it is necessary to move by half of the box size to
            // keep the text in the box.
            transform: Transform::from_xyz(x, y - Tank::RADIUS, 1.0),
            ..default()
        },
        NameTag { tank },
    ));
    tank
}

pub struct CoreCTGraphicsPlugin;

impl Plugin for CoreCTGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Code Tanks".to_string(),
                    width: 1000.,
                    height: 600.,
                    resizable: false,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                },
                ..default()
            }))
            .add_plugin(ShapePlugin)
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_graphics)
            .add_stage_after(
                "request_commands",
                "request_debug_commands",
                SystemStage::single_threaded().with_system(request_debug_commands),
            )
            .add_stage(
                "update_healthbar",
                SystemStage::single_threaded().with_system(update_healthbar),
            )
            .add_stage(
                "update_nametag",
                SystemStage::single_threaded().with_system(update_nametag),
            );
    }
}
