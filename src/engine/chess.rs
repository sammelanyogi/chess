use super::constants::*;
use bevy::prelude::*;

fn print_bits(x: u64) {
    for byte in x.to_be_bytes().iter() {
        println!("{:08b}", byte);
    }
    println!();
}

#[derive(Clone, Debug)]
pub struct Position(pub u8, pub u8);

#[derive(Debug)]
enum CastlingStatus {
    Available,
    QueenSide,
    KingSide,
}

#[derive(Component, Debug)]
pub struct Chess {
    pub pieces: [u64; 12],
    pub white_turn: bool,
    pub castling_status: [CastlingStatus; 2],
    pub possible_enpassant: u8,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            pieces: [
                65280,
                129,
                66,
                36,
                8,
                16,
                71776119061217280,
                9295429630892703744,
                4755801206503243776,
                2594073385365405696,
                576460752303423488,
                1152921504606846976,
            ],
            white_turn: true,
            castling_status: [CastlingStatus::Available, CastlingStatus::Available],
            possible_enpassant: 0,
        }
    }

    pub fn position_to_index(position: &Position) -> u8 {
        (position.1 - 1) * 8 + position.0 - 1
    }

    pub fn index_to_position(index: u8) -> Position {
        Position(index % 8 + 1, index / 8 + 1)
    }

    fn possible_pawn_moves(&self, pawn_position: &Position) -> u64 {
        let all_pieces = self.get_all_pieces();
        let opponent_pieces = self.get_color_pieces(!self.white_turn);

        let position_index = Chess::position_to_index(pawn_position);
        let pawn_location: u64 = 1 << position_index;

        let one_move_forward = if self.white_turn {
            pawn_location << 8 & !all_pieces
        } else {
            pawn_location >> 8 & !all_pieces
        };

        let right_attack = if self.white_turn {
            pawn_location << 9 & *CLEAR_FILE.get("FILE8").unwrap()
        } else {
            pawn_location >> 9 & *CLEAR_FILE.get("FILE1").unwrap()
        };

        let left_attack = if self.white_turn {
            pawn_location << 7 & *CLEAR_FILE.get("FILE1").unwrap()
        } else {
            pawn_location >> 7 & *CLEAR_FILE.get("FILE8").unwrap()
        };

        let pawn_attack = left_attack | right_attack;

        let valid_attacks = pawn_attack & opponent_pieces;
        let valid_moves = one_move_forward;
        valid_attacks | valid_moves
    }

    fn possible_king_moves(&self, king_position: &Position) -> u64 {
        0
    }

    // returns the index of the piece occupying the desired position
    fn get_piece(&self, position: &Position) -> Option<u8> {
        let pos_idx = Chess::position_to_index(position);
        if let Some(found) = self
            .pieces
            .iter()
            .enumerate()
            .find(|(_, piece_pos)| ((1 << pos_idx) & *piece_pos) > 0)
        {
            return Some(found.0 as u8);
        }
        return None;
    }

    pub fn get_possible_moves(&self, position: &Position) -> u64 {
        // Caculate which piece is in the position `position`
        if let Some(piece_idx) = self.get_piece(position) {
            if piece_idx == 0 || piece_idx == 6 {
                return self.possible_pawn_moves(position);
            }
        }
        0
    }

    fn is_move_valid(&self, from: &Position, to: &Position) -> bool {
        let possible_moves = self.get_possible_moves(from);
        let idx = Chess::position_to_index(to);

        ((1 << idx) & possible_moves) > 0
    }

    pub fn is_touch_valid_for_move(&self, pos: &Position) -> bool {
        let own_piece = self.get_color_pieces(self.white_turn);
        let idx = Chess::position_to_index(pos);

        ((1 << idx) & own_piece) == 0
    }

    pub fn move_piece(&mut self, from: &Position, to: &Position) -> bool {
        println!("Moving {:?} -> {:?}", from, to);

        if !self.is_move_valid(from, to) {
            return false;
        }
        // Change bits of moving piece
        let to_idx = Chess::position_to_index(to) as u64;
        let from_idx = Chess::position_to_index(from) as u64;

        if let Some(to_piece_idx) = self.get_piece(to) {
            // if destination square occupies my own piece return
            let my_pieces = self.get_color_pieces(self.white_turn);
            if my_pieces & (1 << to_idx) > 0 {
                return false;
            }
            println!("Destination has other piece");

            // if any piece is in destination, delete the piece
            self.pieces[to_piece_idx as usize] =
                self.pieces[to_piece_idx as usize] & !(1 << to_idx);
        }
        if let Some(from_piece_idx) = self.get_piece(from) {
            // move current piece to destination
            self.pieces[from_piece_idx as usize] =
                self.pieces[from_piece_idx as usize] & !(1 << from_idx) | (1 << to_idx);

            self.white_turn = !self.white_turn;
            return true;
        }
        false
    }

    pub fn is_valid_selection(&self, position: &Position) -> bool {
        let bitboard = self.get_color_pieces(self.white_turn);
        let idx = Chess::position_to_index(position);

        ((1 << idx) & bitboard) > 0
    }

    // piece_color is true for white piece and false for black
    fn get_color_pieces(&self, piece_color: bool) -> u64 {
        let desired_pieces = if piece_color {
            &self.pieces[0..6]
        } else {
            &self.pieces[6..12]
        };
        desired_pieces.iter().fold(0, |acc, &x| acc | x)
    }

    fn get_all_pieces(&self) -> u64 {
        self.pieces.iter().fold(0, |acc, &x| acc | x)
    }
}
