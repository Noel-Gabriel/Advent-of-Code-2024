use std::fs::File;
use std::io::Read;

fn main() {

    let (pos, dir, mut grid) = read_input();
    let cells = traverse(pos, dir, &grid).unwrap();

    println!("Unique cells visited: {}", cells.len());

    let num_loops = cells[1..]
        .into_iter()
        .filter(|(x, y)| {
            grid[*x as usize][*y as usize] = 1;
            let cycle = traverse(pos, dir, &grid).is_none();
            grid[*x as usize][*y as usize] = 0;
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

fn traverse((mut x, mut y): (i32, i32), mut dir: usize, grid: &Vec<Vec<i32>>) -> Option<Vec<(i32, i32)>> {
    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited = vec![ vec![ vec![false; 4]; grid.len() ]; grid[0].len() ]; 
    let mut cells = Vec::new();
    let mut backtracked = false;
    while valid(x, y, &grid) {
        let (dx, dy) = directions[dir];
        if grid[x as usize][y as usize] == 0 {
            if visited[x as usize][y as usize][dir] && !backtracked {
                return None
            } else if visited[x as usize][y as usize].iter().all(|d| !d ) {
                cells.push((x, y));
            }
            visited[x as usize][y as usize][dir] = true;
            (x, y) = (x + dx, y + dy);
            backtracked = false;
        } else {
            (x, y) = (x - dx, y - dy);
            dir = (dir + 1) % 4;
            backtracked = true;
        }
    }
    Some(cells)
}
