use bevy::{prelude::*, window::PrimaryWindow, render::view::screenshot::ScreenshotManager};
use ctsimlib::{TickState, MaxSimulationTicks};

pub fn s_capture(
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    state: Res<TickState>,
    max_ticks: Res<MaxSimulationTicks>,
) {
    let path = format!("./frames/{}.png", state.count);
    screenshot_manager
        .save_screenshot_to_disk(main_window.single(), path)
        .unwrap();

    // println!("{}", state.count);

    if state.count == max_ticks.0 {
        println!("concat pics {} {}", state.count, max_ticks.0);
    }
}