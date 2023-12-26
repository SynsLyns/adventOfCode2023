use std::{fs, env, collections::BinaryHeap, cmp::Ordering};
use std::time::Instant;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();

    let mut grid = vec![];
    let mut dist = vec![];
    for line in contents.lines() {
        for char in line.chars() {
            for _ in 0..4*4 {
                grid.push(char.to_digit(10).unwrap() as usize);
                dist.push(usize::MAX);
            }
        }
    }

    let now = Instant::now();
    
    let mut heap = BinaryHeap::new();
    heap.push(State {cost: 0, position: 0});
    dist[0] = 0;
    
    while let Some(State {cost, position}) = heap.pop() {
        if cost > dist[position] {
            continue;
        }
        
        let next_states = part1_next_states(cost, position, &mut grid, 4, 4, rows, cols);
        for state in &next_states {
            let next = State { cost: state.cost, position: state.position };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    let elapsed = now.elapsed();
    let part1 = dist.iter().rev().take(16).min().unwrap();
    println!("{:.2?}", elapsed);
    println!("PART 1 {part1}");

    let now = Instant::now();

    let mut grid = vec![];
    let mut dist = vec![];
    for line in contents.lines() {
        for char in line.chars() {
            for _ in 0..11*4 {
                grid.push(char.to_digit(10).unwrap() as usize);
                dist.push(usize::MAX);
            }
        }
    }
    
    let mut heap = BinaryHeap::new();
    heap.push(State {cost: 0, position: 10});
    dist[0] = 0;
    
    while let Some(State {cost, position}) = heap.pop() {
        if cost > dist[position] {
            continue;
        }
        
        let next_states = part2_next_states(cost, position, &mut grid, 11, 4, rows, cols);
        for state in &next_states {
            let next = State { cost: state.cost, position: state.position };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    let elapsed = now.elapsed();
    let last_tile = &dist[dist.len()-44..];
    let mut part2 =  usize::MAX;
    for (index, item) in last_tile.iter().enumerate() {
        if index % 11 >= 4 && part2 > *item {
            part2 = *item;
        }
    }
    println!("{:.2?}", elapsed);
    println!("PART 2 {part2:?}");
}

fn part1_next_states(cost: usize, position: usize, grid: &mut Vec<usize>, same_dir_moves: usize, directions: usize, rows: usize, cols: usize) -> Vec<State> {
    let mut next_states = vec![];
    let same_dir_move = position % same_dir_moves;
    let direction = (position / same_dir_moves) % directions;
    let col = (position / same_dir_moves / directions) % cols;
    let row = position / same_dir_moves / directions / cols;
    if same_dir_move != 3 {
        let mut valid = true;
        let displacement = match direction {
            0 => {
                if col + 1 >= cols { valid = false; }
                (same_dir_moves * directions) as i32 + 1
            } // RIGHT
            1 => {
                if row + 1 >= rows { valid = false; }
                (same_dir_moves * directions * cols) as i32 + 1
            } // DOWN
            2 => {
                if col as i32 - 1 < 0 { valid = false; }
                -((same_dir_moves * directions) as i32) + 1
            } // LEFT
            3 => {
                if row as i32 - 1 < 0 { valid = false; }
                -((same_dir_moves * directions * cols) as i32) + 1
            } // UP
            _ => {
                println!("ERROR");
                0
            }
        };

        let next_pos = position as i32 + displacement;
        if valid {
            let next_pos = next_pos as usize;
            next_states.push(State { cost: cost + grid[next_pos], position: next_pos })
        }
    }
    // TURN LEFT
    let mut valid = true;
    let displacement = match direction {
        0 => {
            if row as i32 - 1 < 0 { valid = false; }
            -((same_dir_moves * directions * cols) as i32) + (3 * same_dir_moves) as i32 - same_dir_move as i32 + 1
        } // RIGHT -> UP
        1 => {
            if col + 1 >= cols { valid = false; }
            (same_dir_moves * directions) as i32 - same_dir_moves as i32 - same_dir_move as i32 + 1
        } // DOWN -> RIGHT
        2 => {
            if row + 1 >= rows { valid = false; }
            (same_dir_moves * directions * cols) as i32 - same_dir_moves as i32 - same_dir_move as i32 + 1
        } // LEFT -> DOWN
        3 => {
            if col as i32 - 1 < 0 { valid = false; }
            -((same_dir_moves * directions) as i32) - same_dir_moves as i32 - same_dir_move as i32 + 1
        } // UP -> LEFT
        _ => {
            println!("ERROR");
            0
        }
    };
    let next_pos = position as i32 + displacement;
    if valid {
        let next_pos = next_pos as usize;
        next_states.push(State { cost: cost + grid[next_pos], position: next_pos })
    }
    // TURN RIGHT
    let mut valid = true;
    let displacement = match direction {
        0 => {
            if row + 1 >= rows { valid = false; }
            (same_dir_moves * directions * cols) as i32 + same_dir_moves as i32 - same_dir_move as i32 + 1
        } // RIGHT -> DOWN
        1 => {
            if col as i32 - 1 < 0 { valid = false; }
            -((same_dir_moves * directions) as i32) + same_dir_moves as i32 - same_dir_move as i32 + 1
        } // DOWN -> LEFT
        2 => {
            if row as i32 - 1 < 0 { valid = false; }
            -((same_dir_moves * directions * cols) as i32) + same_dir_moves as i32 - same_dir_move as i32 + 1
        } // LEFT -> UP
        3 => {
            if col + 1 >= cols { valid = false; }
            (same_dir_moves * directions) as i32 - (3 * same_dir_moves) as i32 - same_dir_move as i32 + 1
        } // UP -> RIGHT
        _ => {
            println!("ERROR");
            0
        }
    };
    let next_pos = position as i32 + displacement;
    if valid {
        let next_pos = next_pos as usize;
        next_states.push(State { cost: cost + grid[next_pos], position: next_pos })
    }
    next_states
}

fn part2_next_states(cost: usize, position: usize, grid: &mut Vec<usize>, same_dir_moves: usize, directions: usize, rows: usize, cols: usize) -> Vec<State> {
    let mut next_states = vec![];
    let same_dir_move = position % same_dir_moves;
    let direction = (position / same_dir_moves) % directions;
    let col = (position / same_dir_moves / directions) % cols;
    let row = position / same_dir_moves / directions / cols;
    if same_dir_move != 10 {
        let mut valid = true;
        let displacement = match direction {
            0 => {
                if col + 1 >= cols { valid = false; }
                (same_dir_moves * directions) as i32 + 1
            } // RIGHT
            1 => {
                if row + 1 >= rows { valid = false; }
                (same_dir_moves * directions * cols) as i32 + 1
            } // DOWN
            2 => {
                if col as i32 - 1 < 0 { valid = false; }
                -((same_dir_moves * directions) as i32) + 1
            } // LEFT
            3 => {
                if row as i32 - 1 < 0 { valid = false; }
                -((same_dir_moves * directions * cols) as i32) + 1
            } // UP
            _ => {
                println!("ERROR");
                0
            }
        };

        let next_pos = position as i32 + displacement;
        if valid {
            let next_pos = next_pos as usize;
            next_states.push(State { cost: cost + grid[next_pos], position: next_pos })
        }
    }
    // TURN LEFT
    if same_dir_move >= 4 {
        let mut valid = true;
        let displacement = match direction {
            0 => {
                if row as i32 - 1 < 0 { valid = false; }
                -((same_dir_moves * directions * cols) as i32) + (3 * same_dir_moves) as i32 - same_dir_move as i32 + 1
            } // RIGHT -> UP
            1 => {
                if col + 1 >= cols { valid = false; }
                (same_dir_moves * directions) as i32 - same_dir_moves as i32 - same_dir_move as i32 + 1
            } // DOWN -> RIGHT
            2 => {
                if row + 1 >= rows { valid = false; }
                (same_dir_moves * directions * cols) as i32 - same_dir_moves as i32 - same_dir_move as i32 + 1
            } // LEFT -> DOWN
            3 => {
                if col as i32 - 1 < 0 { valid = false; }
                -((same_dir_moves * directions) as i32) - same_dir_moves as i32 - same_dir_move as i32 + 1
            } // UP -> LEFT
            _ => {
                println!("ERROR");
                0
            }
        };
        let next_pos = position as i32 + displacement;
        if valid {
            let next_pos = next_pos as usize;
            next_states.push(State { cost: cost + grid[next_pos], position: next_pos })
        }
        // TURN RIGHT
        let mut valid = true;
        let displacement = match direction {
            0 => {
                if row + 1 >= rows { valid = false; }
                (same_dir_moves * directions * cols) as i32 + same_dir_moves as i32 - same_dir_move as i32 + 1
            } // RIGHT -> DOWN
            1 => {
                if col as i32 - 1 < 0 { valid = false; }
                -((same_dir_moves * directions) as i32) + same_dir_moves as i32 - same_dir_move as i32 + 1
            } // DOWN -> LEFT
            2 => {
                if row as i32 - 1 < 0 { valid = false; }
                -((same_dir_moves * directions * cols) as i32) + same_dir_moves as i32 - same_dir_move as i32 + 1
            } // LEFT -> UP
            3 => {
                if col + 1 >= cols { valid = false; }
                (same_dir_moves * directions) as i32 - (3 * same_dir_moves) as i32 - same_dir_move as i32 + 1
            } // UP -> RIGHT
            _ => {
                println!("ERROR");
                0
            }
        };
        let next_pos = position as i32 + displacement;
        if valid {
            let next_pos = next_pos as usize;
            next_states.push(State { cost: cost + grid[next_pos], position: next_pos })
        }
    }
    next_states
}