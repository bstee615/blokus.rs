use std::collections::HashSet;
use std::io::stdin;

type BlokusGrid = Vec<Vec<char>>;
type BlokusPoint = (isize, isize);

fn create_grid() -> BlokusGrid {
    vec![vec!['.'; 10]; 10]
}

fn parse_grid(multi_line_string: &str) -> BlokusGrid {
    multi_line_string
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

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

fn get_cell(grid: &BlokusGrid, i: isize, j: isize) -> &char {
    if i < 0 || i >= grid.len() as isize {
        &'-'
    }
    else if j < 0 || j >= grid[0].len() as isize {
        &'-'
    }
    else {
        &grid[i as usize][j as usize]
    }
}

fn get_corners(grid: &BlokusGrid, i: isize, j: isize) -> Vec<BlokusPoint> {
    let mut corners = Vec::new();
    if get_cell(grid, i-1, j) == &'.' && get_cell(grid, i, j-1) == &'.' { corners.push((i-1, j-1)); }
    if get_cell(grid, i-1, j) == &'.' && get_cell(grid, i, j+1) == &'.' { corners.push((i-1, j+1)); }
    if get_cell(grid, i, j-1) == &'.' && get_cell(grid, i+1, j) == &'.' { corners.push((i+1, j-1)); }
    if get_cell(grid, i, j+1) == &'.' && get_cell(grid, i+1, j) == &'.' { corners.push((i+1, j+1)); }

    corners
}


fn detect_corners(grid: &BlokusGrid) -> Vec<BlokusPoint> {
    let mut points = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'x' {
                let corners = get_corners(grid, i as isize, j as isize);
                if corners.len() > 0 {
                    for &(ci, cj) in corners.iter() {
                        points.push((ci, cj));
                    }
                }
            }
        }
    }

    points
}

fn rotate(grid: &BlokusGrid, mut degrees: i32) -> BlokusGrid {
    degrees %= 360;

    let mut new_grid = match degrees {
        0 | 180 | 360 => vec![vec!['@'; grid[0].len()]; grid.len()],
        90 | 270 => vec![vec!['@'; grid.len()]; grid[0].len()],
        _ => {
            panic!("Invalid degrees: {}", degrees)
        },
    };

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match degrees {
                0 => {
                    new_grid[i][j] = grid[i][j];
                }
                90 => {
                    new_grid[j][grid.len() - i - 1] = grid[i][j];
                }
                180 => {
                    new_grid[grid.len() - i - 1][grid[0].len() - j - 1] = grid[i][j]
                }
                270 => {
                    new_grid[grid[0].len() - j - 1][i] = grid[i][j]
                }
                _ => {}
            }
        }
    }

    new_grid
}

type BlokusPlacement = (BlokusGrid, BlokusPoint, BlokusPoint);

fn get_moves(grid: &BlokusGrid, piece: &BlokusGrid, turn: &i32) -> Vec<BlokusPlacement> {
    let mut moves = HashSet::new();

    // For each corner
    let grid_corners = if turn == &0 {
        vec![
            (0, 0),
            (grid.len() as isize, 0),
            (0, grid[0].len() as isize),
            (grid.len() as isize, grid[0].len() as isize),
        ]
    }
    else {
        detect_corners(grid)
    };
    // For rotation
    for degree in vec![0, 90, 180, 270] {
        let rotated_piece = rotate(piece, degree);

        // Find all marked spots on piece
        let piece_marks = detect_marks(&rotated_piece);
        // Try against all corners
        for &gc in &grid_corners {
            for &pc in &piece_marks {
                let this_move = (rotated_piece.clone(), gc, pc);
                if moves.contains(&this_move) {
                    continue;
                }
                if gc.0 - pc.0 < 0 || gc.0 + (rotated_piece.len() as isize - pc.0 - 1) >= grid.len() as isize {
                    continue;
                }
                if gc.1 - pc.1 < 0 || gc.1 + (rotated_piece[0].len() as isize - pc.1 - 1) >= grid[0].len() as isize {
                    continue;
                }
                // Check collisions
                if !collides(grid, &rotated_piece, gc, pc) {
                    moves.insert(this_move);
                }
            }
        }
    }

    // Return piece, point to place
    moves.into_iter().collect()
}

fn detect_marks(grid: &BlokusGrid) -> Vec<BlokusPoint> {
    let mut marks = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'x' {
                marks.push((i as isize, j as isize));
            }
        }
    }

    marks
}

#[cfg(test)]
mod tests {
    use crate::collides;
    use crate::detect_marks;
    use crate::parse_grid;

    #[test]
    fn it_works() {
        let grid = 
            vec![
                vec!['.','.','.','.','.'],
                vec!['.','x','.','.','.'],
                vec!['.','x','.','.','.'],
                vec!['.','.','.','.','.'],
                vec!['.','.','.','.','.'],];
        let piece = 
            vec![
                vec!['x',],
                vec!['x',],];
        let gc = (3, 2);
        assert!(collides(&grid, &piece, gc, (1, 0)));
        assert!(!collides(&grid, &piece, gc, (0, 0)));
    }

    #[test]
    fn it_works_now() {
        let grid = 
            vec![
                vec!['.','.','.','.','.'],
                vec!['.','x','.','.','.'],
                vec!['.','x','.','.','.'],
                vec!['.','x','.','.','.'],
                vec!['.','.','.','.','.'],];
        let piece = 
            vec![
                vec!['x',],
                vec!['x',],
                vec!['x',],];
        let gc = (0, 2);
        assert!(collides(&grid, &piece, gc, (0, 0)));
    }

    #[test]
    #[should_panic]
    fn fails() {
        let grid = 
            vec![
                vec!['.','.','.','.','.'],
                vec!['.','x','.','.','.'],
                vec!['.','x','.','.','.'],
                vec!['.','.','.','.','.'],];
        let piece = 
            vec![
                vec!['x',],
                vec!['x',],];
        collides(&grid, &piece, (3, 2), (0, 0));
    }

    #[test]
    fn fails2() {
        let grid = parse_grid(
            "x.........\n\
             ..........\n\
             ..........\n\
             ..........\n\
             .xxx......\n\
             ...x......\n\
             ...x......\n\
             ..........\n\
             ..........\n\
             ..........\n\
             "
        );
        let piece = parse_grid(
            "x\n\
             x\n\
             x"
        );
        assert!(collides(&grid, &piece, (7, 4), (0, 2)));
    }

    #[test]
    fn fails3() {
        let piece = parse_grid(
            "x\n\
             x\n\
             x"
        );
        assert_eq!(detect_marks(&piece), vec![(0, 0), (1, 0), (2, 0)]);
    }

    #[test]
    fn fails4() {
        let piece = parse_grid(
            "x..\n\
             x..\n\
             xxx"
        );
        assert_eq!(detect_marks(&piece), vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]);
    }

    #[test]
    fn fails5() {
        let piece = parse_grid(
            "xxx\n\
             x..\n\
             x.."
        );
        assert_eq!(detect_marks(&piece), vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0)]);
    }
}

fn collides(grid: &BlokusGrid, piece: &BlokusGrid, gc: BlokusPoint, pc: BlokusPoint) -> bool {
    for _i in 0..piece.len() {
            for _j in 0..piece[0].len() {
                let i = _i as isize;
                let j = _j as isize;
                let cell = piece[_i][_j];
                if cell == 'x' {
                    // Check in the cell and each of the 4 directions projected to the grid
                    // If it's x, there's a collision
                    for o in vec![(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)] {
                        if get_cell(grid, gc.0+(i-pc.0)+o.0, gc.1+(j-pc.1)+o.1) == &'x' {
                            return true;
                        }
                    }
                    // Not outside grid
                    assert_ne!(get_cell(grid, gc.0+(i-pc.0), gc.1+(j-pc.1)), &'-');
            }
        }
    }
    return false;
}

fn main() {
    let mut grid = create_grid();
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
