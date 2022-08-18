use bevy::prelude::*;

use ctsimlib::{
    c_client::{Client, ReaderClient},
    // c_collider::CCollider,
    c_command::*,
    c_tank::create_tank,
    // c_velocity::{CVelocity, TankVelocity},
};

use crate::{CState, CustomAsset};

pub fn load_tanks(mut state: ResMut<CState>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("sim.txt");
}
pub fn setup_tanks(
    mut state: ResMut<CState>,
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

    let tank_ids = lines[0]
        .split(",")
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    info!("players: {:?}", tank_ids);

    let mut n_commands = 0;

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

        create_tank(
            &mut commands,
            n,
            Client {
                client: Box::new(ReaderClient { lines: c_lines }),
            },
        );
    }

    state.printed = true;
}
