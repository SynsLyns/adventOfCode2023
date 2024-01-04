use std::{fs, env, time::Instant, ops::Range};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Clone, Copy)]
struct HailStone {
    px: Decimal,
    py: Decimal,
    pz: Decimal,
    vx: Decimal,
    vy: Decimal,
    vz: Decimal
}

impl HailStone {
    fn get_xy_intersection(&self, other: &HailStone) -> Option<(Decimal, Decimal)> {
        let a11 = self.vx;
        let a21 = self.vy;
        let a12 = -other.vx;
        let a22 = -other.vy;

        let det = a11*a22 - a21*a12;
        if det.abs() < dec![1e-4] { return None }

        let i11 = a22 / det;
        let i21 = -a21 / det;
        let i12 = -a12 / det;
        let i22 = a11 / det;

        let b1 = other.px - self.px;
        let b2 = other.py - self.py;
        
        let x = i11 * b1 + i12 * b2;
        let y = i21 * b1 + i22 * b2;
        if x < dec![0] || y < dec![0] { return None }

        return Some((self.px + self.vx * x, self.py + self.vy * x))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let now = Instant::now();

    let hail_stones = parse_contents(&contents);

    let part_1 = solve_part_1(&hail_stones, dec![200000000000000]..dec![400000000000001]);
    let part_2 = solve_part_2(&hail_stones);

    println!("Part 1: {part_1}");
    println!("Part 2: {}", part_2.round());
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}

fn parse_contents(contents: &String) -> Vec<HailStone> {
    let mut hail_stones = vec![];
    for line in contents.lines() {
        let data: Vec<Decimal> = line.replace("@", ",").split(',').map(|x| x.trim().parse().unwrap()).collect();
        hail_stones.push(
            HailStone {
                px: data[0],
                py: data[1],
                pz: data[2],
                vx: data[3],
                vy: data[4],
                vz: data[5]
            }
        )
    }
    hail_stones
}

fn solve_part_1(hail_stones: &Vec<HailStone>, range: Range<Decimal>) -> usize {
    let mut intersects_in_range = 0;
    for i in 0..hail_stones.len() {
        for j in i+1..hail_stones.len() {
            let s1 = &hail_stones[i];
            let s2 = &hail_stones[j];
            match s1.get_xy_intersection(&s2) {
                None => (),
                Some((x,y)) => {
                    if range.contains(&x) && range.contains(&y) {
                        intersects_in_range += 1;
                    }
                }
            }
        }
    }
    intersects_in_range
}

fn solve_part_2(hail_stones: &Vec<HailStone>) -> Decimal {
    for i in 0..hail_stones.len() {
        for j in i+1..hail_stones.len() {
            'some: for k in j+1..hail_stones.len() {
                let mut A = generate_matrix(hail_stones[i], hail_stones[j], hail_stones[k]);
                
                let solution = gaussian_elim(&mut A);

                for x in 0..A.len() {
                    if A[x][x].abs() < dec![1e-4] {
                        continue 'some
                    }
                }
                return solution[0] + solution[1] + solution[2]
            }
        }
    }
    dec![0]
}

fn generate_matrix(a: HailStone, b: HailStone, c: HailStone) -> Vec<Vec<Decimal>> {
    let mut matrix = vec![vec![dec![0]; 7]; 6];

    for (idx, (a, b)) in [(a, b), (b, c)].iter().enumerate() {
        matrix[0+idx*3][0] = a.vy - b.vy;
        matrix[0+idx*3][1] = b.vx - a.vx;
        matrix[0+idx*3][3] = b.py - a.py;
        matrix[0+idx*3][4] = a.px - b.px;
        
        matrix[1+idx*3][0] = a.vz - b.vz;
        matrix[1+idx*3][2] = b.vx - a.vx;
        matrix[1+idx*3][3] = b.pz - a.pz;
        matrix[1+idx*3][5] = a.px - b.px;

        matrix[2+idx*3][2] = a.vy - b.vy;
        matrix[2+idx*3][1] = b.vz - a.vz;
        matrix[2+idx*3][5] = b.py - a.py;
        matrix[2+idx*3][4] = a.pz - b.pz;

        matrix[0+idx*3][6] = a.px * a.vy - a.py * a.vx - b.px * b.vy + b.py * b.vx;
        matrix[1+idx*3][6] = a.px * a.vz - a.pz * a.vx - b.px * b.vz + b.pz * b.vx;
        matrix[2+idx*3][6] = a.pz * a.vy - a.py * a.vz - b.pz * b.vy + b.py * b.vz;
    }

    matrix
}

fn gaussian_elim(A: &mut Vec<Vec<Decimal>>) -> Vec<Decimal> {
    let rows = A.len();
    let cols = A[0].len();
    let mut h = 0; // pivot row
    let mut k = 0; // pivot col

    while h < rows && k < cols {
        let i_max = (h..rows).max_by_key(|&x| A[x][k].abs()).unwrap();
        if A[i_max][k] == dec![0] {
            k = k + 1;
        }
        else {
            A.swap(h, i_max);
            for i in h+1..rows {
                let f = A[i][k] / A[h][k];
                A[i][k] = dec![0];
                for j in k+1..cols {
                    A[i][j] = A[i][j] - A[h][j] * f;
                }
            }
            h = h + 1;
            k = k + 1;
        }
    }

    // BACK SUBSTITUTE
    let mut solution = vec![dec![0]; rows];
    for i in (0..rows).rev() {
        solution[i] = A[i][cols-1];
        for j in (i + 1)..rows {
            let tmp = solution[j];
            solution[i] -= A[i][j] * tmp;
        }
        solution[i] /= A[i][i];
    }

    solution
}