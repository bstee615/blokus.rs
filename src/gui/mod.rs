use bevy::prelude::*;

use crate::blokus::game::Grid;

mod game_logic;
mod globals;
mod coordinates;

use game_logic::*;

pub fn get_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
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
        ..........")});
    app
}
