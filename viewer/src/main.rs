use bevy::prelude::*;
use ctsimlib::{s_setup_walls::setup_walls, s_request_commands::request_commands, s_apply_commands::apply_commands};

use ctviewer::{s_setup_web_tanks::setup_web_tanks, *};
use s_load_tanks::*;
use s_setup_ground::*;

use ctsimlib::core_plugin::CoreCTPlugin;
use ctsimlibgraphics::CoreCTGraphicsPlugin;

use s_apply_history_transforms::*; 

// #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
// pub struct SetupWebTanks;

// #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
// pub struct ApplyHistoryTransforms;

fn main() {

    App::new()
        .add_plugins(CoreCTPlugin)
        .add_plugins(CoreCTGraphicsPlugin)
        .init_resource::<CustomAssetState>()
        .add_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .add_systems(Startup, (load_tanks, setup_walls, setup_ground).chain())
        .add_systems(
            Update,
            (setup_web_tanks, apply_history_transforms.after(request_commands).before(apply_commands))
            // "request_commands",
            // "apply_history_transforms",
            // SystemStage::single_threaded().with_system(apply_history_transforms),
        )
        .run();
}
