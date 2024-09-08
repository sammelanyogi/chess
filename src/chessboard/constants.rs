use bevy::color::*;

pub const BLACK: Srgba = Srgba::rgb(0.7176470588235294, 0.7529411764705882, 0.8470588235294118);
pub const WHITE: Srgba = Srgba::rgb(0.9098039215686274, 0.9294117647058824, 0.9764705882352941);
pub const BLUE: Srgba = Srgba::new(0.4823529411764706, 0.3803921568627451, 1., 0.8);

pub const DARK_BLACK: Srgba = Srgba::rgb(0., 0., 0.);

pub const GRAY: Srgba = Srgba::new(0.62, 0.62, 0.62, 1.);
pub const GREEN: Srgba = Srgba::new(0.376, 0.922, 0.212, 1.);
pub const TRANSPARENT_PURPLE: Srgba = Srgba::new(0.524, 0., 0.524, 0.7);

pub const SQUARE_SIZE: f32 = 64.;

pub const LEFT: f32 = -SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;
pub const BOTTOM: f32 = -SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;

pub const PIECES_CODE: [&str; 12] = [
    "wp", "wr", "wn", "wb", "wq", "wk", "bp", "br", "bn", "bb", "bq", "bk",
];
