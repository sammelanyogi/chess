use std::{i32, time::Duration};

use bevy::prelude::{EventWriter, Query};
use bevy_async_task::{AsyncTaskRunner, AsyncTaskStatus};

use crate::{chessboard::component::BoardEvent, engine::chess::Chess};

use super::bot1::minimax;

pub fn handle_move(
    q_chess: Query<&Chess>,
    mut async_runner: AsyncTaskRunner<Chess>,
    mut ev_board: EventWriter<BoardEvent>,
) {
    let chess = q_chess.single().clone();

    if chess.is_checkmate() {
        return;
    }

    if chess.white_turn {
        return;
    }

    async fn get_move(c: Chess) -> Chess {
        // let rnd = thread_rng().gen_range(0..10);
        let depth = 3;
        if depth == 3 {
            async_std::task::sleep(Duration::from_millis(800)).await;
        }
        let (chess, _) = minimax(&c, depth, false, i32::MIN, i32::MAX).await;
        let chess = chess.unwrap();
        return chess.clone();
    }

    match async_runner.poll() {
        AsyncTaskStatus::Idle => {
            async_runner.start(get_move(chess.clone()));
        }
        AsyncTaskStatus::Pending => {}
        AsyncTaskStatus::Finished(c) => {
            if let Some(last_move) = c.last_move {
                let from = Chess::index_to_position(last_move.0);
                let to = Chess::index_to_position(last_move.1);
                ev_board.send(BoardEvent::MovePiece(from, to));
            }
        }
    }
}
