mod bot1;
mod system;

use bevy::prelude::*;

use crate::chessboard::component::BoardEvent;
use system::*;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_move,))
            .add_event::<BoardEvent>();
    }
}
