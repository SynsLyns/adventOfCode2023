use std::{fs, env};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut points = vec![];
    let mut current = Point {x: 0, y: 0};
    let mut perimeter = 0;
    points.push(current);

    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let dir = iter.next().unwrap();
        let n: i64 = iter.next().unwrap().parse().unwrap();
        perimeter += n;
        current = match dir {
            "R" => Point { x: current.x + n, y: current.y },
            "L" => Point { x: current.x - n, y: current.y },
            "U" => Point { x: current.x, y: current.y + n},
            "D" => Point { x: current.x, y: current.y - n},
            _ => {
                println!("ERROR");
                Point {x: 0, y: 0}
            }
        };
        points.push(current)
    }

    let mut area = 0;
    for i in 0..points.len() {
        let y_sum = points[i].y + points[(i+1) % points.len()].y;
        let x_diff = points[i].x - points[(i+1) % points.len()].x;
        area += y_sum * x_diff;
    }
    let inner_area = area.abs() / 2 - perimeter / 2 + 1;
    let part1 = inner_area + perimeter;
    
    println!("PART 1: {part1}");

    let mut points = vec![];
    let mut current = Point {x: 0, y: 0};
    let mut perimeter = 0;
    points.push(current);

    for line in contents.lines() {
        let hex = line.split_whitespace().nth(2).unwrap().strip_prefix("(#").unwrap().strip_suffix(")").unwrap().to_string();
        let n = i64::from_str_radix(&hex[0..5], 16).unwrap();
        perimeter += n;
        current = match &hex[5..6] {
            "0" => Point { x: current.x + n, y: current.y },
            "2" => Point { x: current.x - n, y: current.y },
            "3" => Point { x: current.x, y: current.y + n},
            "1" => Point { x: current.x, y: current.y - n},
            _ => {
                println!("ERROR");
                Point {x: 0, y: 0}
            }
        };
        points.push(current)
    }

    let mut area = 0;
    for i in 0..points.len() {
        let y_sum = points[i].y + points[(i+1) % points.len()].y;
        let x_diff = points[i].x - points[(i+1) % points.len()].x;
        area += y_sum * x_diff;
    }
    let inner_area = area.abs() / 2 - perimeter / 2 + 1;
    let part2 = inner_area + perimeter;
    
    println!("PART 2: {part2}");
}
