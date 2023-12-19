// Refactored 2023/12/01 https://chat.openai.com/share/60539e8b-c1e3-4637-9074-23905b8e6d41

use std::collections::HashSet;
use std::ops;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(cells: Vec<Vec<char>>) -> Self {
        Grid { cells }
    }

    pub fn parse(multi_line_string: &str) -> Self {
        let cells = multi_line_string
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Grid { cells }
    }

    pub fn set_cell(&mut self, i: isize, j: isize, cell: char) {
        if i < 0 || i >= self.cells.len() as isize || j < 0 || j >= self.cells[0].len() as isize {
            panic!("Out of bounds: ({}, {})", i, j);
        } else {
            self.cells[i as usize][j as usize] = cell
        }
    }

    pub fn cell(&self, i: isize, j: isize) -> char {
        if i < 0 || i >= self.cells.len() as isize || j < 0 || j >= self.cells[0].len() as isize {
            '-'
        } else {
            self.cells[i as usize][j as usize]
        }
    }

    pub fn width(&self) -> isize {
        self.cells[0].len() as isize
    }

    pub fn height(&self) -> isize {
        self.cells.len() as isize
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct Move {
    pub piece: Grid,
    pub grid_corner: Point,
    pub piece_mark: Point,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point {
    pub col: isize,
    pub row: isize,
}

impl Point {
    pub fn new(row: isize, col: isize) -> Self {
        Point { row, col }
    }
}

impl ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Point { row: self.row + _rhs.row, col: self.col + _rhs.col }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self {
        Point { row: self.row - _rhs.row, col: self.col - _rhs.col }
    }
}

pub fn get_moves(grid: &Grid, piece: &Grid, turn: i32) -> Vec<Move> {
    let mut moves = HashSet::new();

    let grid_corners = if turn == 0 {
        vec![
            Point::new(0, 0),
            Point::new(grid.height(), 0),
            Point::new(0, grid.width()),
            Point::new(grid.height(), grid.width()),
        ]
    } else {
        detect_corners(grid)
    };

    for degree in vec![0, 90, 180, 270] {
        let rotated_piece = rotate(piece, degree);
        let piece_marks = detect_marks(&rotated_piece);

        for &gc in &grid_corners {
            for &pc in &piece_marks {
                let this_move = Move {
                    piece: rotated_piece.clone(),
                    grid_corner: gc,
                    piece_mark: pc,
                };
                if moves.contains(&this_move) {
                    continue;
                }
                if gc.row - pc.row < 0 || gc.row + (rotated_piece.height() - pc.row - 1) >= grid.height() {
                    continue;
                }
                if gc.col - pc.col < 0 || gc.col + (rotated_piece.width() - pc.col - 1) >= grid.width() {
                    continue;
                }
                if !collides(grid, &rotated_piece, gc, pc) {
                    moves.insert(this_move);
                }
            }
        }
    }
    moves.into_iter().collect()
}

fn detect_corners(grid: &Grid) -> Vec<Point> {
    let mut points = Vec::new();
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            if grid.cell(i, j) != '.' && grid.cell(i, j) != '-' {
                points.extend(get_corners(grid, i, j));
            }
        }
    }
    points
}

fn get_corners(grid: &Grid, i: isize, j: isize) -> Vec<Point> {
    let mut corners = Vec::new();
    if grid.cell(i-1, j) == '.' && grid.cell(i, j-1) == '.' { corners.push(Point::new(i-1, j-1)); }
    if grid.cell(i-1, j) == '.' && grid.cell(i, j+1) == '.' { corners.push(Point::new(i-1, j+1)); }
    if grid.cell(i, j-1) == '.' && grid.cell(i+1, j) == '.' { corners.push(Point::new(i+1, j-1)); }
    if grid.cell(i, j+1) == '.' && grid.cell(i+1, j) == '.' { corners.push(Point::new(i+1, j+1)); }
    corners
}

fn rotate(grid: &Grid, degrees: i32) -> Grid {
    let degrees = degrees % 360;
    let (height, width) = (grid.height(), grid.width());
    let mut new_cells = match degrees {
        0 | 180 | 360 => vec![vec!['@'; width as usize]; height as usize],
        90 | 270 => vec![vec!['@'; height as usize]; width as usize],
        _ => panic!("Invalid degrees: {}", degrees),
    };

    for i in 0..height {
        for j in 0..width {
            match degrees {
                0 => new_cells[i as usize][j as usize] = grid.cell(i, j),
                90 => new_cells[j as usize][(height - i - 1) as usize] = grid.cell(i, j),
                180 => new_cells[(height - i - 1) as usize][(width - j - 1) as usize] = grid.cell(i, j),
                270 => new_cells[(width - j - 1) as usize][i as usize] = grid.cell(i, j),
                _ => {},
            }
        }
    }
    Grid::new(new_cells)
}

fn detect_marks(grid: &Grid) -> Vec<Point> {
    let mut marks = Vec::new();
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            if grid.cell(i, j) != '.' && grid.cell(i, j) != '-' {
                marks.push(Point::new(i, j));
            }
        }
    }
    marks
}

pub fn collides(grid: &Grid, piece: &Grid, gc: Point, pc: Point) -> bool {
    for i in 0..piece.height() {
        for j in 0..piece.width() {
            let cell = piece.cell(i, j);
            let ij = Point::new(i, j);
            if cell != '.' && cell != '-' {
                let grid_point = gc + ij - pc;
                for &offset in &[
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(0, 1),
                    Point::new(1, 0),
                    Point::new(0, -1)
                    ] {
                    let offset_point = grid_point + offset;
                    if grid.cell(offset_point.row, offset_point.col) != '.' && grid.cell(offset_point.row, offset_point.col) != '-' {
                        return true;
                    }
                }
                // TODO: bounds check, make grid.cell return Option
                assert_ne!(grid.cell(grid_point.row, grid_point.col), '-');
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_print() {
        let grid = Grid::parse(
            ".....\n\
             .x...\n\
             .x...\n\
             .....\n\
             .....",
        );
        println!("{:?}", grid);
    }

    #[test]
    fn test_collides_true() {
        let grid = Grid::parse(
            ".....\n\
             .x...\n\
             .x...\n\
             .....\n\
             .....",
        );
        let piece = Grid::parse(
            "x\n\
             x",
        );
        assert!(collides(&grid, &piece, Point::new(3, 2), Point::new(1, 0)));
    }
    
    #[test]
    fn test_collides_false() {
        let grid = Grid::parse(
            ".....\n\
             .x...\n\
             .x...\n\
             .....\n\
             .....",
        );
        let piece = Grid::parse(
            "x\n\
             x",
        );
        assert!(!collides(&grid, &piece, Point::new(3, 3), Point::new(0, 0)));
    }
    
    #[test]
    fn test_detect_marks() {
        let piece = Grid::parse(
            "x\n\
             x\n\
             x",
        );
        assert_eq!(detect_marks(&piece), vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)]);
    }

    #[test]
    fn test_rotate() {
        let grid = Grid::parse(
            ".x\n\
             xx",
        );
        let rotated = rotate(&grid, 90);
        let expected = Grid::parse(
            "x.\n\
             xx",
        );
        assert_eq!(rotated, expected);
    }

    // More tests for rotate, get_moves, etc.
}
