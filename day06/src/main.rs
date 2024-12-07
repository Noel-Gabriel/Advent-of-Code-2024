use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};

fn main() {

    let (pos, dir, mut grid) = read_input();
    let mut cells = traverse(pos, dir, &grid);

    println!("Unique cells visited: {}", cells.len());

    cells.remove(&pos); // cannot place obstacle on guard's starting position
    let num_loops = cells
        .into_iter()
        .filter(|&((x, y), _)| {
            grid[x as usize][y as usize] = 1;
            let cycle = traverse(pos, dir, &grid).len() == 0;
            grid[x as usize][y as usize] = 0;
            cycle
        })
        .count() as i32;

    println!("Blocking Obstacles: {}", num_loops);
}

fn read_input() -> ((i32, i32), usize, Vec<Vec<i32>>) {
    let mut file = File::open("input06.txt").expect("Could not open file.");
    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    let mut pos = (0,0);
    let mut dir = 0usize;

    let grid = s.split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        '.' => 0,
                        '#' => 1,
                        '^' | 'v' | '<' | '>'  => {
                            (pos, dir) = ((i as i32, j as i32) , map_guard(c));
                            0
                        }
                         _  => unreachable!(),
                    }
                }).collect()
        }).collect();

    (pos, dir, grid)
}

fn map_guard(c: char) -> usize {
    match c {
        '^' => 0usize,
        '>' => 1usize,
        'v' => 2usize,
        '<' => 3usize,
         _  => unreachable!(),
    }
}

fn valid(x: i32, y: i32, grid: &Vec<Vec<i32>>) -> bool {
    0 <= x && x < grid.len() as i32 && 0 <= y && y < grid[x as usize].len() as i32
}

fn traverse((mut x, mut y): (i32, i32), mut dir: usize, grid: &Vec<Vec<i32>>) -> HashMap<(i32, i32), HashSet<usize>> {
    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut cells: HashMap<(i32, i32), HashSet<usize>> = HashMap::new();
    let mut backtracked = false;
    while valid(x, y, &grid) {
        let (dx, dy) = directions[dir];
        if grid[x as usize][y as usize] == 0 {
            if !backtracked && cells.contains_key(&(x, y)) && cells.get(&(x,y)).unwrap().contains(&dir) {
                return HashMap::new()
            }
            cells.entry((x,y))
                .and_modify(|hs| { hs.insert(dir); })
                .or_insert( HashSet::from([dir]) );
            (x, y) = (x + dx, y + dy);
            backtracked = false;
        } else {
            (x, y) = (x - dx, y - dy);
            dir = (dir + 1) % 4;
            backtracked = true;
        }
    }
    cells
}
