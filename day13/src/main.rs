use std::{env, fs};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = contents.lines();


    // RESET BELOW EVERY TIME "" IS FOUND
    let mut rows: [String; 20] = Default::default();
    let mut cols: [String; 20] = Default::default();
    let mut row = 0;
    // RESET ABOVE EVERY TIME "" IS FOUND
    
    let mut part1 = 0;
    let mut part2 = 0;

    loop {
        match lines.next() {
            Some(line) => {
                if line == "" {
                    let points = get_points(&rows, &cols);
                    part1 += points.0;
                    part2 += points.1;
                    rows = Default::default();
                    cols = Default::default();
                    row = 0;
                    continue;
                }

                rows[row] = line.to_string();
                let mut col = 0;
                for char in line.chars() {
                    cols[col].push(char);
                    col += 1;
                }
                row += 1;
            }
            None => {
                let points = get_points(&rows, &cols);
                part1 += points.0;
                part2 += points.1;
                break;
            }
        };
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("PART 1: {} PART 2: {}", part1, part2);
}

fn get_points(rows: &[String; 20], cols: &[String; 20]) -> (usize, usize) {
    let total_rows = cols[0].len();
    let total_cols = rows[0].len();
    let mut part1 = 0;
    let mut part2 = 0;

    // Horizontal mirror
    match find_mirror(rows, total_rows) {
        Some(x) => part1 = x*100,
        None => ()
    }
    
    if part1 == 0 {
        // Vertical mirror
        match find_mirror(cols, total_cols) {
            Some(x) => part1 = x,
            None => ()
        }
    }   

    // Horizontal check part 2
    match find_mirror_with_smudge(rows, total_rows) {
        Some(x) => {
            part2 = x*100;
            return (part1, part2);
        }
        None => ()
    }

    // Vertical check part 2
    match find_mirror_with_smudge(cols, total_cols) {
        Some(x) => part2 = x,
        None => ()
    }

    (part1, part2)
}

fn find_mirror(data: &[String; 20], len: usize) -> Option<usize> {
    for i in 1..len {
        let mut before: Vec<&str> = vec![];
        let mut after: Vec<&str> = vec![];
        let iter_to = match i > len / 2 {
            true => len-i,
            false => i
        };
        for j in 0..iter_to {
            before.push(&data[i-j-1]);
            after.push(&data[i+j]);
        }
        if before == after {
            return Some(i);
        }
    }
    None
}

fn find_mirror_with_smudge(data: &[String; 20], len: usize) -> Option<usize> {
    for i in 1..len {
        let mut before: Vec<char> = vec![];
        let mut after: Vec<char> = vec![];
        let iter_to = match i > len / 2 {
            true => len-i,
            false => i
        };
        for j in 0..iter_to {
            before.append(&mut data[i-j-1].chars().collect());
            after.append(&mut data[i+j].chars().collect());
        }
        if has_one_smudge(&before, &after) {
            return Some(i);
        }
    }
    None
}

fn has_one_smudge(before: &Vec<char>, after: &Vec<char>) -> bool {
    let mut smudges = 0;
    for (i, j) in before.iter().zip(after.iter()) {
        if i != j {
            smudges += 1;
            if smudges > 1 {
                return false;
            }
        }
    }
    smudges == 1
}