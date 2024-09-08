use std::{i32, time::Duration};

use bevy::{
    prelude::{Commands, Component, Entity, EventWriter, Query, With},
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};

use crate::{
    chessboard::component::{Board, BoardEvent},
    engine::chess::Chess,
};

use super::bot1::minimax;

#[derive(Component)]
pub struct ComputeTask(pub Task<Chess>);

pub fn handle_move(mut commands: Commands, q_chess: Query<&Chess>, mut q_board: Query<&mut Board>) {
    let chess = q_chess.single().clone().clone();

    if chess.is_checkmate() {
        return;
    }

    if chess.white_turn {
        return;
    }

    let pool = AsyncComputeTaskPool::get();
    let entity = commands.spawn_empty().id();

    let task = pool.spawn(async move {
        async_std::task::sleep(Duration::from_secs(1)).await;
        let (chess, _) = minimax(&chess, 3, false, i32::MIN, i32::MAX).await;
        let chess = chess.unwrap();

        return chess.clone();
    });

    commands.entity(entity).insert(ComputeTask(task));
    let mut board = q_board.single_mut();
    board.update_turn(true);
}

pub fn handle_tasks(
    mut commands: Commands,
    mut compute_tasks: Query<&mut ComputeTask>,
    mut ev_board: EventWriter<BoardEvent>,
    query: Query<Entity, With<ComputeTask>>,
) {
    for mut task in &mut compute_tasks {
        if let Some(chess) = block_on(future::poll_once(&mut task.0)) {
            if let Some(last_move) = chess.last_move {
                let from = Chess::index_to_position(last_move.0);
                let to = Chess::index_to_position(last_move.1);
                println!("From {:?} To {:?}", from, to);
                ev_board.send(BoardEvent::MovePiece(from, to));
            }
            for entity in query.iter() {
                commands.entity(entity).remove::<ComputeTask>();
            }
        }
    }
}
