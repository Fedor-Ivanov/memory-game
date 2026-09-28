use bevy::prelude::{Window, WindowPlugin, default};

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 700;

const RESOLUTION: (u32, u32) = (WINDOW_WIDTH, WINDOW_HEIGHT);

const TITLE: &str = "Memory";

pub fn primary_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: TITLE.to_string(),
            resolution: RESOLUTION.into(),
            ..default()
        }),
        ..default()
    }
}
