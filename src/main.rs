mod camera;
mod chessboard;
mod engine;

use bevy::prelude::*;
use bevy::window::*;

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
        ))
        .run();
}
