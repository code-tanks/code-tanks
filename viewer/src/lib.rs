pub mod s_load_tanks;
pub mod s_setup_web_tanks;
pub mod s_apply_history_transforms;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};

use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
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

#[derive(Default)]
pub struct CustomAssetState {
    pub handle: Handle<CustomAsset>,
    pub printed: bool,
}

#[derive(Component)]
pub struct HistoryTransforms {
    pub transforms: Vec<Vec<f32>>,
}