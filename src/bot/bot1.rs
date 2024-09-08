use std::{
    cmp::{max, min},
    i32,
};

use rand::{seq::SliceRandom, thread_rng};

use crate::engine::chess::Chess;

const PIECES_WEIGHT: [i32; 12] = [10, 30, 30, 50, 90, 900, -10, -30, -30, -50, -90, -900];

pub fn evaluate(chess: &Chess) -> i32 {
    let mut total = 0;
    for idx in 0..chess.pieces.len() {
        let n = chess.pieces[idx].count_ones();
        total = total + (n as i32) * PIECES_WEIGHT[idx];
    }
    return total;
}

pub async fn minimax(
    chess: &Chess,
    depth: u8,
    is_maximizing: bool,
    alpha: i32,
    beta: i32,
) -> (Option<Chess>, i32) {
    if depth == 0 || chess.is_checkmate() {
        return (None, evaluate(chess));
    }
    let mut equally_best_moves = Vec::new();
    if is_maximizing {
        let mut max_ev = i32::MIN;
        for new_chess in chess.possible_states().iter() {
            let (_, eval) = Box::pin(minimax(new_chess, depth - 1, false, alpha, beta)).await;
            if eval > max_ev {
                max_ev = eval;
                equally_best_moves = Vec::from([new_chess.clone()])
            }
            if eval == max_ev {
                equally_best_moves.push(new_chess.clone())
            }
            let na = max(alpha, eval);
            if beta <= na {
                break;
            }
        }
        return (
            equally_best_moves.choose(&mut thread_rng()).cloned(),
            max_ev,
        );
    } else {
        let mut min_ev = i32::MAX;
        for new_chess in chess.possible_states().iter() {
            let (_, eval) = Box::pin(minimax(new_chess, depth - 1, true, alpha, beta)).await;

            if eval < min_ev {
                min_ev = eval;
                equally_best_moves = Vec::from([new_chess.clone()])
            }
            if eval == min_ev {
                equally_best_moves.push(new_chess.clone())
            }
            let nb = min(beta, eval);
            if alpha >= nb {
                break;
            }
        }
        return (
            equally_best_moves.choose(&mut thread_rng()).cloned(),
            min_ev,
        );
    }
}
