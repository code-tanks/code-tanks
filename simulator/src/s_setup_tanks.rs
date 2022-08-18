use bevy::prelude::*;

use crate::{
    c_client::{
        Client,
        DockerClient, // , DummyClient
    },
    c_tank::create_tank,
    CState,
};

pub fn setup_tanks(state: Res<CState>, mut commands: Commands) {
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
        );
    }
}
