mod camera;
mod chessboard;
mod engine;

use bevy::prelude::*;

use camera::CameraPlugin;
use chessboard::ChessBoardPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, ChessBoardPlugin))
        .run();
}
