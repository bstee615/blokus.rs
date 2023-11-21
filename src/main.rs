use std::io::stdin;
mod blokus;
use crate::blokus::game::*;

fn print_grid(grid: &BlokusGrid) {
    // Display the array for demonstration
    print!("  ");
    for (j, _) in grid[0].iter().enumerate() {
        print!("{} ", j);
    }
    println!();
    for (i, row) in grid.iter().enumerate() {
        print!("{} ", i);
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn print_move(grid: &BlokusGrid, piece: &BlokusGrid, gc: &BlokusPoint, pc: &BlokusPoint) {
    let mut grid_clone = grid.clone();
    for i in 0..piece.len() {
        for j in 0..piece[0].len() {
            if piece[i][j] == 'x' && gc.0 >= 0 && gc.1 >= 0 {
                grid_clone[(gc.0+(i as isize)-pc.0) as usize][(gc.1+(j as isize)-pc.1) as usize] = 'o';
            }
        }
    }
    print_grid(&grid_clone);
}

fn main() {
    let mut grid = parse_grid(
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
    pieces.push(parse_grid(
        "x\n\
         x\n\
         x"
    ));
    pieces.push(parse_grid(
        "xxx"
    ));
    pieces.push(parse_grid(
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
        let moves = get_moves(&grid, &chosen_piece, &turn);
        println!("Moves: {}", moves.len());
        for (i, (piece, gc, pc)) in moves.iter().enumerate() {
            println!("[{}]", i);
            print_move(&grid, &piece, gc, pc);
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
        let move_piece = &chosen_move.0;
        let gc = chosen_move.1;
        let pc = chosen_move.2;
        for i in 0..move_piece.len() {
            for j in 0..move_piece[0].len() {
                if move_piece[i][j] == 'x' && gc.0 >= 0 && gc.1 >= 0 {
                    grid[(gc.0+(i as isize)-pc.0) as usize][(gc.1+(j as isize)-pc.1) as usize] = 'x';
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
