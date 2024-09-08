mod bot1;
mod system;

use bevy::prelude::*;

use crate::chessboard::component::{Board, BoardEvent};
use system::*;

pub struct BotPlugin;

fn run_if_my_turn(q_board: Query<&Board>) -> bool {
    let board = q_board.get_single().unwrap();
    !board.white_turn
}

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_move.run_if(run_if_my_turn), handle_tasks))
            .add_event::<BoardEvent>();
    }
}
