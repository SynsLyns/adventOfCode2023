use std::{fs, env, time::Instant, cmp, collections::{HashMap, HashSet, VecDeque}};

#[derive(Debug)]
struct Coords3D {
    x: usize,
    y: usize,
    z: usize
}

#[derive(Debug)]
struct Brick {
    start: Coords3D,
    end: Coords3D,
    id: usize
}

impl Coords3D {
    fn new(x: usize, y: usize, z: usize) -> Coords3D {
        Coords3D {
            x,
            y,
            z
        }
    }
}

impl Brick {
    fn min_z(&self) -> usize {
        cmp::min(self.start.z, self.end.z)
    }

    fn max_z(&self) -> usize {
        cmp::max(self.start.z, self.end.z)
    }

    fn min_y(&self) -> usize {
        cmp::min(self.start.y, self.end.y)
    }

    fn max_y(&self) -> usize {
        cmp::max(self.start.y, self.end.y)
    }

    fn min_x(&self) -> usize {
        cmp::min(self.start.x, self.end.x)
    }

    fn max_x(&self) -> usize {
        cmp::max(self.start.x, self.end.x)
    }

    fn move_down(&mut self)  {
        self.start.z -= 1;
        self.end.z -= 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let now = Instant::now();

    let mut bricks = vec![];
    let mut removeable_map = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for (i, line) in contents.lines().enumerate() {
        let mut data = line.split("~");
        let coord1: Vec<usize> = data.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let coord2: Vec<usize> = data.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        for coord in [&coord1, &coord2] {
            if coord[0] > max_x {
                max_x = coord[0];
            }
            if coord[1] > max_y {
                max_y = coord[1];
            }
            if coord[2] > max_z {
                max_z = coord[2];
            }
        }
        bricks.push(Brick 
                        {
                            start: Coords3D::new(coord1[0], coord1[1], coord1[2]),
                            end: Coords3D::new(coord2[0], coord2[1], coord2[2]),
                            id: i
                        }
                    );
        removeable_map.insert(i, (true, HashSet::new(), HashSet::new()));
    }
    bricks.sort_by_key(|a| a.min_z());

    max_x += 1;
    max_y += 1;
    max_z += 1;
    let mut grid3d = vec![None; max_x*max_y*max_z];
    
    for brick in &mut bricks {
        drop(brick, &mut grid3d, max_x, max_y, &mut removeable_map);
    }

    let removable_num = removeable_map.values().filter(|value| value.0 == true).count();

    println!("Part 1: {removable_num}");

    let mut part2 = 0;
    for (id, (removable, _, _)) in &removeable_map {
        if !removable {
            part2 += get_above(id, &removeable_map);
        }
    }

    println!("Part 2: {part2}");

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}

fn drop<'a>(brick: &'a mut Brick, grid3d: &mut Vec<Option<&'a Brick>>, max_x: usize, max_y: usize, removeable_map: &mut HashMap<usize, (bool, HashSet<usize>, HashSet<usize>)>) {
    while brick.min_z() > 1 && below_empty(brick, grid3d, max_x, max_y, removeable_map) {
        brick.move_down();
    }
    for i in brick.min_x()..=brick.max_x() {
        for j in brick.min_y()..=brick.max_y() {
            for k in brick.min_z()..=brick.max_z() {
                grid3d[(k * max_y * max_x) + (j * max_x) + i] = Some(brick);
                
            }
        }
    }
}

fn below_empty(brick: &Brick, grid3d: &Vec<Option<&Brick>>, max_x: usize, max_y: usize, removeable_map: &mut HashMap<usize, (bool, HashSet<usize>, HashSet<usize>)>) -> bool {
    let mut below = HashSet::new();
    if brick.start.z != brick.end.z {
        match grid3d[((brick.min_z() - 1) * max_y * max_x) + (brick.start.y * max_x) + brick.start.x] {
            Some(x) => {
                removeable_map.get_mut(&x.id).unwrap().1.insert(brick.id);
                removeable_map.get_mut(&brick.id).unwrap().2.insert(x.id);
                removeable_map.get_mut(&x.id).unwrap().0 = false;
                return false
            }
            None => ()
        }
    }
    else if brick.start.y != brick.end.y {
        for i in brick.min_y()..brick.max_y()+1 {
            match grid3d[((brick.start.z - 1) * max_y * max_x) + (i * max_x) + brick.start.x] {
                Some(x) => {
                    below.insert(x.id);
                }
                None => ()
            }
        }
    }
    else {
        for i in brick.min_x()..brick.max_x()+1 {
            match grid3d[((brick.start.z - 1) * max_y * max_x) + (brick.start.y * max_x) + i] {
                Some(x) => {
                    below.insert(x.id);
                }
                None => ()
            }
        }
    }
    if below.len() > 0 {
        for item in &below {
            removeable_map.get_mut(&item).unwrap().1.insert(brick.id);
            removeable_map.get_mut(&brick.id).unwrap().2.insert(*item);
        }
        if below.len() == 1 {
            removeable_map.get_mut(below.iter().next().unwrap()).unwrap().0 = false;
        }
        return false
    }
    true
}

fn get_above(base: &usize, map: &HashMap<usize, (bool, HashSet<usize>, HashSet<usize>)>) -> usize {
    let mut falling = HashSet::new();
    falling.insert(base);
    let mut explore: VecDeque<_> = map.get(&base).unwrap().1.iter().collect();
    let mut seen = HashSet::new();
    'outer: while let Some(x) = explore.pop_front() {
        if seen.contains(x) {
            continue
        }
        for i in &map.get(x).unwrap().2 {
            if !falling.contains(i) {
                continue 'outer
            }
        }
        falling.insert(x);
        let mut add: VecDeque<_> = map.get(x).unwrap().1.iter().collect();
        explore.append(&mut add);
        seen.insert(x);
    }
    seen.len()
}