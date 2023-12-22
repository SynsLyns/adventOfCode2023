use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut part1 = 0;

    for str in contents.trim().split(",") {
        let mut current_value = 0;
        for char in str.chars() {
            current_value += char as u32;
            current_value *= 17;
            current_value %= 256;
        }
        part1 += current_value;
    }

    println!("PART 1: {part1}");
}
