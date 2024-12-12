use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet, VecDeque}; 

fn main() {
    let grid = read_input();

    let cost = bfs(&grid, false);
    let batch_cost = bfs(&grid, true);

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
 
    for (rotx, roty) in [rot1, rot2] {
        let (mut u, mut v) = (xn, yn);
        loop {
            (u, v) = (u + rotx, v + roty);
            if sides.contains_key(&(u,v)) {
                if sides
                    .get(&(u,v))
                    .unwrap()
                    .contains(&(dx, dy)) {
                        return 0
                }
            }
            let (w, z) = (u - dx, v - dy);
            if !valid(w, z, grid.len(), grid[0].len()) ||
                grid[w as usize][z as usize] != garden {
                    break;
            }
            if !outside {
                if !valid(u, v, grid.len(), grid[0].len()) ||
                    grid[u as usize][v as usize] == garden {
                        break;
                }
            } 
        }
    }

    sides
        .entry((xn, yn))
        .and_modify(|hs| { hs.insert((dx, dy)); } )
        .or_insert(HashSet::from([(dx, dy)]));
    return 1
}

fn valid(xn: i32, yn: i32, m: usize, n: usize) -> bool {
    0 <= xn && xn < m as i32 && 0 <= yn && yn < n as i32
}

fn bfs(grid: &Vec<Vec<char>>, discount: bool) -> i64 {
    let mut ans = 0;
    let mut current_garden: VecDeque<(usize, usize)> = VecDeque::new();
    let mut unvisited_garden: VecDeque<(usize, usize)> = VecDeque::new();
    unvisited_garden.push_back((0, 0));
    let mut visited = vec![ vec![false; grid[0].len() ]; grid.len() ];
    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((x, y)) = unvisited_garden.pop_front() {
        if visited[x][y] { continue; }
        let current: char = grid[x][y];
        current_garden.push_back((x, y));
        let mut sides: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
        let mut area: i64  = 1;
        let mut perimeter: i64 = 0;        
        while let Some((xc, yc)) = current_garden.pop_front() {
            visited[xc][yc] = true;
            for (dx, dy) in dir {
                let (xn, yn) = (xc as i32 + dx, yc as i32 + dy);
                if valid(xn, yn, grid.len(), grid[0].len()) {
                    let next = grid[xn as usize][yn as usize];
                    if next == current { 
                        if !visited[xn as usize][yn as usize] {
                            area += 1;
                            visited[xn as usize][yn as usize] = true;
                            current_garden.push_back((xn as usize, yn as usize));
                        }
                    } else { 
                        if !discount { perimeter += 1; }
                        else { perimeter += validate_side(current, (xn, yn), (dx, dy), &mut sides, &grid); }
                        unvisited_garden.push_back((xn as usize, yn as usize));
                    }
                } else { 
                    if !discount { perimeter += 1; }
                    else { perimeter += validate_side(current, (xn, yn), (dx, dy), &mut sides, &grid); }
                }
            }
        }
        ans += area * perimeter;
    }
    ans
}
