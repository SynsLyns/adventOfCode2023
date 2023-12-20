use std::fs;
use std::env;

const OFFSETS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

#[derive(Copy, Clone, Debug)]
struct Position {
    row: usize,
    column: usize
}

#[derive(Debug)]
struct Node {
    char: char,
    visited: bool,
    loop_hits: u32
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Node>,
    rows: usize,
    columns: usize
}

impl Position {
    fn new(row: usize, column: usize) -> Position {
        Position {
            row, 
            column
        }
    }
 }

impl Grid {
    fn get_node_at(&mut self, row: usize, column: usize) -> &mut Node {
        &mut self.grid[row*self.columns + column]
    }
    
    fn push(&mut self, item: Node) {
        self.grid.push(item);
    }
}

impl Node {
    fn new(char: char) -> Node {
        Node {
            char,
            visited: false,
            loop_hits: 0
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let rows = contents.lines().count();
    let columns = contents.lines().next().unwrap().len();

    let mut grid: Grid = Grid {
        grid: vec![],
        rows,
        columns
    };

    let mut row = 0;
    let mut column = 0;

    let mut start_position: Position = Position::new(0, 0);
    for line in contents.lines() {
        for char in line.chars() {
            grid.push(Node::new(char));
            if char == 'S' {
                start_position = Position::new(row, column);
            }
            column = (column + 1) % columns;
        }
        row += 1;
    }

    let mut connects_to_start: Vec<Position> = vec![];
    let mut right = false;
    let mut left = false;
    let mut down = false;
    let mut top = false;
    for offset in OFFSETS {
        let row = start_position.row as i32 + offset.0;
        let col = start_position.column as i32 + offset.1;
        if row >= rows as i32 || row < 0 || col >= columns as i32 || col < 0 {
            continue
        }
        let row = row as usize;
        let col = col as usize;
        match grid.get_node_at(row, col).char {
            '|' => {
                if offset.0 != 0 {
                    connects_to_start.push(Position::new(row, col));
                    if offset.0 == 1 {
                        down = true;
                    } else {
                        top = true;
                    }
                }
            }
            '-' => {
                if offset.1 != 0 {
                    connects_to_start.push(Position::new(row, col));
                    if offset.1 == 1 {
                        left = true;
                    } else {
                        right = true;
                    }
                }
            }
            'L' => {
                if offset.0 == 1 || offset.1 == -1 {
                    connects_to_start.push(Position::new(row, col));
                    if offset.0 == 1 {
                        down = true;
                    }
                    else {
                        left = true;
                    }
                }
            }
            'J' => {
                if offset.0 == 1 || offset.1 == 1 {
                    connects_to_start.push(Position::new(row, col));
                    if offset.0 == 1 {
                        down = false;
                    }
                    else {
                        right = true;
                    }
                }
            }
            '7' => {
                if offset.0 == -1 || offset.1 == 1 {
                    connects_to_start.push(Position::new(row, col));
                    if offset.0 == -1 {
                        top = true;
                    }
                    else {
                        right = true;
                    }
                }
            }
            'F' => {
                if offset.0 == -1 || offset.1 == -1 {
                    connects_to_start.push(Position::new(row, col));
                    if offset.0 == -1 {
                        top = true;
                    }
                    else {
                        left = false;
                    }
                }
            }
            _ => ()
        }
    }

    let mut part1 = 1;

    grid.get_node_at(start_position.row, start_position.column).visited = true;
    
    'outer: loop {
        for pos in &mut connects_to_start {
            let node = grid.get_node_at(pos.row, pos.column);
            if node.visited == true {
                break 'outer
            }
            node.visited = true;
            match node.char {
                '|' => {
                    if !grid.get_node_at(pos.row + 1, pos.column).visited {
                        pos.row += 1;
                    }
                    else {
                        pos.row -= 1;
                    }
                }
                '-' => {
                    if !grid.get_node_at(pos.row, pos.column + 1).visited {
                        pos.column += 1;
                    }
                    else {
                        pos.column -= 1;
                    }
                }
                'L' => {
                    if !grid.get_node_at(pos.row - 1, pos.column).visited {
                        pos.row -= 1;
                    }
                    else {
                        pos.column += 1;
                    }
                }
                'J' => {
                    if !grid.get_node_at(pos.row - 1, pos.column).visited {
                        pos.row -= 1;
                    }
                    else {
                        pos.column -= 1;
                    }
                }
                '7' => {
                    if !grid.get_node_at(pos.row + 1, pos.column).visited {
                        pos.row += 1;
                    }
                    else {
                        pos.column -= 1;
                    }
                }
                'F' => {
                    if !grid.get_node_at(pos.row + 1, pos.column).visited {
                        pos.row += 1;
                    }
                    else {
                        pos.column += 1;
                    }
                }
                _ => ()
            }
        }
        part1 += 1;
    }

    let mut grid2: Grid = Grid {
        grid: vec![],
        rows,
        columns
    };

    for i in &mut grid.grid {
        if i.visited {
            if i.char == 'S' {
                if left && down {
                    i.char = '7';
                }
                else if right && top {
                    i.char = 'L';
                }
            }
            grid2.push(Node::new(i.char));
        }
        else {
            grid2.push(Node::new('.'));
        }
    }

    for i in 0..rows {
        grid2.get_node_at(i, 0).loop_hits = 0;
    }
    for i in 0..columns {
        grid2.get_node_at(0, i).loop_hits = 0;
    }

    let mut part2 = 0;
    for i in 1..rows {
        for j in 1..columns {
            let mut loop_hits = 0;
            let hit_node: &Node = grid2.get_node_at(i-1, j-1);
            match hit_node.char {
                '.' => (),
                '7' => (),
                'L' => (),
                _ => {
                    loop_hits += 1;
                }
            }
            loop_hits += hit_node.loop_hits;
            let node =  grid2.get_node_at(i, j);
            if node.char == '.' && loop_hits % 2 == 1 {
                part2 += 1;
            }
            node.loop_hits = loop_hits;
        }
    }

    println!("PART 1: {part1} PART 2: {part2}");
}
