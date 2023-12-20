use std::fs;
use std::env;

const OFFSETS: [(i32, i32); 8] = [(1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)];

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut grid: Vec<char> = vec![];
    let mut grid2: Vec<Vec<i32>> = vec![];
    let rows = 140; // input is 140
    let columns = 140; // input is 140
    let mut gear_positions: Vec<usize> = vec![];

    for (row, line) in contents.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            grid.push(char);
            grid2.push(vec![]);
            if char == '*' {
                gear_positions.push(row*columns + column);
            }
        }
    }

    let mut row = 0;
    let mut column = 0;
    let mut sum = 0;
    while row < rows {
        while column < columns {
            let symbol = grid[row*columns + column];
            if symbol.is_digit(10) {
                let mut number = String::from("");
                let mut is_part = false;
                let mut gears: Vec<usize> = vec![];
                while column < columns && grid[row*columns + column].is_digit(10) {
                    number.push(grid[row*columns + column]);
                    for (row_offset, column_offset) in OFFSETS {
                        let row = (row as i32 + row_offset).clamp(0, (rows - 1) as i32) as usize;
                        let column = (column as i32 + column_offset).clamp(0, (columns - 1) as i32) as usize;
                        if !grid[row*columns + column].is_digit(10) && grid[row*columns + column] != '.' {
                            is_part = true;
                        }
                        if grid[row*columns + column]== '*' && !gears.contains(&(row*columns + column))  {
                            gears.push(row*columns + column);
                        }
                    }
                    column += 1;
                }

                for index in gears {
                    grid2[index].push(number.parse().unwrap());
                }
                if is_part {
                    sum += number.parse::<i32>().unwrap();
                }
            }
            column += 1;
        }
        column = 0;
        row += 1;
    }

    let mut sum2 = 0;
    for gear_position in gear_positions {
        let adjacent = &grid2[gear_position];
        if adjacent.len() == 2 {
            sum2 += adjacent[0]*adjacent[1];
        }
    }

    println!("{sum}, {sum2}");
}