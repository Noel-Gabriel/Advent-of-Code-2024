use std::fs::File;
use std::io::Read;

fn main() {
    let (s, e, grid) = read_input();

    const MIN_DIST_CUT: usize = 100;

    let distances = get_distances(s, e, &grid);
    let shortcuts = find_shortcuts(&distances, MIN_DIST_CUT);

    println!("Number of cheats saving at least {} seconds: {}", MIN_DIST_CUT, shortcuts); 
}

fn read_input() -> ((usize, usize), (usize, usize), Vec<Vec<char>>) {
    let mut s = String::new();
    let _ = File::open("input20.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    let (mut sx, mut sy) = (0, 0);
    let (mut ex, mut ey) = (0, 0);

    let grid = s
        .trim()
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut grid, (i, line)| {
            grid.push(line
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        '#' => '#',
                        '.' => '.',
                        'S' => { (sx, sy) = (i, j); '.' }
                        'E' => { (ex, ey) = (i, j); '.' }
                        _   => unreachable!(),
                    }
                }).collect::<Vec<char>>());
            grid
        });

    ((sx, sy), (ex, ey), grid)
}

fn get_distances((sx, sy): (usize, usize), (ex, ey): (usize, usize), grid: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut distances = vec![ vec![-1; grid[0].len()]; grid.len()];
    distances[sx][sy] = 0;
    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut x, mut y) = (sx as i32, sy as i32);
    while (x as usize, y as usize) != (ex, ey) {
        for (dx, dy) in dir {
            let (u, v) = (x + dx, y + dy);
            if 0 <= u && u < grid.len() as i32 && 0 <= v && v < grid[sx].len() as i32 {
                let current_dist = distances[x as usize][y as usize];
                let dist = &mut distances[u as usize][v as usize];
                if *dist == -1 && grid[u as usize][v as usize] == '.' {
                    *dist = current_dist + 1;
                    (x, y) = (u, v);
                    break;
                }
            }
        }
    }
    distances
}

fn find_shortcuts(distances: &Vec<Vec<i32>>, min_cut: usize) -> u32 {
    let mut shortcuts: u32 = 0; 
    for i in 1..distances.len() - 1 {
        for j in 1..distances[i].len() - 1 {
            if distances[i][j] == -1 { // #
                if distances[i][j-1] != -1 && distances[i][j+1] != -1 {
                    if (distances[i][j-1] - distances[i][j+1]).abs() - 2 >= min_cut as i32 { shortcuts += 1; } 
                }
                if distances[i-1][j] != -1 && distances[i+1][j] != -1 {
                    if (distances[i-1][j] - distances[i+1][j]).abs() - 2 >= min_cut as i32 { shortcuts += 1; }
                }
            }
        }
    }
    shortcuts
}
