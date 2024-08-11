use super::component::*;
use super::constants::*;
use super::utils::*;
use crate::engine::chess::*;
use bevy::prelude::*;
use bevy::window::*;

pub fn spawn_board(mut commands: Commands) {
    let black_square = Sprite {
        color: BLACK,
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..Default::default()
    };

    let white_square = Sprite {
        color: WHITE,
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..Default::default()
    };

    commands.spawn(Board {
        selected_piece: None,
    });

    for i in 1..9 {
        for j in 1..9 {
            let sprite = if (i + j) % 2 == 0 {
                black_square.clone()
            } else {
                white_square.clone()
            };

            let x_t = LEFT + j as f32 * SQUARE_SIZE;
            let y_t = BOTTOM + i as f32 * SQUARE_SIZE;
            commands.spawn((
                SpriteBundle {
                    sprite,
                    transform: Transform::from_xyz(x_t, y_t, 0.),
                    ..Default::default()
                },
                Square,
            ));
        }
    }
}

pub fn handle_board_event(
    mut commands: Commands,
    mut ev_board: EventReader<BoardEvent>,
    q_overlay: Query<Entity, With<Overlay>>,
    mut q_board: Query<&mut Board>,
    mut q_piece: Query<(&mut Piece, &mut Transform)>,
    mut q_chess: Query<&mut Chess>,
) {
    for ev in ev_board.read() {
        // Clear previous overlays before applying new one.
        for overlay in q_overlay.iter() {
            commands.entity(overlay).despawn();
        }
        let mut board = q_board.single_mut();
        let mut chess = q_chess.single_mut();
        match ev {
            BoardEvent::SelectPiece(position) => {
                if !chess.is_valid_selection(position) {
                    return;
                }
                let blue_square = Sprite {
                    color: BLUE,
                    custom_size: Some(Vec2::splat(SQUARE_SIZE)),
                    ..Default::default()
                };
                let x_t = LEFT + position.0 as f32 * SQUARE_SIZE;
                let y_t = BOTTOM + position.1 as f32 * SQUARE_SIZE;

                // Spawn color for selected square
                commands.spawn((
                    SpriteBundle {
                        sprite: blue_square,
                        transform: Transform::from_xyz(x_t, y_t, 0.),
                        ..Default::default()
                    },
                    Overlay,
                ));
                // Get Possible Moves
                let moves = chess.get_possible_moves(position);
                let indices = get_indices_of_set_bits(moves);

                for pos_idx in indices.iter() {
                    let circle_pos = Chess::index_to_position(*pos_idx);
                    let cx_t = LEFT + circle_pos.0 as f32 * SQUARE_SIZE;
                    let cy_t = BOTTOM + circle_pos.1 as f32 * SQUARE_SIZE;

                    let blue_circle = Sprite {
                        color: BLUE,
                        custom_size: Some(Vec2::splat(SQUARE_SIZE / 2.5)),
                        ..default()
                    };

                    commands.spawn((
                        SpriteBundle {
                            sprite: blue_circle,
                            transform: Transform::from_xyz(cx_t, cy_t, 0.),
                            ..default()
                        },
                        Overlay,
                    ));
                }

                for (piece, _) in q_piece.iter() {
                    if piece.position.0 == position.0 && piece.position.1 == position.1 {
                        board.update_piece(Piece {
                            position: Position(piece.position.0, piece.position.1),
                            code: piece.code.clone(),
                        });
                        break;
                    }
                }
            }
            BoardEvent::DeselectAll => {
                board.remove_selected();
            }
            BoardEvent::MovePiece(from, to) => {
                println!("got Move piece event");
                if chess.move_piece(from, to) {
                    println!("Move Successful");
                    // Reflect the move in UI
                    if let Some((mut out_piece, mut out_transform)) = q_piece
                        .iter_mut()
                        .find(|(piece, _)| piece.position.0 == to.0 && piece.position.1 == to.1)
                    {
                        out_transform.translation.y = BOTTOM;
                        out_piece.position = Position(9, 9);
                    }
                    if let Some((mut piece, mut transform)) = q_piece
                        .iter_mut()
                        .find(|(piece, _)| piece.position.0 == from.0 && piece.position.1 == from.1)
                    {
                        transform.translation.x = LEFT + to.0 as f32 * SQUARE_SIZE;
                        transform.translation.y = BOTTOM + to.1 as f32 * SQUARE_SIZE;
                        piece.position = Position(to.0, to.1);
                    }
                }
                board.remove_selected();
            }
        }
    }
}

pub fn handle_input(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_piece: Query<&Piece>,
    mut ev_board: EventWriter<BoardEvent>,
    q_board: Query<&Board>,
    q_chess: Query<&Chess>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.get_single().unwrap();
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // Now world_position is the position of the mouse where we got click event
        if let Some(touch_pos) =
            world_position_to_chess_position((world_position.x, world_position.y))
        {
            let board = q_board.get_single().unwrap();

            match &board.selected_piece {
                Some(piece) => {
                    let chess = q_chess.get_single().unwrap();

                    if chess.is_touch_valid_for_move(&touch_pos) {
                        ev_board.send(BoardEvent::MovePiece(piece.position.clone(), touch_pos));
                    } else {
                        ev_board.send(BoardEvent::SelectPiece(touch_pos));
                    }
                }
                None => {
                    let mut clicked_in_piece = false;
                    for piece in q_piece.iter() {
                        if piece.position.0 == touch_pos.0 && piece.position.1 == touch_pos.1 {
                            clicked_in_piece = true;
                            break;
                        }
                    }

                    if clicked_in_piece {
                        ev_board.send(BoardEvent::SelectPiece(touch_pos));
                    } else {
                        ev_board.send(BoardEvent::DeselectAll);
                    }
                }
            }
        }
    }
}

pub fn spawn_pieces(
    mut commands: Commands,
    q_chess: Query<&Chess>,
    asset_server: ResMut<AssetServer>,
) {
    // Spawn Pieces logic implementations
    let chess = q_chess.get_single().unwrap();

    for (idx, piece) in chess.pieces.iter().enumerate() {
        println!("Piece Spawn: ({}, {:?})", idx, piece);

        let code = PIECES_CODE[idx];
        let texture = asset_server.load(format!("pieces/{code}.png"));

        let set_bits = get_indices_of_set_bits(*piece);

        for pos_idx in set_bits.iter() {
            let position = Chess::index_to_position(*pos_idx);
            let world_pos = chess_position_to_world_position(position.clone());
            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_xyz(world_pos.0, world_pos.1, 10.)
                        .with_scale(Vec3::splat(0.8)),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                Piece {
                    code: code.to_string(),
                    position: Position(position.0, position.1),
                },
            ));
        }
    }
}
