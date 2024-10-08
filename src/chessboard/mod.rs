pub mod component;
mod constants;
pub mod system;
mod utils;

use bevy::input::common_conditions::*;
use bevy::prelude::*;

use crate::engine::system::spawn_engine;
use component::*;
use system::*;

pub struct ChessBoardPlugin;

impl Plugin for ChessBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_engine)
            .add_systems(Startup, (spawn_board, spawn_pieces, spawn_texts))
            .add_event::<BoardEvent>()
            .add_systems(
                Update,
                (
                    handle_input.run_if(input_just_pressed(MouseButton::Left)),
                    handle_board_event,
                ),
            );
    }
}
