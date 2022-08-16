use bevy::prelude::*;

use crate::{
    c_client::{Client, DummyClient},
    c_command::CommandSource,
    c_event::EventSink,
    c_health::Health,
    c_render::Render,
    c_scanner::Scanner,
    CState,
};

pub fn setup_tanks(state: Res<CState>, mut commands: Commands) {
    for _url in state.tanks.iter() {
        commands
            .spawn()
            .insert(Render::as_tank())
            .insert(Health {})
            // .insert(Position {
            //     x: 0.0,
            //     y: 0.0,
            //     rotation: 0.0,
            // })
            // .insert(CVelocity { velocity: 0.0 })
            // .insert(TankVelocity {
            //     angular_velocity: 0.0,
            //     gun_angular_velocity: 0.0,
            //     radar_angular_velocity: 0.0,
            // })
            // .insert(CCollider::tank())
            .insert(CommandSource::default())
            .insert(Client {
                client: Box::new(DummyClient {}),
            })
            .insert(Scanner {})
            .insert(EventSink::default());
        // .insert(TankUtilities {})
    }
}
