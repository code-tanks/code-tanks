use bevy::prelude::{AssetServer, Commands, Res, ResMut};
use ctsimlib::c_client::DesktopClient;
use ctsimlib::{c_client::Client, *};
use ctsimlibgraphics::*;

use crate::PORTS;

pub fn setup_desktop_tanks(
    mut state: ResMut<TickState>,
    tank_state: Res<TankIds>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    state.tick += 1; // TODO
    if state.tick > 1 {
        return;
    }

    for (i, tank_id) in tank_state.tank_ids.iter().enumerate() {
        create_graphics_tank(
            &mut commands,
            i,
            Client {
                client: Box::new(DesktopClient {
                    tank_id: tank_id.to_string(),
                    port: PORTS[i],
                }),
            },
            &asset_server,
            tank_id.to_string(),
        );
    }
}
