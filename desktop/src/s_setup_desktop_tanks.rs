use bevy::prelude::{AssetServer, Commands, Res};
use ctsimlib::c_client::DockerClient;
use ctsimlib::{c_client::Client, *};
use ctsimlibgraphics::*;

pub fn setup_desktop_tanks(
    tank_state: Res<TankIds>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (i, tank_id) in tank_state.tank_ids.iter().enumerate() {
        create_graphics_tank(
            &mut commands,
            i,
            Client {
                client: Box::new(DockerClient {
                    tank_id: tank_id.to_string(),
                }),
            },
            &asset_server,
            tank_id.to_string(),
        );
    }
}
