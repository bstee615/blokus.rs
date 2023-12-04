use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::blokus::game::{Grid, Point, collides};

use super::globals::*;
use super::coordinates::*;

#[derive(Component)]
pub struct GamePiece {
    id: usize,
    valid: bool,
}

#[derive(Resource)]
pub struct GameLogic {
    pub grid: Grid
}

#[derive(Resource)]
pub struct SelectedPiece {
    pub entity: Option<Entity>,
}

pub fn setup(
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
    add_piece(&mut commands, 0.0, -200.0, 0);
    add_piece(&mut commands, 50.0, -200.0, 1);
}

fn add_piece(commands: &mut Commands, x: f32, y: f32, id: usize) {
    let sprite = Sprite {
        color: Color::rgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
        ..default()
    };
    let transform = Transform::from_xyz(x, y, 0.0);
    commands.spawn(SpriteBundle {
        sprite,
        transform,
        ..Default::default()
    }).insert(GamePiece {
        id: id,
        valid: true,
    });
}

fn print_grid(grid: &Grid) {
    // Display the array for demonstration
    print!("  ");
    for j in 0..grid.width() {
        print!("{} ", j);
    }
    println!();
    for i in 0..grid.height() {
        print!("{} ", i);
        for j in 0..grid.width() {
            print!("{} ", grid.cell(i, j));
        }
        println!();
    }
}

pub fn piece_selection(
    mouse_input: Res<Input<MouseButton>>,
    query: Query<(Entity, &GamePiece, &Sprite, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut game_logic: ResMut<GameLogic>,
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
        // Place piece on board
        if newly_selected_entity.is_none() {
            if let Some(selected_entity) = selected_piece.entity {
                if let Ok((_, piece, _, transform)) = query.get(selected_entity) {
                    let grid_pos = game_to_grid(transform.translation.truncate());
                    if piece.valid {
                        println!("Placed at: {:?}", grid_pos);
                        // TODO: place entire piece
                        game_logic.grid.set_cell(grid_pos.row, grid_pos.col, 'x');
                        println!("Updated grid:");
                        print_grid(&game_logic.grid);
                    }
                    else {
                        println!("Prevented invalid placement: {:?}", grid_pos);
                        newly_selected_entity = selected_piece.entity
                    }
                }
            }
        }
        selected_piece.entity = newly_selected_entity;
    }
}

pub fn piece_hover(
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
                let world_game_pos = game_to_grid(world_pos);
                if in_bounds(world_game_pos) { // TODO: if the whole piece is in bounds
                    // Update position
                    let old_game_pos = game_to_grid(selected_piece_transform.translation.truncate());
                    let mut new_pos = grid_to_game(world_game_pos);
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
