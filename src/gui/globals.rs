use bevy::math::Vec2;

pub const GRID_SQUARES: isize = 10;
pub const GRID_SIZE: f32 = GRID_SQUARES as f32;
pub const SQUARE_SIZE: f32 = 30.0;
pub const PAD_SIZE: f32 = 5.0;
pub const SQUARE_PLUS_PAD_SIZE: f32 = SQUARE_SIZE + PAD_SIZE;
pub const BOARD_SIZE: f32 = GRID_SIZE * SQUARE_PLUS_PAD_SIZE;
pub const BOARD_OFFSET: Vec2 = Vec2::new(0.0, 0.0);
