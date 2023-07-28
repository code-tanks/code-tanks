pub mod s_apply_history_transforms;
pub mod s_load_tanks;
pub mod s_setup_web_tanks;
pub mod s_setup_ground;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::{TypeUuid, TypePath},
    utils::BoxedFuture, prelude::{Resource, Handle, Component},
};

use ct_api::{Command, Commands};
use ctsimlib::{c_client::ClientTrait, c_event::CTEvent};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct CustomAsset(String);

#[derive(Default)]
pub struct CustomAssetLoader;

impl AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = CustomAsset(String::from_utf8(bytes.to_vec()).unwrap());
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}

#[derive(Default, Resource)]
pub struct CustomAssetState {
    pub handle: Handle<CustomAsset>,
    pub printed: bool,
}

#[derive(Component)]
pub struct HistoryTransforms {
    pub transforms: Vec<Vec<f32>>,
}

pub struct ReaderClient {
    pub lines: Vec<Command>,
}

impl ClientTrait for ReaderClient {
    fn request_commands(&mut self) -> Vec<Command> {
        if self.lines.is_empty() {
            vec![Commands::NONE]
        } else {
            vec![self.lines.remove(0)]
        }
    }

    fn request_commands_by_event(&mut self, _event: &CTEvent) -> Vec<Command> {
        self.request_commands()
    }
}