use std::collections::HashSet;
type BlokusGrid = Vec<Vec<char>>;
type BlokusPoint = (isize, isize);

fn create_grid() -> BlokusGrid {
    let mut array = vec![vec!['.'; 10]; 10];
    array[0][0] = 'x';
    array[4][3] = 'x';
    array[5][3] = 'x';
    array[6][3] = 'x';
    array[4][2] = 'x';
    array[4][1] = 'x';
    array
}
fn create_piece() -> BlokusGrid {
    let mut array = vec![vec!['.'; 1]; 3];
    array[0][0] = 'x';
    array[1][0] = 'x';
    array[2][0] = 'x';
    array
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

fn get_moves(grid: &BlokusGrid, piece: &BlokusGrid) -> Vec<BlokusPlacement> {
    let mut moves = HashSet::new();

    // For each corner
    let grid_corners = detect_corners(grid);
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
                    println!("Skip vertical ({}, {}) <-> ({}, {}) {} - {} < {} || {} >= {}", gc.0, gc.1, pc.0, pc.1, degree, gc.0 - pc.0, 0, gc.0 + (piece.len() as isize - pc.0), grid.len());
                    continue;
                }
                if gc.1 - pc.1 < 0 || gc.1 + (rotated_piece[0].len() as isize - pc.1 - 1) >= grid[0].len() as isize {
                    println!("Skip horizontal ({}, {}) <-> ({}, {}) {}", gc.0, gc.1, pc.0, pc.1, degree);
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
    use crate::BlokusGrid;
    use crate::detect_marks;

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

    fn parse_grid(multi_line_string: &str) -> BlokusGrid {
        multi_line_string
            .lines()
            .map(|line| line.chars().collect())
            .collect()
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
    let my_grid = create_grid();
    print_grid(&my_grid);

    for (ci, cj) in detect_corners(&my_grid) {
        println!("Corner: ({}, {})", ci, cj);
    }
    
    let my_grid_rotate = rotate(&my_grid, 180);
    println!("Rotated");
    print_grid(&my_grid_rotate);

    let piece = create_piece();
    print_grid(&piece);
    for (ci, cj) in detect_corners(&piece) {
        println!("Corner: ({}, {})", ci, cj);
    }
    
    let moves = get_moves(&my_grid, &piece);
    println!("Moves: {}", moves.len());
    for (piece, gc, pc) in moves {
        let mut grid_clone = my_grid.clone();
        println!("Placement point: grid=({}, {}) piece=({}, {})", gc.0, gc.1, pc.0, pc.1);
        println!("Piece");
        print_grid(&piece);
        println!("Board");
        for i in 0..piece.len() {
            for j in 0..piece[0].len() {
                if piece[i][j] == 'x' && gc.0 >= 0 && gc.1 >= 0 {
                    grid_clone[(gc.0+(i as isize)-pc.0) as usize][(gc.1+(j as isize)-pc.1) as usize] = 'o';
                }
            }
        }
        print_grid(&grid_clone);
    }
}
