use std::{fs, env, time::Instant, collections::HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let now = Instant::now();

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();

    let mut rocks = HashSet::new();
    let mut start = (0, 0);

    for (row, line) in contents.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    rocks.insert((row, col));
                }
                'S' => {
                    start = (row, col);
                }
                _ => ()
            }
        }
    }

    let steps = 64;
    let mut reachable_plots = HashSet::new();
    reachable_plots.insert(start);

    for _ in 0..steps {
        let mut new_plots = HashSet::new();
        for pos in reachable_plots {
            for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let new_pos = (pos.0 as isize + offset.0, pos.1 as isize + offset.1);
                if 0 <= new_pos.0 && new_pos.0 < rows as isize && 0 <= new_pos.1 && new_pos.1 < cols as isize {
                    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                    if !rocks.contains(&new_pos) {
                        new_plots.insert(new_pos);
                    }
                }
            }
        }
        reachable_plots = new_plots;
    }
    println!("PART 1: {}", reachable_plots.len());

    // 26501365 = 202300 * 131 + 65
    let y0 = find_reachable(&rocks, start, 65, rows as usize, cols as usize);
    let y1 = find_reachable(&rocks, start, 65 + 131, rows as usize, cols as usize);
    let y2 = find_reachable(&rocks, start, 65 + 262, rows as usize, cols as usize);
    let a0 = y0;
    let a1 = y1 - y0;
    let a2 = (y2 - 2*y1 + y0)/2;

    let part2 = poly2(202300, a2, a1-a2, a0);

    println!("PART 2: {}", part2);
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}

fn find_reachable(rocks: &HashSet<(usize, usize)>, start: (usize, usize), steps: usize, rows: usize, cols: usize) -> usize {
    let rows = rows as isize;
    let cols = cols as isize;
    let mut reachable_plots = HashSet::new();
    reachable_plots.insert((start.0 as isize, start.1 as isize));
    

    for _ in 0..steps {
        let mut new_plots = HashSet::new();
        for pos in reachable_plots {
            for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                let hash_pos = ((((new_pos.0 % rows) + rows) % rows) as usize, (((new_pos.1 % cols) + cols) % cols) as usize);
                if !rocks.contains(&hash_pos) {
                    new_plots.insert(new_pos);
                }
            }
        }
        reachable_plots = new_plots;
    }
    reachable_plots.len()
}

fn poly2(x: usize, a: usize, b: usize, c: usize) -> usize {
    a * x.pow(2) + b * x + c
}