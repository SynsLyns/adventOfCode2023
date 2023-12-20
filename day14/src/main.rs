use std::{env, fs, collections::HashMap};

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Grid {
        Grid {
            grid: vec![vec![' '; rows]; cols],
            rows,
            cols
        }
    }

    fn push(&mut self, char: char, row: usize, col: usize) {
        self.grid[col][row] = char;
    }

    fn roll(&mut self) {
        for col in &mut self.grid {
            let mut swap_index = 0;
            for i in 0..col.len() {
                match col[i] {
                    'O' => {
                        if swap_index != i {
                            col[swap_index] = 'O';
                            col[i] = '.';
                        }
                        swap_index += 1;
                    }
                    '#' => swap_index = i + 1,
                    _ => ()
                }
            }
        }
    }
    
    fn rotate_right(&mut self) {
        let mut new_grid = vec![vec![' '; self.cols]; self.rows];
        for col in 0..self.cols {
            for row in 0..self.rows {
                new_grid[self.rows - row - 1][col] = self.grid[col][row];
            }
        }
        self.grid = new_grid;
        self.rows = self.cols;
        self.cols = self.rows;
    }

    fn get_load(&self) -> usize {
        let mut load = 0;
        for col in &self.grid {
            for i in 0..self.rows {
                match col[i] {
                    'O' => load += self.rows - i,
                    _ => ()
                }
            }
        }
        load
    }

    fn to_string(&self) -> String {
        let mut map_as_string = String::new();
        for col in &self.grid {
            let append: String = col.iter().collect();
            map_as_string.push_str(&append);
        }
        map_as_string
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; cols];
    let mut grid = Grid::new(rows, cols);
    let mut part1 = 0;


    let mut row = 0;
    for line in contents.lines() {
        let mut col = 0;
        for char in line.chars() {
            grid.push(char, row, col);
            stacks[col].push(char);
            col += 1;
        }
        row += 1;
    }

    for stack in stacks {
        let mut currrent_height = 0;
        for (index, char) in stack.iter().enumerate() {
            match char {
                'O' => {
                    part1 += rows - currrent_height;
                    currrent_height += 1;
                }
                '#' => {
                    currrent_height = index + 1;
                }
                _ => ()
            }
        }
    }

    let mut part2 = 0;
    let cycles = 1000000000;

    let mut cache: HashMap<String, usize> = HashMap::new();
    for i in 0..cycles {
        grid.roll();
        for _ in 0..3 {
            grid.rotate_right();
            grid.roll();
        }
        grid.rotate_right();
        match cache.insert(grid.to_string(), i) {
            Some(x) => {
                let offset = (cycles - x) % (i - x);
                if offset == 1 {
                    part2 = grid.get_load();
                    break;
                }
            }
            None => ()
        }
    }

    println!("PART 1: {}", part1);
    println!("PART 2: {}", part2);
}
