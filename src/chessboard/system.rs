use super::component::*;
use super::constants::*;
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

pub fn handle_overlays(
    mut commands: Commands,
    mut ev_select_square: EventReader<SelectSquareEvent>,
    q_overlay: Query<Entity, With<Overlay>>,
) {
    for ev in ev_select_square.read() {
        let position = &ev.0;
        for overlay in q_overlay.iter() {
            commands.entity(overlay).despawn();
        }
        let blue_square = Sprite {
            color: BLUE,
            custom_size: Some(Vec2::splat(SQUARE_SIZE)),
            ..Default::default()
        };
        let x_t = LEFT + position.0 as f32 * SQUARE_SIZE;
        let y_t = BOTTOM + position.1 as f32 * SQUARE_SIZE;
        println!("Event: {:?}", position);

        commands.spawn((
            SpriteBundle {
                sprite: blue_square,
                transform: Transform::from_xyz(x_t, y_t, 0.),
                ..Default::default()
            },
            Overlay,
        ));
    }
}

pub fn handle_input(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_square: Query<&Square>,
    mut ev_select_square: EventWriter<SelectSquareEvent>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.get_single().unwrap();
    println!("Squares: {:?}", q_square);
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
                ev_select_square.send(SelectSquareEvent(Position(x as u8, y as u8)));
            }

            eprintln!("World coords: {:?} ({}, {})", world_position, x, y);
        }
    }
}

pub fn spawn_pieces() {
    // Spawn Pieces logic implementations
    println!("Pieces Spawned");
}
