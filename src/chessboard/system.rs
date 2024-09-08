use super::component::*;
use super::constants::*;
use super::utils::*;
use crate::engine::chess::*;
use bevy::prelude::*;
use bevy::sprite::*;
use bevy::window::*;

pub fn spawn_board(mut commands: Commands) {
    let black_square = Sprite {
        color: BLACK.into(),
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..Default::default()
    };

    let white_square = Sprite {
        color: WHITE.into(),
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..Default::default()
    };

    commands.spawn(Board {
        white_turn: true,
        selected_piece: None,
        white_out_count: 0,
        black_out_count: 0,
    });

    for i in 0..2 {
        let is_white = if i == 0 { true } else { false };
        let color = if is_white { GREEN.into() } else { GRAY.into() };
        let player_sprite = Sprite {
            color,
            custom_size: Some(Vec2::splat(SQUARE_SIZE / 2.)),
            ..default()
        };
        let (x_t, y_t) = if is_white {
            (3.5 * SQUARE_SIZE, -4.5 * SQUARE_SIZE)
        } else {
            (-3.5 * SQUARE_SIZE, 4.5 * SQUARE_SIZE)
        };
        commands.spawn((
            SpriteBundle {
                sprite: player_sprite,
                transform: Transform::from_xyz(x_t, y_t, 0.),
                ..default()
            },
            Player { is_white },
        ));
    }

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

pub fn spawn_texts(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/ProtestGuerrilla-Regular.ttf");
    let d_font: Handle<Font> = asset_server.load("fonts/Gantari.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        color: DARK_BLACK.into(),
        font_size: 90.0,
        ..default()
    };
    let msg_style = TextStyle {
        font: d_font.clone(),
        color: DARK_BLACK.into(),
        font_size: 40.,
        ..default()
    };
    let status_style = TextStyle {
        font: d_font.clone(),
        color: WHITE.into(),
        font_size: 32.,
        ..default()
    };
    let text_justification = JustifyText::Center;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("CHECKMATE", text_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(0., 20., 30.).with_scale(Vec3::splat(0.)),
            ..default()
        },
        TextInfo { text_type: 1 },
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("", msg_style.clone()).with_justify(text_justification),
            transform: Transform::from_xyz(0., -30., 30.).with_scale(Vec3::splat(0.)),
            ..default()
        },
        TextInfo { text_type: 2 },
    ));
    // White's status
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Your turn", status_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(
                3.5 * SQUARE_SIZE - SQUARE_SIZE * 1.5,
                -4.5 * SQUARE_SIZE,
                30.,
            )
            .with_scale(Vec3::splat(1.)),
            ..default()
        },
        TextInfo { text_type: 3 },
    ));
    // Black's status
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Thinking..", status_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(
                -3.5 * SQUARE_SIZE + SQUARE_SIZE * 1.5,
                4.5 * SQUARE_SIZE,
                30.,
            )
            .with_scale(Vec3::splat(0.)),
            ..default()
        },
        TextInfo { text_type: 4 },
    ));
}

pub fn handle_board_event(
    mut commands: Commands,
    mut ev_board: EventReader<BoardEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q_chess: Query<&mut Chess>,
    mut q_board: Query<&mut Board>,
    mut q_piece: Query<(&mut Piece, &mut Transform), Without<TextInfo>>,
    mut q_player: Query<(&mut Player, &mut Sprite)>,
    mut q_texts: Query<(&TextInfo, &mut Transform, &mut Text)>,
    q_overlay: Query<Entity, With<Overlay>>,
    q_last_move_overlay: Query<Entity, With<LastMoveOverlay>>,
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
                    color: BLUE.into(),
                    custom_size: Some(Vec2::splat(SQUARE_SIZE)),
                    ..Default::default()
                };
                let (x_t, y_t) = chess_position_to_world_position(position.clone());

                // Spawn color for selected square
                commands.spawn((
                    SpriteBundle {
                        sprite: blue_square,
                        transform: Transform::from_xyz(x_t, y_t, 5.),
                        ..Default::default()
                    },
                    Overlay,
                ));
                // Get Possible Moves
                let moves = chess.get_possible_moves(position);
                let indices = get_indices_of_set_bits(moves);

                // Spawn highlights for all possible moves
                for pos_idx in indices.iter() {
                    let circle_pos = Chess::index_to_position(*pos_idx);
                    let (cx_t, cy_t) = chess_position_to_world_position(circle_pos.clone());

                    if chess.contains_piece(&circle_pos) {
                        let annulus_mesh = Mesh2dHandle(
                            meshes.add(Annulus::new(SQUARE_SIZE / 2. - 12., SQUARE_SIZE / 2. - 4.)),
                        );
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: annulus_mesh,
                                material: materials.add(ColorMaterial::from_color(BLUE)),
                                transform: Transform::from_xyz(cx_t, cy_t, 5.0),
                                ..default()
                            },
                            Overlay,
                        ));
                    } else {
                        let circle_mesh = Mesh2dHandle(meshes.add(Circle {
                            radius: SQUARE_SIZE / 6.0,
                        }));
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: circle_mesh,
                                material: materials.add(ColorMaterial::from_color(BLUE)),
                                transform: Transform::from_xyz(cx_t, cy_t, 5.0),
                                ..default()
                            },
                            Overlay,
                        ));
                    }
                }

                for (piece, _) in q_piece.iter() {
                    if piece.position.0 == position.0 && piece.position.1 == position.1 {
                        board.update_piece(Piece {
                            is_white: piece.is_white,
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
                if chess.move_piece(from, to) {
                    // Reflect the move in UI
                    if let Some((mut out_piece, mut out_transform)) = q_piece
                        .iter_mut()
                        .find(|(piece, _)| piece.position.0 == to.0 && piece.position.1 == to.1)
                    {
                        if out_piece.is_white {
                            board.white_out_count = board.white_out_count + 1;
                            out_transform.translation.y = 4.5 * SQUARE_SIZE;
                            out_transform.translation.x = 4. * SQUARE_SIZE
                                - (SQUARE_SIZE * board.white_out_count as f32 / 4.);
                        } else {
                            board.black_out_count = board.black_out_count + 1;
                            out_transform.translation.y = -4.5 * SQUARE_SIZE;
                            out_transform.translation.x = -4. * SQUARE_SIZE
                                + (SQUARE_SIZE * board.black_out_count as f32 / 4.);
                        }
                        out_transform.scale = out_transform.scale * 0.5;
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
                    for (player, mut sprite) in q_player.iter_mut() {
                        if chess.white_turn == player.is_white {
                            sprite.color = GREEN.into();
                        } else {
                            sprite.color = GRAY.into();
                        }
                    }
                    board.update_turn(chess.white_turn);

                    for (text_info, mut transform, _) in q_texts.iter_mut() {
                        if text_info.text_type == 3 && chess.white_turn {
                            transform.scale = Vec3::splat(1.);
                        } else if text_info.text_type == 4 && !chess.white_turn {
                            transform.scale = Vec3::splat(1.);
                        } else {
                            transform.scale = Vec3::splat(0.);
                        }
                    }
                }
                if chess.is_in_check() {
                    if chess.is_checkmate() {
                        let text_val = if chess.white_turn {
                            "Black won"
                        } else {
                            "White Won"
                        };
                        for (text_info, mut transform, mut text) in q_texts.iter_mut() {
                            if text_info.text_type == 1 {
                                transform.scale = Vec3::splat(1.);
                            } else if text_info.text_type == 2 {
                                transform.scale = Vec3::splat(1.);
                                text.sections[0].value = text_val.to_string();
                            }
                        }
                        println!("CHECKMATE!! {} ", text_val)
                    } else {
                        println!("Check!!");
                    }
                }
                board.remove_selected();
                for lmo in q_last_move_overlay.iter() {
                    commands.entity(lmo).despawn();
                }
                if let Some(last_move) = &chess.last_move {
                    for pos_idx in [last_move.0, last_move.1].iter() {
                        let position = Chess::index_to_position(*pos_idx);
                        let blue_square = Sprite {
                            color: TRANSPARENT_PURPLE.into(),
                            custom_size: Some(Vec2::splat(SQUARE_SIZE)),
                            ..Default::default()
                        };
                        let (x_t, y_t) = chess_position_to_world_position(position.clone());

                        // Spawn color for selected square
                        commands.spawn((
                            SpriteBundle {
                                sprite: blue_square,
                                transform: Transform::from_xyz(x_t, y_t, 5.),
                                ..Default::default()
                            },
                            LastMoveOverlay,
                        ));
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
                    is_white: code.chars().nth(0).unwrap() == 'w',
                    code: code.to_string(),
                    position: Position(position.0, position.1),
                },
            ));
        }
    }
}
