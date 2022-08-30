use crate::{CustomAsset, CustomAssetState};
use bevy::prelude::{info, AssetServer, Assets, Commands, Res, ResMut};
use ctsimlib::{
    c_client::{Client, ReaderClient},
    c_command::*,
};
use ctsimlibgraphics::*;

pub fn setup_web_tanks(
    mut state: ResMut<CustomAssetState>,
    mut commands: Commands,
    custom_assets: ResMut<Assets<CustomAsset>>,
    asset_server: Res<AssetServer>,
) {
    let custom_asset = custom_assets.get(&state.handle);
    if state.printed || custom_asset.is_none() {
        return;
    }

    let custom_asset = custom_asset.unwrap();

    let lines: Vec<String> = custom_asset.0.lines().map(|l| l.to_string()).collect();

    let tank_ids = lines[0]
        .split(",")
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    info!("players: {:?}", tank_ids);

    let mut n_commands = 0;

    for n in 0..tank_ids.len() {
        let c_lines: Vec<CCommand> = lines[(1 + n)..]
            .iter()
            .step_by(tank_ids.len())
            .map(|f| f.to_string().parse::<CCommand>().unwrap())
            .collect();
        if n_commands == 0 && c_lines.len() > 0 {
            n_commands = c_lines.len();
        }
        assert!(n_commands == c_lines.len());

        create_graphics_tank(
            &mut commands,
            n,
            Client {
                client: Box::new(ReaderClient { lines: c_lines }),
            },
            &asset_server,
        );
    }

    state.printed = true;
}
