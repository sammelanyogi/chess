use crate::engine::chess::*;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub enum BoardEvent {
    SelectPiece(Position),
    DeselectAll,
    MovePiece(Position, Position),
}

#[derive(Component, Debug)]
pub struct Square;

#[derive(Component, Debug)]
pub struct Overlay;

#[derive(Component, Debug)]
pub struct LastMoveOverlay;

#[derive(Component, Debug)]
pub struct Piece {
    pub is_white: bool,
    pub position: Position,
    pub code: String,
}

#[derive(Component, Debug)]
pub struct Board {
    pub white_turn: bool,
    pub selected_piece: Option<Piece>,
    pub white_out_count: u8,
    pub black_out_count: u8,
}
impl Board {
    pub fn update_piece(&mut self, piece: Piece) {
        self.selected_piece = Some(piece);
    }
    pub fn remove_selected(&mut self) {
        self.selected_piece = None;
    }
    pub fn update_turn(&mut self, white_turn: bool) {
        self.white_turn = white_turn;
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub is_white: bool,
}

#[derive(Component, Debug)]
pub struct TextInfo {
    pub text_type: u8,
}
