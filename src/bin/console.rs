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

fn print_move(grid: &Grid, game_move: &Move, mark: char) {
    let mut grid_clone = grid.clone();
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            if game_move.piece.cell(i, j) == 'x' && game_move.grid_corner.row >= 0 && game_move.grid_corner.col >= 0 {
                grid_clone.set_cell(game_move.grid_corner.row+(i as isize)-game_move.piece_mark.row, game_move.grid_corner.col+(j as isize)-game_move.piece_mark.col, mark.to_ascii_lowercase())
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

    let mut player1_pieces = get_pieces();
    let mut player2_pieces = get_pieces();

    let mut whose_turn = 1;
    let mut turn_count = 0;
    loop {
        let pieces = {
            if whose_turn == 1 {
                &mut player1_pieces
            }
            else if whose_turn == 2 {
                &mut player2_pieces
            }
            else {
                panic!("Turn should be 1 or 2!");
            }
        };
        let mark = {
            if whose_turn == 1 {
                'A'
            }
            else if whose_turn == 2 {
                'B'
            }
            else {
                panic!("Turn should be 1 or 2!");
            }
        };
        println!("Player turn: {}", mark);

        // Choose piece
        let piece_choice = {
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
            line_to_parse.parse::<usize>().expect("Input is not an index")
        };
        let chosen_piece = &pieces[piece_choice];

        // Choose orientation
        let moves = get_moves(&grid, &chosen_piece, turn_count);
        let move_choice = {
            println!("Moves: {}", moves.len());
            for (i, m) in moves.iter().enumerate() {
                println!("[{}]", i);
                print_move(&grid, &m, mark);
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
            line_to_parse.parse::<usize>().expect("Input is not an index")
        };
        let chosen_move = &moves[move_choice];

        // Play
        let move_piece = &chosen_move.piece;
        let grid_corner = chosen_move.grid_corner;
        let piece_mark = chosen_move.piece_mark;
        for i in 0..move_piece.height() {
            for j in 0..move_piece.width() {
                if move_piece.cell(i, j) == 'x' && grid_corner.row >= 0 && grid_corner.col >= 0 {
                    grid.set_cell(grid_corner.row+(i as isize)-piece_mark.row, grid_corner.col+(j as isize)-piece_mark.col, mark)
                }
            }
        }
        pieces.remove(piece_choice);
        if player1_pieces.len() == 0 && player2_pieces.len() == 0 {
            println!("Used all pieces!");
            break;
        }

        println!("Grid:");
        print_grid(&grid);
        turn_count += 1;
        whose_turn = {
            if whose_turn == 1 && player2_pieces.len() > 0 {
                2
            }
            else if whose_turn == 2 && player1_pieces.len() > 0 {
                1
            }
            else {
                panic!("Turn should be 1 or 2!");
            }
        };
    }
    println!("Final grid:");
    print_grid(&grid);
}

fn get_pieces() -> Vec<Grid> {
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
    pieces
}
