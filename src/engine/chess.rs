use super::constants::*;
use bevy::prelude::*;

fn print_bits(x: u64) {
    for byte in x.to_be_bytes().iter() {
        println!("{:08b}", byte);
    }
    println!();
}

#[derive(Debug)]
struct Position(u8, u8);

#[derive(Debug)]
enum CastlingStatus {
    Available,
    QueenSide,
    KingSide,
}

#[derive(Component, Debug)]
pub struct Chess {
    pieces: [u64; 12],
    white_turn: bool,
    castling_status: [CastlingStatus; 2],
    possible_enpassant: u8,
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

    fn position_to_index(position: &Position) -> u8 {
        (position.1 - 1) * 8 + position.0 - 1
    }

    fn index_to_position(index: u8) -> Position {
        Position(index / 8 + 1, index % 8 + 1)
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

        print_bits(pawn_location);
        print_bits(right_attack);

        valid_attacks | valid_moves
    }

    fn possible_king_moves(&self, king_position: &Position) -> u64 {
        0
    }

    pub fn get_possible_moves(position_idx: u8) -> u64 {
        // Caculate which piece is in the position `position_idx`
        0
    }

    fn is_move_valid() -> bool {
        false
    }

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
