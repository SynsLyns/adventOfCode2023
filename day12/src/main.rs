use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut arragement_sum: usize = 0;
    let mut arragement_sum2: usize = 0;
    for line in contents.lines() {
        arragement_sum += solve(line, 1);
        arragement_sum2 += solve(line, 5);
    }

    println!("PART 1: {arragement_sum}");
    println!("PART 2: {arragement_sum2}");
}

fn solve(line: &str, repeat: usize) -> usize {
    let mut data = line.split_whitespace();
    let mut springs: Vec<char> = data.next().unwrap().chars().collect();
    let mut broken_springs: Vec<usize> = data.next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    if repeat > 1 {
        springs.push('?');
        springs = springs.repeat(repeat);
        springs.remove(springs.len() - 1);
        broken_springs = broken_springs.repeat(5);
    }

    springs.push('.');

    let broken_spring_sum: usize = broken_springs.iter().sum();
    let leeway = springs.len() - (broken_spring_sum + broken_springs.len() - 1) + 1 - 1;

    let mut broken_so_far = 0;
    let mut allowable_broken: Vec<usize> = vec![0; springs.len() + 1]; //when indexing start from 1
    for (i, spring) in springs.iter().enumerate() {
        if *spring == '#' || *spring == '?' {
            broken_so_far += 1;
        }
        allowable_broken[i + 1] = broken_so_far;
    }

    let mut solution_matrix: Vec<usize> = vec![0; springs.len() * broken_springs.len()];

    let size = broken_springs[0];
    let mut possible = true;
    let mut sum = 0;
    for i in 0..leeway {
        if springs[i + size] == '#' {
            sum = 0;
        }
        else if possible && allowable_broken[i + size] - allowable_broken[i] == size {
            sum += 1;
        }

        solution_matrix[i+size] = sum;

        if springs[i] == '#' {
            possible = false;
        }
    }

    let mut index = size + 1;

    for i in 1..broken_springs.len() {
        let size = broken_springs[i];
        let mut sum = 0;
        for j in 0..leeway {
            if springs[(index + j) + size] == '#' {
                sum = 0;
            }
            else if allowable_broken[(index + j) + size] - allowable_broken[index + j] == size {
                sum += solution_matrix[(i-1)*springs.len() + (index + j - 1)];
            }

            solution_matrix[i*springs.len() + (index + j + size)] += sum;
        }
        index += size + 1;
    }

    *solution_matrix.last().unwrap()
}