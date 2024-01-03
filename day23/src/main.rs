use std::{fs, env, time::Instant, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let now = Instant::now();
    
    let longest_path_1 = solve(&contents, false);
    let longest_path_2 = solve(&contents, true);
    
    println!("Part 1: {longest_path_1}");
    println!("Part 2: {longest_path_2}");
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}

fn solve(contents: &String, is_part2: bool) -> usize  {
    let cols = contents.lines().next().unwrap().len() as isize;
    let grid: Vec<char> = contents.chars().filter(|&x| x != '\n').collect();
    let mut graph = HashMap::<_,Vec<_>>::new();
    for i in 0..grid.len() {
        let all_offsets: &[isize] = &[1, -1, cols, -cols];
        let offsets = match grid[i] {
            '#' => continue,
            _ if is_part2 => all_offsets,
            '.' => all_offsets,
            '>' => &all_offsets[0..][..1],
            '<' => &all_offsets[1..][..1],
            'v' => &all_offsets[2..][..1],
            _ => unreachable!(),
        };
        let e = graph.entry(i).or_default();
        for &offset in offsets {
            let other = (i as isize + offset) as usize;
            if grid.get(other).is_some_and(|&x| x != '#') {
                e.push((other, 1));
            }
        }    
    }

    let corridors: Vec<usize> = graph.iter()
                                                            .filter(|(_, x)| x.len() == 2)
                                                            .map(|(&x, _)| x)
                                                            .collect();

    for i in corridors {
        let neighbors = graph.remove(&i).unwrap();
        let (i1, d1) = neighbors[0];
        let (i2, d2) = neighbors[1];
        let n1 = graph.get_mut(&i1).unwrap();
        if let Some(index) = n1.iter().position(|&(ii, _)| ii == i) {
            n1[index] = (i2, d1 + d2);
        }
        let n2 = graph.get_mut(&i2).unwrap();
        if let Some(index) = n2.iter().position(|&(ii, _)| ii == i) {
            n2[index] = (i1, d1 + d2);
        }
    }

    let indexes = graph.keys().enumerate().map(|(i, pos)| (pos, i)).collect::<HashMap<_,_>>();
    let mut idx_graph = vec![Vec::new(); graph.len()];
    for (pos, neighbors) in &graph {
        idx_graph[indexes[pos]] = neighbors.iter().map(|&(i, d)| (indexes[&i], d)).collect();
    }
  
    let end = indexes[&(grid.len()-2)];
    dfs(&idx_graph, end, indexes[&1])
}

fn dfs(graph: &[Vec<(usize,usize)>], end: usize, start: usize) -> usize {
    let seen = vec![false; graph.len()];
    let mut stack = vec![];
    stack.push((start, 0, seen));

    let mut max_dist = 0;
    while let Some((curr, dist, seen)) = stack.pop() {
        if curr == end && max_dist < dist {
            max_dist = dist;
            continue
        }
        for &(next, d) in &graph[curr] {
            let mut seen = seen.clone();
            if !seen[next] {
                seen[next] = true;
                stack.push((next, d+dist, seen));
            }
        }
    }
    max_dist
  }