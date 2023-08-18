use bevy::prelude::*;
use ctengine::{c_tank::AllTankInfo, create_gun, create_radar, c_client::Client, create_base_tank};

use crate::DockerClient;

pub fn setup_sim_tanks(state: Res<AllTankInfo>, mut commands: Commands) {

    for tank_info in state.all.iter() {
        let x = 150.0 * (tank_info.index as f32) + 10.0;
        let y = 0.0;

        let gun = create_gun(&mut commands, x, y);
        let radar = create_radar(&mut commands, x, y);

        let client = Client {
            client: Box::new(DockerClient {
                tank_container_name: tank_info.container_name.to_string(),
            }),
        };
        create_base_tank(tank_info, &mut commands, gun, radar, x, y, client);
    }
}
