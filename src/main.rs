mod bot;
mod camera;
mod chessboard;
mod engine;

use bevy::ecs::schedule::ExecutorKind;
use bevy::prelude::*;
use bevy::window::*;

use bot::BotPlugin;
use camera::CameraPlugin;
use chessboard::ChessBoardPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1280., 720.).with_scale_factor_override(1.),
                    ..default()
                }),
                ..default()
            }),
            CameraPlugin,
            ChessBoardPlugin,
            BotPlugin,
        ))
        .edit_schedule(Update, |schedule| {
            schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        })
        .run();
}
