use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;

    let re = Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    for line in contents.lines() {
        let mut buffer = ['?' ; 6];
        let mut buffer_index = 0;
        let mut first_number = 0;
        let mut last_number = 0;
        let mut is_first = true;
        for char in line.chars() {
            buffer[buffer_index] = char;
            let s: String = buffer[0..buffer_index+1].iter().collect();
            buffer_index = (buffer_index + 1) % 6;
            if buffer_index == 0 {
                buffer.rotate_left(1);
                buffer_index = 5;
            }
            let m = match re.find(&s) {
                None => continue,
                Some(i) => i.as_str()
            };

            buffer_index = 1;
            let last_char = m.chars().last().unwrap();
            buffer = ['?' ; 6];
            buffer[0] = last_char;

            let number = match m {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => {
                    buffer[0] = '?';
                    buffer_index = 0;
                    m.parse().unwrap()
                }
            };
            
            if is_first {
                first_number = number;
                is_first = false;
            }

            last_number = number;
        }
        sum += first_number*10 + last_number;
    }
    println!("{sum}");
}