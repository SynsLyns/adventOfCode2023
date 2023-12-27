use std::{fs, env, collections::HashMap};
use std::ops::Range;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut lines = contents.lines();
    let mut map = HashMap::new();
    let now = Instant::now();

    while let Some(str) = lines.next() {
        if str == "" {
            break
        }
        let mut data = str.split("{");
        let name = data.next().unwrap();
        let workflow: Vec<&str> = data.next().unwrap().strip_suffix("}").unwrap().split(",").collect();
        map.insert(name, workflow);
    }

    let mut part1 = 0;
    while let Some(str) = lines.next() {
        let mut data = str.strip_prefix("{").unwrap()
                                    .strip_suffix("}").unwrap()
                                    .split(",");
        let x: usize = data.next().unwrap().split("=").nth(1).unwrap().parse().unwrap();
        let m: usize = data.next().unwrap().split("=").nth(1).unwrap().parse().unwrap();
        let a: usize = data.next().unwrap().split("=").nth(1).unwrap().parse().unwrap();
        let s: usize = data.next().unwrap().split("=").nth(1).unwrap().parse().unwrap();
        let accepted = do_workflow(x, m, a, s, &map);
        if accepted {
            part1 += x + m + a + s;
        }
    }

    let part2 = solve_part2(&map);
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("PART 1: {part1} PART 2: {part2}");
}

fn do_workflow(x: usize, m: usize, a: usize, s: usize, workflows: &HashMap<&str, Vec<&str>>) -> bool {
    let mut current_workflow_key = "in";
    'outer: loop {
        let workflow = &workflows[current_workflow_key];
        for work in workflow {
            if !work.contains(":") {
                match *work {
                    "A" => {
                        return true
                    }
                    "R" => {
                        return false
                    },
                    x => {
                        current_workflow_key = x;
                        continue 'outer
                    }
                }
            }
            let mut data = work.split(":");
            let work = data.next().unwrap().to_string();
            let var_to_compare = match &work[0..1] {
                "x" => x,
                "m" => m,
                "a" => a,
                "s" => s,
                _ => {
                    println!("ERROR");
                    0
                }
            };
            let pass = match &work[1..2] {
                "<" => var_to_compare < work[2..].parse::<usize>().unwrap(),
                ">" => var_to_compare > work[2..].parse::<usize>().unwrap(),
                _ => {
                    println!("ERROR");
                    false
                }
            };
            if pass {
                match data.next().unwrap() {
                    "A" => {
                        return true
                    }
                    "R" => {
                        return false
                    },
                    x => {
                        current_workflow_key = x;
                        continue 'outer
                    }
                }
            }
        }
    }
}

fn solve_part2(workflows: &HashMap<&str, Vec<&str>>) -> usize {
    let mut total = 0;
    let mut processes = vec![];
    processes.push((1..4001, 1..4001, 1..4001, 1..4001, "in"));

    'outer: loop {
        let mut process = match processes.pop() {
            None => return total,
            Some(x) => x
        };
        let workflow = &workflows[process.4];
        for work in workflow {
            if !work.contains(":") {
                match *work {
                    "A" => {
                        total += process.clone().0.len() * process.clone().1.len() * process.clone().2.len() * process.clone().3.len();
                    }
                    "R" => (),
                    x => {
                        processes.push((process.clone().0, process.clone().1, process.clone().2, process.clone().3, x));
                    }
                }
                continue 'outer
            }
            let mut data = work.split(":");
            let work = data.next().unwrap().to_string();
            let (left, right) = match &work[0..1] {
                "x" => split_at(process.clone().0, work[2..].parse::<usize>().unwrap(), &work[1..2]),
                "m" => split_at(process.clone().1, work[2..].parse::<usize>().unwrap(), &work[1..2]),
                "a" => split_at(process.clone().2, work[2..].parse::<usize>().unwrap(), &work[1..2]),
                "s" => split_at(process.clone().3, work[2..].parse::<usize>().unwrap(), &work[1..2]),
                _ => {
                    println!("ERROR");
                    (None, None)
                }
            };
            match right {
                Some(x) => {
                    match data.next().unwrap() {
                        "A" => {
                            total += match &work[0..1] {
                                "x" => x.len() * process.clone().1.len() * process.clone().2.len() * process.clone().3.len() ,
                                "m" => process.clone().0.len()  * x.len() * process.clone().2.len() * process.clone().3.len(),
                                "a" => process.clone().0.len() * process.clone().1.len() * x.len() * process.clone().3.len(),
                                "s" => process.clone().0.len() * process.clone().1.len() * process.clone().2.len() * x.len(),
                                _ => {
                                    println!("ERROR");
                                    0
                                }
                            };
                        }
                        "R" => (),
                        y => {
                            processes.push(match &work[0..1] {
                                    "x" => (x, process.clone().1, process.clone().2, process.clone().3, y),
                                    "m" => (process.clone().0, x, process.clone().2, process.clone().3, y),
                                    "a" => (process.clone().0, process.clone().1, x, process.clone().3, y),
                                    "s" => (process.clone().0, process.clone().1, process.clone().2, x, y),
                                    _ => {
                                        println!("ERROR");
                                        process.clone()
                                    }
                                }
                            );
                        }
                    }
                }
                None => ()
            };
            match left {
                Some(x) => {
                    process = match &work[0..1] {
                        "x" => (x, process.clone().1, process.clone().2, process.clone().3, process.clone().4),
                        "m" => (process.clone().0, x, process.clone().2, process.clone().3, process.clone().4),
                        "a" => (process.clone().0, process.clone().1, x, process.clone().3, process.clone().4),
                        "s" => (process.clone().0, process.clone().1, process.clone().2, x, process.clone().4),
                        _ => {
                            println!("ERROR");
                            process.clone()
                        }
                    };
                }
                None => {
                    continue 'outer
                }
            };
        }
    }
}

// first is used for rest of current workflow, second is used for range to jump to another workflow
fn split_at(range: Range<usize>, i: usize, t: &str) -> (Option<Range<usize>>, Option<Range<usize>>) {
    if range.end - 1 < i {
        match t {
            "<" => {
                return (None, Some(range))
            }
            ">" => {
                return (Some(range), None)
            }
            _ => {
                return (None, None)
            }
        }
    }
    else if range.start > i {
        match t {
            "<" => {
                return (Some(range), None)
            }
            ">" => {
                return (None, Some(range))
            }
            _ => {
                return (None, None)
            }
        }
    }
    else {
        match t {
            "<" => {
                return (Some(i..range.end), Some(range.start..i))
            }
            ">" => {
                return (Some(range.start..i+1), Some(i+1..range.end))
            }
            _ => {
                return (None, None)
            }
        }
        
    }
}