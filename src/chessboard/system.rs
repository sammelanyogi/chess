use super::component::*;
use super::constants::*;
use super::utils::*;
use crate::engine::chess::Chess;
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
) {
    for ev in ev_board.read() {
        for overlay in q_overlay.iter() {
            commands.entity(overlay).despawn();
        }
        let mut board = q_board.single_mut();
        match ev {
            BoardEvent::SelectPiece(position) => {
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
                for (mut piece, mut transform) in q_piece.iter_mut() {
                    if piece.position.0 == from.0 && piece.position.1 == from.1 {
                        transform.translation.x = LEFT + to.0 as f32 * SQUARE_SIZE;
                        transform.translation.y = BOTTOM + to.1 as f32 * SQUARE_SIZE;
                        piece.position = Position(to.0, to.1);
                        board.remove_selected();
                        break;
                    }
                }
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
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.get_single().unwrap();
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Now world_position is the position of the mouse where we got click event

            let (x, y) = (
                ((world_position.x + SQUARE_SIZE * 4.) / SQUARE_SIZE).trunc() as i8 + 1,
                ((world_position.y + SQUARE_SIZE * 4.) / SQUARE_SIZE).trunc() as i8 + 1,
            );

            if x <= 8 && x >= 1 && y <= 8 && y >= 1 {
                let mut clicked_in_piece = false;
                for piece in q_piece.iter() {
                    if piece.position.0 as i8 == x && piece.position.1 as i8 == y {
                        clicked_in_piece = true;
                        break;
                    }
                }

                if clicked_in_piece {
                    ev_board.send(BoardEvent::SelectPiece(Position(x as u8, y as u8)));
                } else {
                    let board = q_board.get_single().unwrap();

                    match &board.selected_piece {
                        Some(piece) => {
                            ev_board.send(BoardEvent::MovePiece(
                                Position(piece.position.0, piece.position.1),
                                Position(x as u8, y as u8),
                            ));
                        }
                        None => {
                            ev_board.send(BoardEvent::DeselectAll);
                        }
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
        println!("Chess: ({}, {:?})", idx, piece);

        let code = PIECES_CODE[idx];
        let texture = asset_server.load(format!("pieces/{code}.png"));

        let set_bits = get_indices_of_set_bits(*piece);

        for pos_idx in set_bits.iter() {
            let position = Chess::index_to_position(*pos_idx);
            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_xyz(
                        LEFT + position.0 as f32 * SQUARE_SIZE,
                        BOTTOM + position.1 as f32 * SQUARE_SIZE,
                        10.,
                    )
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
