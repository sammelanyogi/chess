use bevy::prelude::*;

#[derive(Debug)]
pub struct Position(pub u8, pub u8);

#[derive(Event, Debug)]
pub enum BoardEvent {
    SelectPiece(Position),
    DeselectAll,
    MovePiece(Position, Position)
}

#[derive(Component, Debug)]
pub struct Square;

#[derive(Component, Debug)]
pub struct Overlay;

#[derive(Component, Debug)]
pub struct Piece {
    pub position: Position,
    pub code: String,
}

#[derive(Component, Debug)]
pub struct Board {
    pub selected_piece: Option<Piece>
}
impl Board {
    pub fn update_piece(&mut self, piece: Piece) {
        self.selected_piece = Some(piece);
    }
    pub fn remove_selected(&mut self) {
        self.selected_piece = None;
    }
}
