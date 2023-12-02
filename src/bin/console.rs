use std::io::stdin;
extern crate blokus;
use blokus::blokus::game::{*};

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

fn print_move(grid: &Grid, game_move: &Move) {
    let mut grid_clone = grid.clone();
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            if game_move.piece.cell(i, j) == 'x' && game_move.grid_corner.row >= 0 && game_move.grid_corner.col >= 0 {
                grid_clone.set_cell(game_move.grid_corner.row+(i as isize)-game_move.piece_mark.row, game_move.grid_corner.col+(j as isize)-game_move.piece_mark.col, 'o')
            }
        }
    }
    print_grid(&grid_clone);
}

fn main() {
    let mut grid = Grid::parse(
        "..........\n\
         ..........\n\
         ..........\n\
         ..........\n\
         ..........\n\
         ..........\n\
         ..........\n\
         ..........\n\
         ..........\n\
         .........."
    );
    let mut pieces = Vec::new();
    pieces.push(Grid::parse(
        "x\n\
         x\n\
         x"
    ));
    pieces.push(Grid::parse(
        "xxx"
    ));
    pieces.push(Grid::parse(
        "xxx\n\
         x..\n\
         x.."
    ));

    let mut turn = 0;
    loop {
        // Choose piece
        println!("Pieces:");
        for (i, piece) in pieces.iter().enumerate() {
            println!("[{}]", i);
            print_grid(&piece);
        }
        println!("Choose piece: ");
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read piece choice");
        let line_to_parse = line.trim();
        if line_to_parse == "q" {
            println!("Quitting!");
            break;
        }
        let piece_choice = line_to_parse.parse::<usize>().expect("Input is not an index");
        let chosen_piece = &pieces[piece_choice];

        // Choose orientation
        let moves = get_moves(&grid, &chosen_piece, turn);
        println!("Moves: {}", moves.len());
        for (i, m) in moves.iter().enumerate() {
            println!("[{}]", i);
            print_move(&grid, &m);
        }
        println!("Choose move: ");
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read move choice");
        let line_to_parse = line.trim();
        if line_to_parse == "b" {
            continue;
        }
        if line_to_parse == "q" {
            println!("Quitting!");
            break;
        }
        let move_choice = line_to_parse.parse::<usize>().expect("Input is not an index");
        let chosen_move = &moves[move_choice];

        // Play
        let move_piece = &chosen_move.piece;
        let grid_corner = chosen_move.grid_corner;
        let piece_mark = chosen_move.piece_mark;
        for i in 0..move_piece.height() {
            for j in 0..move_piece.width() {
                if move_piece.cell(i, j) == 'x' && grid_corner.row >= 0 && grid_corner.col >= 0 {
                    grid.set_cell(grid_corner.row+(i as isize)-piece_mark.row, grid_corner.col+(j as isize)-piece_mark.col, 'x')
                }
            }
        }
        pieces.remove(piece_choice);
        if pieces.len() == 0 {
            println!("Used all pieces!");
            break;
        }

        println!("Grid:");
        print_grid(&grid);
        turn += 1;
    }
    println!("Final grid:");
    print_grid(&grid);
}
