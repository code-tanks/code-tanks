// use crate::{CustomAsset, CustomAssetState, *};
use bevy::{prelude::*, sprite::Anchor};
use ct_api::Command;
use ctsimlib::{c_client::Client, c_tank::TankInfo, Game};
// use ctsimlibgraphics::*;

pub fn setup_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
      
) {

    // create_environment(&mut commands, &asset_server);
    for x in 0..(Game::WIDTH as i32 / 64) {
        for y in 0..(Game::HEIGHT as i32 / 64) {
            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(
                    -(Game::WIDTH / 2.) + x as f32 * 64.,
                    (Game::HEIGHT / 2.) - y as f32 * 64.,
                    0.,
                ),
                sprite: Sprite {
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture: asset_server.load("tileSand1.png"),
                ..default()
            });
        }
    }
}
