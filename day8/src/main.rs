use std::collections::HashMap;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let mut lines = contents.lines();
    let instructions = lines.next().unwrap();
    lines.next();
    populate_map(&mut map, lines.collect());

    let mut current_string = String::from("AAA");
    let mut steps = 0;
    'outer: loop {
        for i in instructions.chars() {
            match i {
                'L' => {
                    current_string = map.get(&current_string).cloned().unwrap().0;
                }
                'R' => {
                    current_string = map.get(&current_string).cloned().unwrap().1;
                }
                _ => ()
            }
            steps += 1;
            if current_string == "ZZZ" {
                break 'outer;
            }
        }
    }

    let mut current_strings: Vec<String> = map.keys().cloned().filter(|x| x.ends_with("A")).collect();
    println!("{:?}", current_strings);
    let mut steps2 = 0;
    'outer: loop {
        'inner : for i in instructions.chars() {
            match i {
                'L' => {
                    current_strings.iter_mut().for_each(|x| *x = map.get(x).cloned().unwrap().0);
                }
                'R' => {
                    current_strings.iter_mut().for_each(|x| *x = map.get(x).cloned().unwrap().1);
                }
                _ => ()
            }
            steps2 += 1;
            for s in &current_strings {
                if !s.ends_with("Z") {
                    continue 'inner;
                }
            }
            break 'outer;
        }
    }
    /// PART 2 skipped, ended up finding cycles and then LCM.

    println!("Part 1: {} Part 2: {}", steps, steps2);
}

fn populate_map(map: &mut HashMap<String, (String, String)>, data: Vec<&str>) {
    for line in data {
        let key = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        map.insert(key, (left, right));
    }
}