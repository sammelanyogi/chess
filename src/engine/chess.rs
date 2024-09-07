use super::constants::*;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct Position(pub u8, pub u8);

// #[derive(Debug, Clone)]
// enum CastlingStatus {
//     Available,
//     QueenSide,
//     KingSide,
// }

#[derive(Component, Debug, Clone)]
pub struct Chess {
    pub pieces: [u64; 12],
    pub white_turn: bool,
    // pub castling_status: [CastlingStatus; 2],
    // pub possible_enpassant: u8,
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
            // possible_enpassant: 0,
        }
    }

    pub fn position_to_index(position: &Position) -> u8 {
        (position.1 - 1) * 8 + position.0 - 1
    }

    pub fn index_to_position(index: u8) -> Position {
        Position(index % 8 + 1, index / 8 + 1)
    }

    fn possible_pawn_moves(&self, pawn_position: &Position, for_piece: bool) -> u64 {
        let all_pieces = self.get_all_pieces();
        let opponent_pieces = self.get_color_pieces(!for_piece);

        let position_index = Chess::position_to_index(pawn_position);
        let pawn_location: u64 = 1 << position_index;

        let one_move_forward = if for_piece {
            pawn_location << 8 & !all_pieces
        } else {
            pawn_location >> 8 & !all_pieces
        };
        let two_moves_forward = if for_piece {
            ((one_move_forward & *MASK_RANK.get("RANK3").unwrap()) << 8) & !all_pieces
        } else {
            ((one_move_forward & *MASK_RANK.get("RANK6").unwrap()) >> 8) & !all_pieces
        };

        let right_attack = if for_piece {
            pawn_location << 9 & *CLEAR_FILE.get("FILE8").unwrap()
        } else {
            pawn_location >> 9 & *CLEAR_FILE.get("FILE1").unwrap()
        };

        let left_attack = if for_piece {
            pawn_location << 7 & *CLEAR_FILE.get("FILE1").unwrap()
        } else {
            pawn_location >> 7 & *CLEAR_FILE.get("FILE8").unwrap()
        };

        let pawn_attack = left_attack | right_attack;

        let valid_attacks = pawn_attack & opponent_pieces;
        let valid_moves = one_move_forward | two_moves_forward;
        valid_attacks | valid_moves
    }

    fn possible_rook_moves(&self, rook_position: &Position, for_piece: bool) -> u64 {
        let own_pieces = self.get_color_pieces(for_piece);
        let opposition_pieces = self.get_color_pieces(!for_piece);
        let position_idx = Chess::position_to_index(rook_position);
        let rl = 1 << position_idx;

        let mut moves = Vec::new();
        // Parse right
        //
        let mut right_pointer = rl;
        for _ in (rook_position.0 + 1)..9 {
            if (right_pointer << 1) & !own_pieces == 0 {
                break;
            }
            if (right_pointer << 1) & !opposition_pieces == 0 {
                moves.push(right_pointer << 1);
                break;
            }
            moves.push(right_pointer << 1);
            right_pointer = right_pointer << 1;
        }

        // Parse left
        let mut left_pointer = rl;
        for _ in (0..(rook_position.0 - 1)).rev() {
            if (left_pointer >> 1) & !own_pieces == 0 {
                break;
            }
            if (left_pointer >> 1) & !opposition_pieces == 0 {
                moves.push(left_pointer >> 1);
                break;
            }
            moves.push(left_pointer >> 1);
            left_pointer = left_pointer >> 1;
        }

        // Parse top
        let mut top_pointer = rl;
        for _ in (rook_position.1 + 1)..9 {
            if (top_pointer << 8) & !own_pieces == 0 {
                break;
            }
            if (top_pointer << 8) & !opposition_pieces == 0 {
                moves.push(top_pointer << 8);
                break;
            }
            moves.push(top_pointer << 8);
            top_pointer = top_pointer << 8;
        }

        // Parse bottom
        let mut bottom_pointer = rl;
        for _ in (0..(rook_position.1 - 1)).rev() {
            if (bottom_pointer >> 8) & !own_pieces == 0 {
                break;
            }
            if (bottom_pointer >> 8) & !opposition_pieces == 0 {
                moves.push(bottom_pointer >> 8);
                break;
            }
            moves.push(bottom_pointer >> 8);
            bottom_pointer = bottom_pointer >> 8;
        }
        moves.iter().fold(0, |acc, x| (acc | x))
    }

    fn possible_bishop_moves(&self, bishop_position: &Position, for_piece: bool) -> u64 {
        let own_pieces = self.get_color_pieces(for_piece);
        let opposition_pieces = self.get_color_pieces(!for_piece);
        let position_idx = Chess::position_to_index(bishop_position);
        let bl = 1 << position_idx;

        let mut moves = Vec::new();
        // parse north west
        let mut nw = bl;
        while nw & *CLEAR_FILE.get("FILE1").unwrap() > 0 {
            if (nw << 9) & !own_pieces == 0 {
                break;
            }
            if (nw << 9) & !opposition_pieces == 0 {
                moves.push(nw << 9);
                break;
            }
            moves.push(nw << 9);
            nw = nw << 9;
        }
        // parse north east
        let mut ne = bl;
        while ne & *CLEAR_FILE.get("FILE8").unwrap() > 0 {
            if (ne << 7) & !own_pieces == 0 {
                break;
            }
            if (ne << 7) & !opposition_pieces == 0 {
                moves.push(ne << 7);
                break;
            }
            moves.push(ne << 7);
            ne = ne << 7;
        }
        // parse south west
        let mut sw = bl;
        while sw & *CLEAR_FILE.get("FILE1").unwrap() > 0 {
            if (sw >> 7) & !own_pieces == 0 {
                break;
            }
            if (sw >> 7) & !opposition_pieces == 0 {
                moves.push(sw >> 7);
                break;
            }
            moves.push(sw >> 7);
            sw = sw >> 7;
        }
        // parse south east
        let mut se = bl;
        while se & *CLEAR_FILE.get("FILE8").unwrap() > 0 {
            if (se >> 9) & !own_pieces == 0 {
                break;
            }
            if (se >> 9) & !opposition_pieces == 0 {
                moves.push(se >> 9);
                break;
            }
            moves.push(se >> 9);
            se = se >> 9;
        }
        moves.iter().fold(0, |acc, x| (acc | x))
    }

    fn possible_knight_moves_by_location(&self, nl: u64, own_pieces: u64) -> u64 {
        // North spots
        let p1 = nl << 6 & *CLEAR_FILE.get("FILE1").unwrap() & *CLEAR_FILE.get("FILE2").unwrap();
        let p2 = nl << 10 & *CLEAR_FILE.get("FILE7").unwrap() & *CLEAR_FILE.get("FILE8").unwrap();
        let p3 = nl << 15 & *CLEAR_FILE.get("FILE1").unwrap();
        let p4 = nl << 17 & *CLEAR_FILE.get("FILE8").unwrap();

        // South spots
        let p5 = nl >> 6 & *CLEAR_FILE.get("FILE7").unwrap() & *CLEAR_FILE.get("FILE8").unwrap();
        let p6 = nl >> 10 & *CLEAR_FILE.get("FILE1").unwrap() & *CLEAR_FILE.get("FILE2").unwrap();
        let p7 = nl >> 15 & *CLEAR_FILE.get("FILE8").unwrap();
        let p8 = nl >> 17 & *CLEAR_FILE.get("FILE1").unwrap();

        (p1 | p2 | p3 | p4 | p5 | p6 | p7 | p8) & !own_pieces
    }

    fn possible_knight_moves(&self, knight_position: &Position, for_piece: bool) -> u64 {
        let own_pieces = self.get_color_pieces(for_piece);

        let position_idx = Chess::position_to_index(knight_position);
        let nl = 1 << position_idx;
        self.possible_knight_moves_by_location(nl, own_pieces)
    }

    fn possible_queen_moves(&self, queen_position: &Position, for_piece: bool) -> u64 {
        self.possible_rook_moves(queen_position, for_piece)
            | self.possible_bishop_moves(queen_position, for_piece)
    }

    fn possible_king_moves(&self, king_position: &Position, for_piece: bool) -> u64 {
        let own_pieces = self.get_color_pieces(for_piece);
        let position_idx = Chess::position_to_index(king_position);
        let king_location = 1 << position_idx;

        let north = king_location << 8;
        let south = king_location >> 8;
        let east = king_location << 1 & *CLEAR_FILE.get("FILE8").unwrap();
        let west = king_location >> 1 & *CLEAR_FILE.get("FILE1").unwrap();

        let ne = king_location << 9 & *CLEAR_FILE.get("FILE8").unwrap();
        let nw = king_location << 7 & *CLEAR_FILE.get("FILE1").unwrap();
        let se = king_location >> 7 & *CLEAR_FILE.get("FILE8").unwrap();
        let sw = king_location >> 9 & *CLEAR_FILE.get("FILE1").unwrap();

        (north | south | east | west | ne | nw | se | sw) & !own_pieces
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

    fn get_positions(piece: u64) -> Vec<Position> {
        let mut all_positions = Vec::new();
        for i in 0..(64 as u64) {
            if (1 << i) & piece > 0 {
                all_positions.push(Chess::index_to_position(i as u8))
            }
        }
        return all_positions;
    }

    pub fn king_saving_move(&self, piece_idx: u8, position: &Position) -> u64 {
        let possible_moves =
            self.get_possible_moves_by_piece_idx(piece_idx, position, self.white_turn);
        let positions = Chess::get_positions(possible_moves);

        let mut saving_moves = 0;

        for new_position in positions {
            let mut new_chess = self.clone();
            new_chess.dry_move(position, &new_position);
            if !new_chess.is_in_check() {
                let idx = Chess::position_to_index(&new_position);
                if piece_idx != self.get_king_position_idx() {
                    return 1 << idx;
                } else {
                    saving_moves = saving_moves | (1 << idx);
                }
            }
        }
        return saving_moves;
    }

    pub fn get_possible_moves(&self, position: &Position) -> u64 {
        // Caculate which piece is in the position `position`
        if let Some(piece_idx) = self.get_piece(position) {
            if self.is_in_check() {
                return self.king_saving_move(piece_idx, position);
            }
            let mut location =  self.get_possible_moves_by_piece_idx(piece_idx, position, self.white_turn);
            // prune all the move which causes check when applied
                    // Check if the move is causing further check
            let new_positions = Chess::get_positions(location);
            for new_position in new_positions.iter() {

                let mut new_chess = self.clone();
                new_chess.dry_move(position, new_position);
                if new_chess.is_in_check() {
                    let prune_idx = Chess::position_to_index(new_position);
                    location = location & !(1 << prune_idx);
                }
            }
            return location;
        }
        0
    }

    pub fn get_possible_moves_by_piece_idx(
        &self,
        piece_idx: u8,
        position: &Position,
        for_piece: bool,
    ) -> u64 {
        if piece_idx == 0 || piece_idx == 6 {
            return self.possible_pawn_moves(position, for_piece);
        } else if piece_idx == 1 || piece_idx == 7 {
            return self.possible_rook_moves(position, for_piece);
        } else if piece_idx == 2 || piece_idx == 8 {
            return self.possible_knight_moves(position, for_piece);
        } else if piece_idx == 3 || piece_idx == 9 {
            return self.possible_bishop_moves(position, for_piece);
        } else if piece_idx == 4 || piece_idx == 10 {
            return self.possible_queen_moves(position, for_piece);
        } else if piece_idx == 5 || piece_idx == 11 {
            return self.possible_king_moves(position, for_piece);
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

    pub fn contains_piece(&self, pos: &Position) -> bool {
        let all_pieces = self.get_all_pieces();
        let idx = Chess::position_to_index(pos);

        ((1 << idx) & all_pieces) > 0
    }

    pub fn dry_move(&mut self, from: &Position, to: &Position) -> bool {
        // Change bits of moving piece
        let to_idx = Chess::position_to_index(to) as u64;
        let from_idx = Chess::position_to_index(from) as u64;
        if let Some(from_piece_idx) = self.get_piece(from) {
            if let Some(to_piece_idx) = self.get_piece(to) {
                // if destination square occupies my own piece return
                let my_pieces = self.get_color_pieces(self.white_turn);
                if my_pieces & (1 << to_idx) > 0 {
                    return false;
                }

                // if any piece is in destination, delete the piece
                self.pieces[to_piece_idx as usize] =
                    self.pieces[to_piece_idx as usize] & !(1 << to_idx);
            }
            // move current piece to destination
            self.pieces[from_piece_idx as usize] =
                self.pieces[from_piece_idx as usize] & !(1 << from_idx) | (1 << to_idx);

            return true
        }
        false
    }

    pub fn move_piece(&mut self, from: &Position, to: &Position) -> bool {
        println!("Moving {:?} -> {:?}", from, to);

        if !self.is_move_valid(from, to) {
            return false;
        }
        // Check if the move is causing further check
        let mut new_chess = self.clone();
        new_chess.dry_move(from, to);
        if new_chess.is_in_check() {
            return false
        }

        let move_successful = self.dry_move(from, to);
        // Check if move created any check
        if move_successful {
            self.white_turn = !self.white_turn;
        }
        return move_successful;
    }

    pub fn get_piece_indices(for_piece: bool) -> [u8; 6] {
        if for_piece {
            return [0, 1, 2, 3, 4, 5];
        }
        return [6, 7, 8, 9, 10, 11];
    }

    pub fn get_king_position_idx(&self) -> u8 {
        if self.white_turn {
            5
        } else {
            11
        }
    }

    pub fn opponent_all_valid_moves(&self) -> u64 {
        let indices = Chess::get_piece_indices(!self.white_turn);

        let mut valid_moves = 0 as u64;
        for i in indices {
            // check if king is attacked by opponent pieces
            let positions = Chess::get_positions(self.pieces[i as usize]);
            for position in positions.iter() {
                let vm = self.get_possible_moves_by_piece_idx(i, position, !self.white_turn);
                valid_moves = valid_moves | vm;
            }
        }
        return valid_moves;
    }

    pub fn is_in_check(&self) -> bool {
        let king_piece = if self.white_turn {
            self.pieces[5]
        } else {
            self.pieces[11]
        };
        // Check if king is being attacked by any other opponent piece
        //
        let opponent_valid_moves = self.opponent_all_valid_moves();
        (king_piece & opponent_valid_moves) > 0
    }

    pub fn is_checkmate(&self) -> bool {
        let indices = Chess::get_piece_indices(self.white_turn);
        // check if any of my piece can save king by moving
        for i in indices {
            let positions  = Chess::get_positions(self.pieces[i as usize]);
            for position in positions.iter() {
                let vm = self.king_saving_move(i, position);
                if vm > 0 {
                    return false;
                }
            }
        }
        return true;
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
