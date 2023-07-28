use crate::{CustomAsset, CustomAssetState, *};
use bevy::{prelude::{info, AssetServer, Assets, Commands, Res, ResMut, Mesh}, sprite::ColorMaterial};
use ct_api::Command;
use ctsimlib::{c_client::Client, c_tank::TankInfo};
use ctsimlibgraphics::*;

pub fn setup_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
      
) {

    create_environment(&mut commands, &asset_server);

}
