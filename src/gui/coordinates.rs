/*
There are three coordinate systems:

1. Window coordinates:
- Sprite positions are at the center of the sprite

        +y
        ^
        |
        |
-x <----(0,0)----> +x
        |
        |
        v
        -y

2. Mouse coordinates

            -y
    (0,0)    ^
            |
            |
-x <------+------> +x
            |
            |
            v
            +y

3. Game board coordinates

        -row
    (0,0)    ^
            |
            |
-col <------+------> +col
            |
            |
            v
        +row

*/

use bevy::math::Vec2;

use crate::blokus::game::Point;

use super::globals::*;

pub fn mouse_to_game(window_size: Vec2, cursor_pos: Vec2) -> Vec2 {
    (cursor_pos - window_size / 2.0) * Vec2::new(1.0, -1.0)
}

pub fn game_to_mouse(window_size: Vec2, game_pos: Vec2) -> Vec2 {
    (game_pos / Vec2::new(1.0, -1.0)) + (window_size / 2.0)
}

pub fn game_to_grid(mut game_pos: Vec2) -> Point {
    game_pos.y = -game_pos.y;
    let grid_pos = (game_pos + (BOARD_SIZE/2.0) - BOARD_OFFSET) / SQUARE_PLUS_PAD_SIZE;
    return Point::new(grid_pos.y as isize, grid_pos.x as isize);
}

pub fn grid_to_game(grid_pos: Point) -> Vec2 {
    let mut grid_pos = BOARD_OFFSET + (Vec2::new(grid_pos.col as f32, grid_pos.row as f32) * SQUARE_PLUS_PAD_SIZE) - (BOARD_SIZE/2.0);
    grid_pos.y = -grid_pos.y;
    return grid_pos;
}

#[cfg(test)]
mod coordinate_tests {
    use bevy::math::Vec2;
    use blokus::game::Point;

    use crate::blokus;
    use super::super::globals::*;
    use super::*;

    #[test]
    fn test_to_grid_q4() {
        let game_pos = Vec2::new(SQUARE_PLUS_PAD_SIZE * 3.0, SQUARE_PLUS_PAD_SIZE * 2.0);
        let grid_pos = game_to_grid(game_pos);
        assert_eq!(grid_pos, Point { col: 3 + GRID_SQUARES/2, row: GRID_SQUARES/2 - 2 });
    }

    #[test]
    fn test_to_game_q2() {
        let grid_pos = Point { col: 1, row: 2 };
        let game_pos = grid_to_game(grid_pos);
        assert_eq!(game_pos, Vec2::new(-SQUARE_PLUS_PAD_SIZE*((GRID_SQUARES/2 - 1) as f32), SQUARE_PLUS_PAD_SIZE*((GRID_SQUARES/2 - 2) as f32)));
    }

    #[test]
    fn test_to_game_q4() {
        let grid_pos = Point { col: GRID_SQUARES/2 + 3, row: GRID_SQUARES/2 + 2 };
        let game_pos = grid_to_game(grid_pos);
        assert_eq!(game_pos, Vec2::new(SQUARE_PLUS_PAD_SIZE*3.0, -SQUARE_PLUS_PAD_SIZE*2.0));
    }
}
