use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet, VecDeque}; 

fn main() {
    let grid = read_input();

    let cost = compute(&grid, false);
    let batch_cost = compute(&grid, true);

    println!("Cost of fence: {}", cost);
    println!("Cost of fence with batch discount: {}", batch_cost);
}

fn read_input() -> Vec<Vec<char>> {
    let mut s = String::new();

    let _ = File::open("input12.txt")
        .expect("Could not open file")
        .read_to_string(&mut s);

    s
        .trim()
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_ascii_uppercase())
                .collect()
        })
        .collect()
}

fn validate_side(
    garden: char,
    (xn, yn): (i32, i32),
    (dx, dy): (i32, i32), 
    sides: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>,
    grid: &Vec<Vec<char>>) -> i64 {

    let rot1 = (  dy,-dx);
    let rot2 = (- dy, dx);

    let outside = !valid(xn, yn, grid.len(), grid[0].len());
    // check if side already counted by following parameter 
    for (rotx, roty) in [rot1, rot2] {
        let (mut u, mut v) = (xn, yn);
        loop {
            (u, v) = (u + rotx, v + roty);
            if sides.contains_key(&(u,v)) {
                if sides
                    .get(&(u,v))
                    .unwrap()
                    .contains(&(dx, dy)) {
                        return 0 // already counted
                }
            }
            let (w, z) = (u - dx, v - dy);
            if !valid(w, z, grid.len(), grid[0].len()) ||
                grid[w as usize][z as usize] != garden {
                    break; // outside of current perimeter
            }
            if !outside {
                if !valid(u, v, grid.len(), grid[0].len()) ||
                    grid[u as usize][v as usize] == garden {
                        break; // found next perimeter (perpendicular to current)
                }
            } 
        }
    }
    // current side not counted, add to Map
    sides
        .entry((xn, yn))
        .and_modify(|hs| { hs.insert((dx, dy)); } )
        .or_insert(HashSet::from([(dx, dy)]));
    return 1
}

fn valid(xn: i32, yn: i32, m: usize, n: usize) -> bool {
    0 <= xn && xn < m as i32 && 0 <= yn && yn < n as i32
}

fn compute(grid: &Vec<Vec<char>>, discount: bool) -> i64 {
    let mut ans = 0;
    let mut unvisited_gardens: VecDeque<(usize, usize)> = VecDeque::new();
    unvisited_gardens.push_back((0, 0));
    let mut visited_gardens = vec![ vec![false; grid[0].len() ]; grid.len() ];
    while let Some((x, y)) = unvisited_gardens.pop_front() {
        if visited_gardens[x][y] { continue; }
        ans += bfs(&grid, (x, y), &mut visited_gardens, &mut unvisited_gardens, discount);
    }
    ans
}

fn bfs(
    grid: &Vec<Vec<char>>, 
    start: (usize, usize), 
    visited_gardens: &mut Vec<Vec<bool>>, 
    unvisited_gardens: &mut VecDeque<(usize, usize)>,
    discount: bool) -> i64 {

    let mut current_garden: VecDeque<(usize, usize)> = VecDeque::new();
    current_garden.push_back(start);
    let garden: char = grid[start.0][start.1];
    let mut sides: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let mut area: i64  = 1;
    let mut perimeter: i64 = 0;  
    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((x, y)) = current_garden.pop_front() {
            visited_gardens[x][y] = true;
            for (dx, dy) in dir {
                let (xn, yn) = (x as i32 + dx, y as i32 + dy);
                if valid(xn, yn, grid.len(), grid[0].len()) {
                    let next = grid[xn as usize][yn as usize];
                    if next == garden { 
                        if !visited_gardens[xn as usize][yn as usize] {
                            area += 1;
                            visited_gardens[xn as usize][yn as usize] = true;
                            current_garden.push_back((xn as usize, yn as usize));
                        }
                    } else { 
                        if !discount { perimeter += 1; }
                        else { perimeter += validate_side(garden, (xn, yn), (dx, dy), &mut sides, &grid); }
                        unvisited_gardens.push_back((xn as usize, yn as usize));
                    }
                } else { 
                    if !discount { perimeter += 1; }
                    else { perimeter += validate_side(garden, (xn, yn), (dx, dy), &mut sides, &grid); }
                }
            }
        }
    area * perimeter
}
