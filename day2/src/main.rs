use std::env;
use std::fs;

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    let mut sum2 = 0;
    for line in contents.lines() {
        let game_number = get_first_number(line, 5);
        if is_possible(line) {
            sum += game_number;
        }
        sum2 += get_power(line);
    }
    println!("{sum}, {sum2}");
}

fn is_possible(line: &str) -> bool {
    let line: Vec<&str> = line.split(":").collect();
    let pulls = line[1].trim().split(";");
    let mut possible = true;
    for pull in pulls {
        let pull_contents = pull.split(",");
        for contents in pull_contents {
            let value: Vec<&str> = contents.trim().split(" ").collect();
            let n: i32 = value[0].parse().unwrap();
            match value[1] {
                "red" => {
                    if n > MAX_RED_CUBES {
                        possible = false;
                        break;
                    }
                },
                "green" => {
                    if n > MAX_GREEN_CUBES {
                        possible = false;
                        break;
                    }
                },
                "blue" => {
                    if n > MAX_BLUE_CUBES {
                        possible = false;
                        break;
                    }
                },
                _ => {}
            }
        }
    }
    possible
}

fn get_power(line: &str) -> i32 {
    let line: Vec<&str> = line.split(":").collect();
    let pulls = line[1].trim().split(";");
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;
    for pull in pulls {
        let pull_contents = pull.split(",");
        for contents in pull_contents {
            let value: Vec<&str> = contents.trim().split(" ").collect();
            let n: i32 = value[0].parse().unwrap();
            match value[1] {
                "red" => {
                    if n > min_red {
                        min_red = n;
                    }
                },
                "green" => {
                    if n > min_green {
                        min_green = n;
                    }
                },
                "blue" => {
                    if n > min_blue {
                        min_blue = n;
                    }
                },
                _ => {}
            }
        }
    }
    min_red * min_blue * min_green
}

fn get_first_number(line: &str, start_index: usize) -> i32 {
    let bytes = line.chars();
    for (i, item) in bytes.skip(start_index).enumerate() {
        if !item.is_digit(10) {
            return line[start_index..start_index+i].parse().unwrap();
        }
    }
    // should never return -1.
    return -1;
}