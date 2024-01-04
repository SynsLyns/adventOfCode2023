use std::{fs, env, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let now = Instant::now();

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}
