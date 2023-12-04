use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use blokus::blokus::game::{Grid, Point};
use blokus::blokus::game::collides;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(Update, (piece_selection, piece_placement))
        .add_systems(Update, (piece_selection, piece_hover))
        .insert_resource(SelectedPiece { entity: None })
        .insert_resource(GameLogic {grid: Grid::parse("..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........")})
        .run();
}

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

const GRID_SQUARES: isize = 10;
const GRID_SIZE: f32 = GRID_SQUARES as f32;
const SQUARE_SIZE: f32 = 30.0;
const PAD_SIZE: f32 = 5.0;
const SQUARE_PLUS_PAD_SIZE: f32 = SQUARE_SIZE + PAD_SIZE;
const BOARD_SIZE: f32 = GRID_SIZE * SQUARE_PLUS_PAD_SIZE;
const BOARD_OFFSET: Vec2 = Vec2::new(0.0, 0.0);

fn setup(
    mut commands: Commands,
) {
    // Create a camera
    commands.spawn(Camera2dBundle::default());

    // Create the grid
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(
            BOARD_OFFSET.x,
            BOARD_OFFSET.y,
            0.0,
        ),
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(BOARD_SIZE + (PAD_SIZE * 2.0), BOARD_SIZE + (PAD_SIZE * 2.0))),
            ..default()
        },
        ..Default::default()
    });
    for i in 0..GRID_SQUARES {
        for j in 0..GRID_SQUARES {
            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(
                    i as f32 * SQUARE_PLUS_PAD_SIZE - (BOARD_SIZE / 2.0) + (SQUARE_PLUS_PAD_SIZE / 2.0),
                    j as f32 * SQUARE_PLUS_PAD_SIZE - (BOARD_SIZE / 2.0) + (SQUARE_PLUS_PAD_SIZE / 2.0),
                    0.0,
                ),
                sprite: Sprite {
                    color: Color::rgb(0.75, 0.75, 0.75),
                    custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                    ..default()
                },
                ..Default::default()
            });
        }
    }

    // Load and display Blokus pieces (placeholder logic)
    let sprite = Sprite {
        color: Color::rgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
        ..default()
    };
    let transform = Transform::from_xyz(0.0, -200.0, 0.0);
    // commands.spawn(GamePiece {
    //     id: 0,
    // });
    commands.spawn(SpriteBundle {
        sprite,
        transform,
        ..Default::default()
    }).insert(GamePiece {
        id: 0,
        valid: true,
    });

    // Add more game setup logic here if needed
}

#[derive(Debug, Resource)]
struct GameLogic {
    grid: Grid
}

// Assuming you have a struct to represent a game piece
#[derive(Component)]
struct GamePiece {
    id: usize,
    valid: bool,
}

// Component to identify selectable pieces
#[derive(Component)]
struct Selectable;

// Resource to keep track of selected piece
#[derive(Resource)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn piece_selection(
    mouse_input: Res<Input<MouseButton>>,
    query: Query<(Entity, &GamePiece, &Sprite, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        let mut newly_selected_entity = None;
        if let Some(cursor_pos) = window.cursor_position() {
            let world_pos = mouse_to_game(Vec2::new(window.width() as f32, window.height() as f32), cursor_pos);

            for (entity, piece, sprite, transform) in query.iter() {
                let mut is_selected = false;
                if let Some(selected_entity) = selected_piece.entity {
                    is_selected = selected_entity.index() == entity.index();
                }
                // Calculate the size of the piece based on its properties
                let piece_size = sprite.custom_size.unwrap(); // Placeholder

                // Check if the cursor is over this piece
                if !is_selected && (
                    world_pos.x > transform.translation.x - piece_size.x / 2.0 &&
                    world_pos.x < transform.translation.x + piece_size.x / 2.0 &&
                    world_pos.y > transform.translation.y - piece_size.y / 2.0 &&
                    world_pos.y < transform.translation.y + piece_size.y / 2.0) {
                        newly_selected_entity = Some(entity);
                        println!("Selected piece id: {}", piece.id);
                        break;
                }
            }
        }
        selected_piece.entity = newly_selected_entity;
    }
}

fn mouse_to_game(window_size: Vec2, cursor_pos: Vec2) -> Vec2 {
    (cursor_pos - window_size / 2.0) * Vec2::new(1.0, -1.0)
}

fn game_to_mouse(window_size: Vec2, game_pos: Vec2) -> Vec2 {
    (game_pos / Vec2::new(1.0, -1.0)) + (window_size / 2.0)
}

fn piece_hover(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    selected_piece: ResMut<SelectedPiece>,
    mut piece_query: Query<(&mut Transform, &mut GamePiece, &mut Sprite)>,
    game_logic: Res<GameLogic>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(selected_entity) = selected_piece.entity {
            if let Ok((mut selected_piece_transform, mut selected_game_piece, mut sprite)) = piece_query.get_mut(selected_entity) {
                let world_pos = mouse_to_game(Vec2::new(window.width() as f32, window.height() as f32), cursor_pos);

                // Place the piece at the grid square's position
                let world_game_pos = to_grid(world_pos);
                if in_bounds(world_game_pos) { // TODO: if the whole piece is in bounds
                    // Update position
                    let old_game_pos = to_grid(selected_piece_transform.translation.truncate());
                    let mut new_pos = to_game(world_game_pos);
                    // Translate to center of sprite
                    let num_squares = 1;
                    let piece_size = SQUARE_PLUS_PAD_SIZE * num_squares as f32;
                    new_pos.x = new_pos.x + (0.5 * (piece_size));
                    new_pos.y = new_pos.y - (0.5 * (piece_size));
                    selected_piece_transform.translation = new_pos.extend(0.0);

                    // Update validity
                    selected_game_piece.valid = if in_bounds(old_game_pos) {
                        // check if different than old position
                        // if different, check validity and update validity
                        if old_game_pos != world_game_pos {
                            is_piece_placement_valid(world_game_pos, &game_logic.grid)
                        }
                        else {
                            selected_game_piece.valid
                        }
                    } else {
                        false
                    };
                    if selected_game_piece.valid {
                        sprite.color = Color::rgb(1.0, 0.0, 0.0);
                    } else {
                        sprite.color = Color::rgb(0.0, 0.0, 1.0);
                    }
                }
            }
        }
    }
}

fn in_bounds(pos: Point) -> bool {
    return pos.row >= 0 && pos.col >= 0 &&
        pos.row < GRID_SQUARES && pos.col < GRID_SQUARES
}

fn is_piece_placement_valid(grid_position: Point,
    grid: &Grid,
    // game_move: &Move,
) -> bool {
    // let piece_point = Point::new(1, 1);
    // let piece = Grid::parse("x..\n\
    // x..\n\
    // xxx");
    
    let piece_point = Point::new(0, 0);
    let piece = Grid::parse("x");

    !collides(grid, &piece, grid_position, piece_point)
}

fn to_grid(mut game_pos: Vec2) -> Point {
    game_pos.y = -game_pos.y;
    let grid_pos = (game_pos + (BOARD_SIZE/2.0) - BOARD_OFFSET) / SQUARE_PLUS_PAD_SIZE;
    return Point::new(grid_pos.y as isize, grid_pos.x as isize);
}

fn to_game(grid_pos: Point) -> Vec2 {
    let mut grid_pos = BOARD_OFFSET + (Vec2::new(grid_pos.col as f32, grid_pos.row as f32) * SQUARE_PLUS_PAD_SIZE) - (BOARD_SIZE/2.0);
    grid_pos.y = -grid_pos.y;
    return grid_pos;
}

#[cfg(test)]
mod coordinate_tests {
    use bevy::math::Vec2;
    use blokus::blokus::game::Point;

    use crate::{SQUARE_PLUS_PAD_SIZE, to_grid, GRID_SQUARES, to_game};

    #[test]
    fn test_to_grid_q4() {
        let game_pos = Vec2::new(SQUARE_PLUS_PAD_SIZE * 3.0, SQUARE_PLUS_PAD_SIZE * 2.0);
        let grid_pos = to_grid(game_pos);
        assert_eq!(grid_pos, Point { col: 3 + GRID_SQUARES/2, row: GRID_SQUARES/2 - 2 });
    }

    #[test]
    fn test_to_game_q2() {
        let grid_pos = Point { col: 1, row: 2 };
        let game_pos = to_game(grid_pos);
        assert_eq!(game_pos, Vec2::new(-SQUARE_PLUS_PAD_SIZE*((GRID_SQUARES/2 - 1) as f32), SQUARE_PLUS_PAD_SIZE*((GRID_SQUARES/2 - 2) as f32)));
    }

    #[test]
    fn test_to_game_q4() {
        let grid_pos = Point { col: GRID_SQUARES/2 + 3, row: GRID_SQUARES/2 + 2 };
        let game_pos = to_game(grid_pos);
        assert_eq!(game_pos, Vec2::new(SQUARE_PLUS_PAD_SIZE*3.0, -SQUARE_PLUS_PAD_SIZE*2.0));
    }
}

// Assuming the same GamePiece, Selectable, and SelectedPiece structures from before

// #[derive(Component)]
// struct GridSquare {
//     x: usize,
//     y: usize,
// }

// fn piece_placement(
//     mut commands: Commands,
//     mouse_input: Res<Input<MouseButton>>,
//     mut selected_piece: ResMut<SelectedPiece>,
//     grid_query: Query<(&Transform, &GridSquare)>,
//     piece_query: Query<&Transform, With<GamePiece>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//     let Ok(window) = window_query.get_single() else {
//         return;
//     };

//     if mouse_input.just_pressed(MouseButton::Left) {
//         if let Some(cursor_pos) = window.cursor_position() {
//             if let Some(selected_entity) = selected_piece.entity {
//                 for (transform, grid_square) in grid_query.iter() {
//                     // Placeholder size for grid squares
//                     let square_size = Vec2::new(30.0, 30.0);

//                     // Check if the cursor is over this grid square
//                     if cursor_pos.x > transform.translation.x - square_size.x / 2.0 &&
//                         cursor_pos.x < transform.translation.x + square_size.x / 2.0 &&
//                         cursor_pos.y > transform.translation.y - square_size.y / 2.0 &&
//                         cursor_pos.y < transform.translation.y + square_size.y / 2.0 {
                            
//                             // Get the transform of the selected piece
//                             if let Ok(selected_piece_transform) = piece_query.get(selected_entity) {
//                                 // Place the piece at the grid square's position
//                                 commands.entity(selected_entity).insert(Transform {
//                                     translation: transform.translation,
//                                     ..selected_piece_transform.clone()
//                                 });

//                                 // Reset the selected piece
//                                 selected_piece.entity = None;
//                                 println!("Placed piece at grid position: ({}, {})", grid_square.x, grid_square.y);
//                                 break;
//                             }
//                     }
//                 }
//             }
//         }
//     }
// }
