use std::{env, fs, collections::HashSet};
use std::time::Instant;

struct Tile {
    char: char,
    visited: bool,
    visited_dirs: HashSet<(i32, i32)>
}

struct Grid<T> {
    grid: Vec<T>,
    rows: usize,
    cols: usize
}

impl Tile {
    fn new(char: char) -> Tile {
        Tile {
            char,
            visited: false,
            visited_dirs: HashSet::new()
        }
    }
}

impl<T> Grid<T> {
    fn new(rows: usize, cols: usize) -> Grid<T>  {
        Grid {
            grid: vec![],
            rows,
            cols
        }
    }

    fn get(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.grid[row * self.cols + col]
    }

    fn push(&mut self, item: T) {
        self.grid.push(item);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();

    let mut grid = Grid::<Tile>::new(rows, cols);

    for line in contents.lines() {
        for char in line.chars() {
            grid.push(Tile::new(char));
        }
    }

    let now = Instant::now();
    beam(&mut grid, 0, 0, (0, 1));

    let part1 = get_energized_and_reset(&mut grid);

    let mut part2 = 0;
    for i in 0..rows {
        beam(&mut grid, i, 0, (0, 1));
        let energized = get_energized_and_reset(&mut grid);
        if part2 < energized {
            part2 = energized;
        }
        beam(&mut grid, i, cols - 1, (0, -1));
        let energized = get_energized_and_reset(&mut grid);
        if part2 < energized {
            part2 = energized;
        }
    }
    for i in 0..cols {
        beam(&mut grid, 0, i, (1, 0));
        let energized = get_energized_and_reset(&mut grid);
        if part2 < energized {
            part2 = energized;
        }
        beam(&mut grid, rows - 1, i, (-1, 0));
        let energized = get_energized_and_reset(&mut grid);
        if part2 < energized {
            part2 = energized;
        }
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("PART 1: {part1} PART 2: {part2}");
}

fn beam(grid: &mut Grid<Tile>, row: usize, col: usize, dir: (i32, i32)) {
    let tile = grid.get(row, col);
    if tile.visited == true && tile.visited_dirs.contains(&dir) {
        return
    }
    tile.visited = true;
    tile.visited_dirs.insert(dir);

    let mut new_dir = dir;
    match tile.char {
        '/' => {
            match dir {
                (0, 1) => new_dir = (-1, 0),
                (1, 0) => new_dir = (0, -1),
                (0, -1) => new_dir = (1, 0),
                (-1, 0) => new_dir = (0, 1),
                _ => ()
            }
        }
        '\\' => {
            match dir {
                (0, 1) => new_dir = (1, 0),
                (1, 0) => new_dir = (0, 1),
                (0, -1) => new_dir = (-1, 0),
                (-1, 0) =>  new_dir = (0, -1),
                _ => ()
            }
        },
        '|' => {
            match dir {
                (0, 1) | (0, -1) => {
                    let split_dir = (-1, 0);
                    let split_row = row as i32 + split_dir.0;
                    let split_col = col as i32 + split_dir.1;
                    if 0 <= split_row && split_row < grid.rows as i32 && 0 <= split_col && split_col < grid.cols as i32 {
                        beam(grid, split_row as usize, split_col as usize, split_dir);
                    }
                    new_dir = (1, 0);
                }
                _ => ()
            }
        }
        '-' => {
            match dir {
                (1, 0) | (-1, 0) => {
                    let split_dir = (0, -1);
                    let split_row = row as i32 + split_dir.0;
                    let split_col = col as i32 + split_dir.1;
                    if 0 <= split_row && split_row < grid.rows as i32 && 0 <= split_col && split_col < grid.cols as i32 {
                        beam(grid, split_row as usize, split_col as usize, split_dir);
                    }
                    new_dir = (0, 1);
                }
                _ => ()
            }
        }
        _ => ()
    }
    let new_row = row as i32 + new_dir.0;
    let new_col = col as i32 + new_dir.1;
    if 0 <= new_row && new_row < grid.rows as i32 && 0 <= new_col && new_col < grid.cols as i32{
        beam(grid, new_row as usize, new_col as usize, new_dir);
    }
}

fn get_energized_and_reset(grid: &mut Grid<Tile>) -> usize {
    let mut energized = 0;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            let tile = grid.get(i, j);
            if tile.visited == true {
                energized += 1;
                tile.visited = false;
            }
            tile.visited_dirs.clear();
        }
    }
    energized
}