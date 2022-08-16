use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy_rapier2d::{prelude::*, rapier::prelude::RigidBodyBuilder};
use ctsim::{
    c_client::{Client, ReaderClient},
    c_collider::CCollider,
    c_command::{CCommand, CommandSource},
    c_event::EventSink,
    c_health::Health,
    c_position::Position,
    c_render::Render,
    c_scanner::Scanner,
    c_velocity::{CVelocity, TankVelocity},
    s_apply_commands::apply_commands,
    s_physics::physics,
    s_publish_events::publish_events,
    s_request_commands::request_commands,
};
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<State>()
        .add_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .add_startup_system(setup)
        .add_system(setup_2)
        // .add_system(print_on_load)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        // .add_startup_system(setup_tanks)
        // .add_startup_
        .add_system(print_ball_altitude)
        .add_stage(
            "request_commands",
            SystemStage::single_threaded().with_system(request_commands),
        )
        .add_stage(
            "apply_commands",
            SystemStage::single_threaded().with_system(apply_commands),
        )
        .add_stage(
            "physics",
            SystemStage::single_threaded().with_system(physics),
        )
        // schedule.add_stage(
        //     "scanner",
        //     SystemStage::single_threaded().with_system(scanner),
        // );
        .add_stage(
            "publish_events",
            SystemStage::single_threaded().with_system(publish_events),
        )
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    // commands
    //     .spawn()
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::ball(50.0))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn print_ball_altitude(
    mut positions: Query<(&Transform, &mut RigidBody, &mut Velocity)>,
    keys: Res<Input<KeyCode>>,
) {
    for (transform, mut body, mut velocity) in &mut positions {
        info!("Ball altitude: {}", transform.translation.y);

        let mut vector_direction = Vec2::ZERO;
        if keys.pressed(KeyCode::W) {
            vector_direction.y += 100.0;
        }
        if keys.pressed(KeyCode::S) {
            vector_direction.y -= 100.0;
        }
        if keys.pressed(KeyCode::A) {
            vector_direction.x -= 100.0;
        }
        if keys.pressed(KeyCode::D) {
            vector_direction.x += 100.0;
        }
        velocity.linvel = vector_direction;
    }
}
#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct CustomAsset(String);

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

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .init_resource::<State>()
//         .add_asset::<CustomAsset>()
//         .init_asset_loader::<CustomAssetLoader>()
//         .add_startup_system(setup)
//         .add_system(print_on_load)
//         .run();
// }

#[derive(Default)]
struct State {
    handle: Handle<CustomAsset>,
    printed: bool,
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("sim.txt");
}
fn setup_2(
    mut state: ResMut<State>,
    mut commands: Commands,
    custom_assets: ResMut<Assets<CustomAsset>>,
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

    let s: usize = lines[0].parse::<usize>().unwrap();
    info!("# players: {}", s);

    let mut n_commands = 0;

    // let mut world = World::default();
    for n in 0..s {
        let c_lines: Vec<CCommand> = lines[(1 + n)..]
            .iter()
            .step_by(s)
            .map(|f| f.to_string().parse::<CCommand>().unwrap())
            .collect();
        // info!("{} lines: {:?}", n + 1, c_lines);
        if n_commands == 0 && c_lines.len() > 0 {
            n_commands = c_lines.len();
        }
        assert!(n_commands == c_lines.len());

        commands
            .spawn()
            .insert(Render::as_tank())
            .insert(Health {})
            .insert(Position {
                x: 0.0,
                y: 0.0,
                rotation: 0.0,
            })
            .insert(CVelocity { velocity: 0.0 })
            .insert(TankVelocity {
                angular_velocity: 0.0,
                gun_angular_velocity: 0.0,
                radar_angular_velocity: 0.0,
            })
            .insert(CCollider::tank())
            .insert(CommandSource::default())
            .insert(Client {
                client: Box::new(ReaderClient { lines: c_lines }),
            })
            .insert(Scanner {})
            .insert(EventSink::default())
            .insert(GravityScale(0.0))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(50.0))
            .insert(Restitution::coefficient(0.1))
            .insert(Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            })
            .insert(Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            })
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                100.0 * (n as f32) + 10.0,
                300.0,
                0.0,
            )))
            .insert(ColliderMassProperties::Mass(1.0))
            .insert(ColliderMassProperties::Density(1.0));
    }

    state.printed = true;
}
