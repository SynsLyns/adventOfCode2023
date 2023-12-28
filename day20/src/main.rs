use std::{env, fs, time::Instant, collections::{HashMap, VecDeque}};

#[derive(Debug)]
enum Module {
    FlipFlop(bool),
    Conjunction(usize, usize, HashMap<String, bool>),
    Broadcast
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let now = Instant::now();

    let mut modules = HashMap::new();
    let mut connections = vec![];

    for line in contents.lines() {
        let mut data = line.split("->");
        let name = data.next().unwrap().trim();
        let destinations: Vec<&str> = data.next().unwrap().trim().split(", ").collect();
        connections.push((&name[1..], destinations.clone()));
        match &name[0..1] {
            "%" => {
                modules.insert(&name[1..], (Module::FlipFlop(false), destinations));
            }
            "&" => {
                modules.insert(&name[1..], (Module::Conjunction(0, 0, HashMap::new()), destinations));
            }
            _ => {
                modules.insert(name, (Module::Broadcast, destinations));
            }
        }
    }

    for (from, to) in connections {
        for dest in to {
            match modules.get_mut(dest) {
                None => continue,
                Some(x) => {
                    match &mut x.0 {
                        Module::Conjunction(inputs, _, memory) => {
                            *inputs += 1;
                            memory.insert(from.to_string(), false);
                        }
                        _ => ()
                    }
                }
            };
        }
    }

   let mut pulses = VecDeque::new();
   let mut low_pulses = 0;
   let mut high_pulses = 0;
   let mut presses: usize = 0;
   let mut cycles = HashMap::new();
   'outer: loop {
        pulses.push_back(("button", "broadcaster", false));
        presses += 1;
        while let Some((from, to, is_high)) = pulses.pop_front() {
            match is_high {
                false => low_pulses += 1,
                true => high_pulses += 1
            }
            let send_pulse;
            if (to == "ks" || to == "kp" || to == "xc" || to == "ct") && !is_high {
                cycles.insert(to, presses);
                if cycles.len() == 4 {
                    break 'outer
                }
            }
            match modules.get_mut(to) {
                None => continue,
                Some(x) => {
                    match &mut x.0 {
                        Module::FlipFlop(is_on) => {
                            if is_high {
                                continue;
                            }
                            *is_on = !*is_on;
                            send_pulse = *is_on;
                        },
                        Module::Conjunction(inputs, highs, memory) => {
                            let last_pulse = memory[from];
                            *memory.get_mut(from).unwrap() = is_high;
                            if last_pulse != is_high {
                                match is_high {
                                    true => *highs += 1,
                                    false => *highs -= 1
                                }
                            }
                            send_pulse = !(inputs == highs);
                        },
                        Module::Broadcast => send_pulse = is_high,
                    }
                }
            };
            
            for dest in &modules[to].1 {
                pulses.push_back((to, dest, send_pulse));
            }
        }
        if presses == 1000 {
            println!("PART 1: {}", low_pulses * high_pulses);
        }
    }

    let mut part2 = 1;
    for item in cycles.values() {
        part2 = lcm(part2, *item);
    }
   
    let elapsed = now.elapsed();
    println!("PART 2: {}", part2);
    println!("{:.2?}", elapsed);
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}