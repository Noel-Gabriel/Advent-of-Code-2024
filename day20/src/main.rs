use std::fs::File;
use std::io::Read;

fn main() {
    let (s, e, grid) = read_input();

    const MIN_DIST_CUT: usize = 100;
    const MAX_CHEAT_DUR: usize = 20;
    
    let path = get_path(s, e, &grid);
    let shortcuts = find_shortcuts(&path, MIN_DIST_CUT, MAX_CHEAT_DUR);

    println!("Cheats saving at least {}s with duration of at most {}s: {}", MIN_DIST_CUT, MAX_CHEAT_DUR, shortcuts); 
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

fn get_path(
    (sx, sy): (usize, usize), 
    (ex, ey): (usize, usize), 
    grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {

    let mut distances = vec![ vec![-1; grid[0].len()]; grid.len()];
    let mut path = Vec::new();
    distances[sx][sy] = 0;
    path.push((sx, sy));
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
                    path.push((x as usize, y as usize));
                    break;
                }
            }
        }
    }
    path
}

fn find_shortcuts(
    path: &Vec<(usize, usize)>,
    min_cut: usize,
    max_dur: usize) -> u64 {

    let mut shortcuts: u64 = 0;
    for (i, &(sx, sy)) in path.iter().enumerate() {
        for end in (i+min_cut)..path.len() { // at least distance of min_cut apart
            let (ex, ey) = path[end];
            // best possible (shortest) cheat. Only works because of input constraints
            let cheat_duration = (ex as i32 - sx as i32).abs() + (ey as i32 - sy as i32).abs();
            // cheat_duration at most max_dur and saves enough time
            if cheat_duration <= max_dur as i32 && (end - i - cheat_duration as usize ) >= min_cut { 
                shortcuts += 1;
            }
        }
    }
    shortcuts
}
