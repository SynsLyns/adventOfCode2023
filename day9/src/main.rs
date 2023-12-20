use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in contents.lines() {
        let mut data: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        part1 += predict_next_number(&data);
        data.reverse();
        part2 += predict_next_number(&data);
    }
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn predict_next_number(data: &Vec<i64>) -> i64 {
    if data.iter().all(|x| *x == 0) {
        return 0
    }
    let mut diffs: Vec<i64> = vec![];
    for i in 1..data.len() {
        diffs.push(data[i] - data[i-1]);
    }
    data.last().unwrap() + predict_next_number(&diffs)
}