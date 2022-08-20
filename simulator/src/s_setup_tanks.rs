use bevy::prelude::*;

use crate::{
    c_client::{
        Client,
        DockerClient, // , DummyClient
    },
    c_tank::create_tank,
    CState,
};

pub fn setup_tanks(
    state: Res<CState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
        create_tank(
            &mut commands,
            i,
            Client {
                // client: Box::new(DummyClient {}),
                client: Box::new(DockerClient {
                    tank_id: tank_id.to_string(),
                }),
            },
            &asset_server, // &texture_atlas_handle,
        );
    }
}
