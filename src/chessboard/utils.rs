use super::constants::*;
use crate::engine::chess::Position;

pub fn get_indices_of_set_bits(value: u64) -> Vec<u8> {
    let mut number = value;
    let mut indices = Vec::new();
    let mut index = 0;

    while number > 0 {
        if number & 1 == 1 {
            indices.push(index);
        }
        number >>= 1;
        index += 1;
    }

    indices
}

pub fn chess_position_to_world_position(chess_position: Position) -> (f32, f32) {
    let x_t = LEFT + chess_position.0 as f32 * SQUARE_SIZE;
    let y_t = BOTTOM + chess_position.1 as f32 * SQUARE_SIZE;
    return (x_t, y_t);
}

pub fn world_position_to_chess_position(world_position: (f32, f32)) -> Option<Position> {
    let (x, y) = (
        ((world_position.0 + SQUARE_SIZE * 4.) / SQUARE_SIZE).trunc() as i8 + 1,
        ((world_position.1 + SQUARE_SIZE * 4.) / SQUARE_SIZE).trunc() as i8 + 1,
    );

    if  x <= 8 && x >= 1 && y <= 8 && y >= 1  {
        return Some(Position(x as u8, y as u8))
    }
    return None

}
