use std::{fs, env};

struct Grid {
    grid: Vec<char>,
}

impl Grid {
    fn push(&mut self, char: char) {
        self.grid.push(char);
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

    let mut grid = Grid {
        grid: vec![],
    };

    let mut galaxies_per_row: Vec<usize> = vec![0; rows];
    let mut galaxies_per_col: Vec<usize> = vec![0; cols];
    let mut galaxy_positions: Vec<(usize, usize)> = vec![];

    let mut row = 0;
    let mut col = 0;
    for line in contents.lines() {
        for char in line.chars() {
            grid.push(char);
            if char == '#' {
                galaxies_per_row[row] += 1;
                galaxies_per_col[col] += 1;
                galaxy_positions.push((row, col));
            }
            col = (col + 1) % cols;
        }
        row += 1;
    }

    let mut add_to_row: Vec<usize> = vec![0; rows];
    let mut add_to_col: Vec<usize> = vec![0; cols];
    let mut rows_to_add = 0;
    let mut cols_to_add = 0;
    for i in 0..rows {
        add_to_row[i] = rows_to_add;
        if galaxies_per_row[i] == 0 {
            rows_to_add += 1;
        }
    }
    for i in 0..cols {
        add_to_col[i] = cols_to_add;
        if galaxies_per_col[i] == 0 {
            cols_to_add += 1;
        }
    }
    
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..galaxy_positions.len() {
        for j in i + 1..galaxy_positions.len() {
            let g1 = galaxy_positions[i];
            let g2 = galaxy_positions[j];
            let distance = (g2.0 + add_to_row[g2.0]).abs_diff(g1.0 + add_to_row[g1.0]) + (g2.1 + add_to_col[g2.1]).abs_diff(g1.1 + add_to_col[g1.1]);
            let distance2 = (g2.0 + add_to_row[g2.0]*999_999).abs_diff(g1.0 + add_to_row[g1.0]*999_999) + (g2.1 + add_to_col[g2.1]*999_999).abs_diff(g1.1 + add_to_col[g1.1]*999_999);
            part1 += distance;
            part2 += distance2;
        }
    }
    println!("PART 1: {:?} PART 2: {:?}", part1, part2);

}
