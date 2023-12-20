use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = contents.lines();
    let times: Vec<&str> = lines.next().unwrap().split(":").nth(1).unwrap().trim().split_ascii_whitespace().collect();
    let record_distances: Vec<&str> = lines.next().unwrap().split(":").nth(1).unwrap().trim().split_ascii_whitespace().collect();

    let mut part1: f64 = 1.0;
    for (time, record_distance) in times.iter().zip(record_distances.iter()) {
        let t: f64 = time.parse().unwrap();
        let d: f64 = record_distance.parse().unwrap();
        let (min, max) = get_quadratic_roots(1.0, -t, d);
        part1 *= (max - 1.0).ceil() - (min + 1.0).floor() + 1.0;
    }
    println!("PART 1: {part1}");

    let t: f64 = times.join("").parse().unwrap();
    let d: f64 = record_distances.join("").parse().unwrap();
    println!("{t} {d} {:?}", times);
    let (min, max) = get_quadratic_roots(1.0, -t, d);
    println!("PART 2: {}", (max - 1.0).ceil() - (min + 1.0).floor() + 1.0);
}


fn get_quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    ((-b-(b.powf(2.0) - 4.0*a*c).sqrt()) / 2.0*a, (-b+(b.powf(2.0) - 4.0*a*c).sqrt()) / 2.0*a)
}