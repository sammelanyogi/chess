use super::chess::Chess;
use bevy::prelude::*;

pub fn spawn_engine(mut commands: Commands) {
    commands.spawn((Chess::new(),));
}
