pub mod c_client;
pub mod c_command_source;
pub mod c_event;
pub mod c_health;
pub mod c_tank;
pub mod c_radar_needs_update;
pub mod core_plugin;

pub mod s_apply_commands;
pub mod s_bullet_physics;
pub mod s_radar_physics;
pub mod s_request_commands;
pub mod s_request_commands_by_event;
pub mod s_setup_physics;
pub mod s_setup_walls;
pub mod s_tank_physics;
pub mod s_apply_history_transforms;
pub mod s_save_commands;

use std::process::Command;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{Component, Resource};

use bevy::prelude::*;

use crate::{
    // c_client::{Client, DockerClient},
    c_tank::Gun,
    c_tank::Radar,
    c_tank::Tank,
    c_tank::{DamageDealer, TankInfo},
};
use bevy_rapier2d::prelude::*;

use crate::{c_command_source::CommandSource, c_event::EventSink, c_health::Health};

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::{TypeUuid, TypePath},
    utils::BoxedFuture, prelude::{Handle},
};

// use ct_api::{Commands};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct CustomAsset(pub String);

#[derive(Default)]
pub struct CustomAssetLoader;

impl AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = CustomAsset(String::from_utf8(bytes.to_vec()).unwrap());
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}

#[derive(Default, Resource)]
pub struct CustomAssetState {
    pub handle: Handle<CustomAsset>,
    pub printed: bool,
}


#[derive(Default, Resource)]
pub struct TickState {
    pub count: u32,
}

impl TickState {
    // pub const MAXIMUM_SIMULATION_TICKS: u32 = 300 * 2; // 10 secs
    pub const DT: f32 = 1.0 / 60.0;
}

#[derive(Resource)]
pub struct MaxSimulationTicks(pub u32);

pub struct Game {}

impl Game {
    pub const WIDTH: f32 = 1024.;
    pub const HEIGHT: f32 = 640.0;
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SaveCommands;

pub enum CollisionMask {}

impl CollisionMask {
    pub const NONE: u32 = 0b0;
    pub const TANK: u32 = 0b1;
    pub const WALL: u32 = 0b1 << 1;
    pub const BULLET: u32 = 0b1 << 2;
    pub const RADAR: u32 = 0b1 << 3;
}

#[derive(Component)]
pub struct CCollider {
    pub collision_type: CollisionType,
}

#[derive(Debug, PartialEq)]
pub enum CollisionType {
    Bullet,
    Tank,
    Wall,
    Radar,
}

pub fn remove_tank(tank_container_name: &str) {
    Command::new("docker")
        .arg("rm")
        .arg("--force")
        .arg(tank_container_name)
        .output()
        .expect("failed to communicate with docker");
}

// pub fn run_tank(url: &str, game_url: &str, post_fix: usize) {
// let tank_container_name = format!("{}-{}-{}", game_url, url, post_fix);

pub fn run_tank(tank_container_name: &str, tank_image_name: &str, port: &str, no_internet: bool) {
    remove_tank(&tank_container_name);
    let output_raw = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg(if no_internet {
            "--network=no-internet"
        } else {
            "--network=bridge"
        })
        // .arg("--network=code-tanks_no-internet")
        .arg("-p")
        .arg(port)
        .arg("--name")
        .arg(&tank_container_name)
        // .arg("--label")
        // .arg("com.docker.compose.project=code-tanks")
        .arg(tank_image_name)
        .output()
        .expect("failed to communicate with docker");
    let result_raw = String::from_utf8_lossy(&output_raw.stdout);

    println!("run stdout:");
    println!("{}", result_raw);
    // tank_container_name
}

pub fn create_gun(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let mut t = Transform::from_xyz(x, y, 0.0);
    t.rotate_local_z(Tank::INITIAL_ROTATION);
    commands
        .spawn((
            Gun { locked: true },
            SpatialBundle {
                transform: t,
                visibility: Visibility::Visible,
                ..default()
            },
            Sensor,
            GravityScale(0.0),
            RigidBody::Dynamic,
            ColliderMassProperties::Mass(0.0),
            // ColliderMassProperties::Density(1.0),
            Collider::ball(5.0),
            Restitution::coefficient(0.0),
            CollisionGroups::new(
                Group::from_bits_truncate(CollisionMask::NONE),
                Group::from_bits_truncate(CollisionMask::NONE),
            ),
            Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
        ))
        .id()
}

pub fn create_radar(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let mut t = Transform::from_xyz(x, y, 0.0);
    t.rotate_local_z(Tank::INITIAL_ROTATION);

    commands
        .spawn((
            CCollider {
                collision_type: CollisionType::Radar,
            },
            Radar {
                locked: true,
                disabled: false,
            },
            SpatialBundle {
                transform: t,
                visibility: Visibility::Visible,
                ..default()
            },
            Sensor,
            GravityScale(0.0),
            RigidBody::Dynamic,
            ColliderMassProperties::Mass(0.0),
            // ColliderMassProperties::Density(1.0),
            Collider::triangle(
                Vec2::new(0.0, 0.0),
                Vec2::new(-25.0, Game::WIDTH + Game::HEIGHT),
                Vec2::new(25.0, Game::WIDTH + Game::HEIGHT),
            ),
            Restitution::coefficient(0.0),
            CollisionGroups::new(
                Group::from_bits_truncate(CollisionMask::RADAR),
                Group::from_bits_truncate(
                    CollisionMask::TANK | CollisionMask::BULLET | CollisionMask::WALL,
                ),
            ),
            Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
        ))
        .id()
}

pub fn create_base_tank(
    tank_info: &TankInfo,
    commands: &mut Commands,
    gun: Entity,
    radar: Entity,
    x: f32,
    y: f32,
    client: impl Component,
) -> Entity {
    let mut t = Transform::from_xyz(x, y, 0.0);
    t.rotate_local_z(Tank::INITIAL_ROTATION);
    commands
        .spawn((
            (
                ActiveEvents::COLLISION_EVENTS,
                CCollider {
                    collision_type: CollisionType::Tank,
                },
            ),
            // Sleeping::disabled(),
            // Ccd::enabled(),
            Tank {
                info: tank_info.clone(),
                cooldown: 0,
                gun,
                radar,
            },
            Health {
                val: Health::MAX_HEALTH,
            },
            DamageDealer { damage_dealt: 0 },
            CommandSource::default(),
            EventSink::default(),
            GravityScale(0.0),
            RigidBody::Dynamic,
            // ColliderMassProperties::Mass(1.0),
            ColliderMassProperties::Density(1.0),
            Collider::ball(Tank::RADIUS),
            (
                Restitution::coefficient(0.0),
                Friction {
                    coefficient: 0.,
                    combine_rule: CoefficientCombineRule::Min,
                },
            ),
            CollisionGroups::new(
                Group::from_bits_truncate(CollisionMask::TANK),
                Group::from_bits_truncate(
                    CollisionMask::TANK
                        | CollisionMask::BULLET
                        | CollisionMask::WALL
                        | CollisionMask::RADAR,
                ),
            ),
            (
                Damping {
                    linear_damping: 0.0,
                    angular_damping: 0.0,
                },
                Velocity {
                    linvel: Vec2::new(0.0, 0.0),
                    angvel: 0.0,
                },
            ),
            client,
            SpatialBundle {
                transform: t,
                visibility: Visibility::Visible,
                ..default()
            },
        ))
        .id()
}
