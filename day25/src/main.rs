use std::{fs, env, time::Instant, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let now = Instant::now();

    let (graph, edge_list) = parse_contents(&contents);

    let part1 = solve_part_1(&graph, &edge_list);

    println!("Part 1: {part1}");
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}

fn parse_contents(contents: &String) -> (Vec<Vec<usize>>, Vec<(usize, usize)>)  {
    let mut graph = HashMap::<_,Vec<_>>::new();
    let mut edge_list = vec![];

    for line in contents.lines() {
        let mut data = line.split(":");
        let left = data.next().unwrap().trim();
        let mut connections: Vec<&str> = data.next().unwrap().trim().split_whitespace().collect();
        for &connection in &connections {
            let other = graph.entry(connection).or_default();
            other.push(left);
            edge_list.push((left, connection));
        }
        let v = graph.entry(left).or_default();
        v.append(&mut connections);
    }

    let indexes = graph.keys().enumerate().map(|(i, name)| (name, i)).collect::<HashMap<_,_>>();
    let edge_list = edge_list.iter().map(|&x| (indexes[&x.0], indexes[&x.1])).collect();
    let mut idx_graph = vec![Vec::new(); graph.len()];
    for (name, connections) in &graph {
        idx_graph[indexes[name]] = connections.iter().map(|&x| indexes[&x]).collect();
    }
    (idx_graph, edge_list)
}

fn solve_part_1(graph: &Vec<Vec<usize>>, edge_list: &Vec<(usize, usize)>) -> usize {
    for i in 0..edge_list.len() {
        for j in i+1..edge_list.len() {
            let mut graph = graph.clone();
            for edge in [edge_list[i], edge_list[j]] {
                graph[edge.0].retain(|&x| x != edge.1);
                graph[edge.1].retain(|&x| x != edge.0);
            }
            let cut_edge = dfs_find_cut_edge(&graph);
            if let Some(edge) = cut_edge {
                graph[edge.0].retain(|&x| x != edge.1);
                graph[edge.1].retain(|&x| x != edge.0);
            }
            let components = dfs_for_component_sizes(&graph);
            if components.len() == 2 {
                return components.iter().fold(1, |acc, x| acc * x);
            }
        }
    }   
    0
}

fn dfs_for_component_sizes(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut components = vec![];
    let mut seen = vec![false; graph.len()];
    let mut stack = vec![];
    for u in 0..graph.len() {
        if !seen[u] {
            let mut counter = 1;
            stack.push(u);
            seen[u] = true;
            while let Some(node) = stack.pop() {
                for &x in &graph[node] {
                    if !seen[x] {
                        seen[x] = true;
                        stack.push(x);
                        counter += 1;
                    }
                }
            }
            components.push(counter);
        }
    }
    components
}

fn dfs_find_cut_edge(graph: &Vec<Vec<usize>>) -> Option<(usize, usize)> {
    let mut seen = vec![false; graph.len()];
    let mut disc = vec![0; graph.len()];
    let mut low = vec![0; graph.len()];
    let mut cut_edges = vec![];
    
    dfs_find_cut_edge_recurse(graph, 0, &mut seen, &mut disc, &mut low, 0, -1, &mut cut_edges);
    if cut_edges.len() > 0 {
        return Some(cut_edges[0])
    }
    None
}

fn dfs_find_cut_edge_recurse(graph: &Vec<Vec<usize>>, node: usize, seen: &mut Vec<bool>, disc: &mut Vec<usize>, low: &mut Vec<usize>, time: usize, parent: isize, cut_edges: &mut Vec<(usize, usize)>) {
    seen[node] = true;
    disc[node] = time;
    low[node] = time;
    let time = time + 1;

    for &adj in &graph[node] {
        if adj as isize == parent { continue }
        if !seen[adj] {
            dfs_find_cut_edge_recurse(graph, adj, seen, disc, low, time, node as isize, cut_edges);
            low[node] = low[node].min(low[adj]);
            if low[adj] > disc[node] {
                cut_edges.push((node, adj));
            }
        }
        else if adj as isize != parent{
            low[node] = low[node].min(disc[adj]);
        }
    }
}