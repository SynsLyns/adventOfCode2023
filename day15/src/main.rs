use std::{env, fs};

#[derive(Debug)]
struct Item {
    label: String,
    focal_length: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2 = 0;

    let mut boxes: [Vec<Item>; 256] = std::array::from_fn(|_| vec![]);

    for str in contents.trim().split(",") {
        part1 += hash_on(str);

        let end_index = str.find(|c| c == '-' || c == '=').unwrap();
        let label = &str[..end_index];

        let b = hash_on(label);

        if str.contains("=") {
           let focal_length: usize = str[end_index+1..].parse().unwrap();
           match boxes[b].iter_mut().find(|x| x.label == label) {
                Some(i) => {
                    i.focal_length = focal_length;
                }
                None => {
                    boxes[b].push(
                        Item {
                            label: label.to_string(),
                            focal_length
                        }
                    )
                }
           }
        }
        else {
            match boxes[b].iter().position(|x| x.label == label) {
                Some(i) => {
                    boxes[b].remove(i);
                }
                None => ()
           }
        }
    }

    for (i, b) in boxes.iter().enumerate() {
        for (j, item) in b.iter().enumerate() {
            part2 += (i+1) * (j+1) * item.focal_length;
        }
    }


    println!("PART 1: {part1} PART 2: {part2}");
}


fn hash_on(input: &str) -> usize {
    let mut current_value = 0;
    for char in input.chars() {
        current_value += char as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}