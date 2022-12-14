use bevy::prelude::*;
use ctsimlib::s_setup_walls::setup_walls;

use ctviewer::{s_setup_web_tanks::setup_web_tanks, *};
use s_load_tanks::*;

use ctsimlib::core_plugin::CoreCTPlugin;
use ctsimlibgraphics::CoreCTGraphicsPlugin;

use s_apply_history_transforms::*;

fn main() {
    App::new()
        .add_plugin(CoreCTPlugin)
        .add_plugin(CoreCTGraphicsPlugin)
        .init_resource::<CustomAssetState>()
        .add_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .add_startup_system(load_tanks)
        .add_system(setup_web_tanks)
        .add_startup_system(setup_walls)
        .add_stage_after(
            "request_commands",
            "apply_history_transforms",
            SystemStage::single_threaded().with_system(apply_history_transforms),
        )
        .run();
}
