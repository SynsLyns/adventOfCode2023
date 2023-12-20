use std::collections::VecDeque;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut points_sum = 0;
    let mut total_scratchcards = 0;
    let mut next_five: VecDeque<i32> = VecDeque::from([1; 10]);

    for line in contents.lines() {
        let mut wins = 0;
        let mut data = line.split(":").nth(1).unwrap().split("|");
        let mut winning_numbers: Vec<i32> = data.next().unwrap().trim().split(" ").filter(|x| *x != "").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut our_numbers: Vec<i32> = data.next().unwrap().trim().split(" ").filter(|x| *x != "").map(|x| x.parse::<i32>().unwrap()).collect();
        winning_numbers.sort();
        our_numbers.sort();

        let winning_numbers_len = winning_numbers.len();
        let our_numbers_len = our_numbers.len();
        let mut winning_index = 0;
        let mut our_index = 0;

        let cards = next_five.pop_front().unwrap();
        total_scratchcards += cards;
        next_five.push_back(1);
        
        while winning_index < winning_numbers_len && our_index < our_numbers_len {
            if winning_numbers[winning_index] == our_numbers[our_index] {
                wins += 1;
                winning_index += 1;
                our_index += 1;
            }
            else if winning_numbers[winning_index] < our_numbers[our_index] {
                winning_index += 1;
            }
            else {
                our_index += 1;
            }
        }
        for i in 0..(wins) {
            next_five[i] += cards;
        }
        if wins != 0 {
            points_sum += i32::pow(2, (wins - 1) as u32);
        }
    }
    println!("Part 1: {points_sum}, Part 2: {total_scratchcards}");
}
