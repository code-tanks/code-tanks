use bevy::prelude::*;
use ctgraphics::s_setup_reader_tanks::setup_reader_tanks;
use ctengine::{CustomAssetState, CustomAsset, CustomAssetLoader};
use ctengine::s_apply_history_transforms::apply_history_transforms;
use ctengine::{s_setup_walls::setup_walls, s_request_commands::request_commands, s_apply_commands::apply_commands};

// use ctviewer::s_load_tanks;
// use ctviewer::s_load_tanks::{self, load_tanks_from_file};
// use ctviewer::{s_setup_reader_tanks::setup_reader_tanks, *};
use ctviewer::s_load_tanks::load_tanks_from_file;
// use s_setup_ground::*;

use ctengine::core_plugin::CoreCTPlugin;
use ctgraphics::CoreCTGraphicsPlugin;

use ctgraphics::s_setup_ground::setup_ground;
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
        .add_systems(Startup, (load_tanks_from_file, setup_walls, setup_ground))
        .add_systems(
            Update,
            (setup_reader_tanks, apply_history_transforms.after(request_commands).before(apply_commands))
            // "request_commands",
            // "apply_history_transforms",
            // SystemStage::single_threaded().with_system(apply_history_transforms),
        )
        .run();
}
