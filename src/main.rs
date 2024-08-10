use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::*;
use phf::phf_map;

const BLACK: Color = Color::rgb(0.7176470588235294, 0.7529411764705882, 0.8470588235294118);
const WHITE: Color = Color::rgb(0.9098039215686274, 0.9294117647058824, 0.9764705882352941);
const BLUE: Color = Color::rgba(0.4823529411764706, 0.3803921568627451, 1., 0.8);

const SQUARE_SIZE: f32 = 64.;

const LEFT: f32 = -SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;
const BOTTOM: f32 = -SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;

static CLEAR_RANK: phf::Map<&'static str, &'static u64> = phf_map! {
    "RANK1" => &18446744073709551360,
    "RANK2" => &18446744073709486335,
    "RANK3" => &18446744073692839935,
    "RANK4" => &18446744069431361535,
    "RANK5" => &18446742978492891135,
    "RANK6" => &18446463698244468735,
    "RANK7" => &18374967954648334335,
    "RANK8" => &72057594037927935
};

static MASK_RANK: phf::Map<&'static str, &'static u64> = phf_map! {
    "RANK1" => &255,
    "RANK2" => &65280,
    "RANK3" => &16711680,
    "RANK4" => &4278190080,
    "RANK5" => &1095216660480,
    "RANK6" => &280375465082880,
    "RANK7" => &71776119061217280,
    "RANK8" => &18374686479671623680,
};

static CLEAR_FILE: phf::Map<&'static str, &'static u64> = phf_map! {
    "FILE1" => &9187201950435737471,
    "FILE2" => &13816973012072644543,
    "FILE3" => &16131858542891098079,
    "FILE4" => &17289301308300324847,
    "FILE5" => &17868022691004938231,
    "FILE6" => &18157383382357244923,
    "FILE7" => &18302063728033398269,
    "FILE8" => &18374403900871474942,
};

static MASK_FILE: phf::Map<&'static str, &'static u64> = phf_map! {
    "FILE1" => &9259542123273814144,
    "FILE2" => &4629771061636907072,
    "FILE3" => &2314885530818453536,
    "FILE4" => &1157442765409226768,
    "FILE5" => &578721382704613384,
    "FILE6" => &289360691352306692,
    "FILE7" => &144680345676153346,
    "FILE8" => &72340172838076673
};

#[derive(Debug)]
struct Position(u8, u8);

#[derive(Debug)]
enum CastlingStatus {
    Available,
    QueenSide,
    KingSide,
}

#[derive(Debug)]
struct Chess {
    pieces: [u64; 12],
    white_turn: bool,
    castling_status: [CastlingStatus; 2],
    possible_enpassant: u8,
}

fn print_bits(x: u64) {
    for byte in x.to_be_bytes().iter() {
        println!("{:08b}", byte);
    }
    println!();
}

impl Chess {
    fn new() -> Chess {
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

    fn get_possible_moves(position_idx: u8) -> u64 {
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

#[derive(Event, Debug)]
struct SelectSquareEvent(Position);

#[derive(Component, Debug)]
struct Square {
    position: Position,
    coordinates: (f32, f32),
}

#[derive(Component, Debug)]
struct Overlay;

fn spawn_board(mut commands: Commands) {
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
                Square {
                    position: Position(j, i),
                    coordinates: (x_t, y_t),
                },
            ));
        }
    }
}

fn handle_overlays(
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

fn handle_input(
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

struct ChessBoardPlugin;

impl Plugin for ChessBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_board,))
            .add_event::<SelectSquareEvent>()
            .add_systems(
                Update,
                (
                    handle_input.run_if(input_just_pressed(MouseButton::Left)),
                    handle_overlays,
                ),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        ..default()
    },));
}

struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, ChessBoardPlugin))
        .run();
}
