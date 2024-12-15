use std::fs::File;
use std::io::Read;

fn main() {
    let ((x, y), mut grid, robot_dir) = read_input();
    let mut grid_expanded = expand_grid(&grid);
        
    let gps = get_gps((x, y), &robot_dir, &mut grid, 'O', &advance);
    let gps_expanded = get_gps((x, y * 2), &robot_dir, &mut grid_expanded, '[', &advance_expanded);

    println!("Sum of gps: {}", gps);
    println!("Sum of gps with expanded grid: {}", gps_expanded);
}

fn read_input() -> ((usize, usize), Vec<Vec<char>>, Vec<(i32, i32)>) {
    let mut s = String::new();
    let _ = File::open("input15.txt")
        .expect("Could not open File.")
        .read_to_string(&mut s);

    let mut robot_pos: (usize, usize) = (0, 0);
    let (p1, p2) = s
        .trim()
        .split_once("\n\n")
        .expect("Could not find empty line");

    let grid = p1
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        '@' => { robot_pos = (i, j); '.' }
                         _  => c,
                    }
                }).collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();

    let robot_dir = p2
        .replace("\n", "")
        .chars()
        .map(|c| {
            match c {
                '^' => (-1, 0),
                '>' => ( 0, 1),
                'v' => ( 1, 0),
                '<' => ( 0,-1),
                 _  => unreachable!(),
            }
        }).collect::<Vec<(i32, i32)>>();

    (robot_pos, grid, robot_dir)
}

fn expand_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid_expanded = vec![ vec![' '; grid[0].len() * 2]; grid.len() ];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                'O' => {
                    grid_expanded[i][j * 2] = '[';
                    grid_expanded[i][j * 2 + 1] = ']';
                }
                c   => {
                    grid_expanded[i][j * 2] = c;
                    grid_expanded[i][j * 2 + 1] = c;
                }
            }
        }
    }
    grid_expanded
}

fn advance(
    (x, y): (usize, usize), 
    (dx, dy): (i32, i32), 
    grid: &mut Vec<Vec<char>>) -> (usize, usize) {

    let (u, v) = (x as i32 + dx, y as i32 + dy); // next pos
    let (mut w, mut z) = (u, v);
    while grid[w as usize][z as usize] == 'O' {
        (w, z) = (w + dx, z + dy);
    }
    
    match grid[w as usize][z as usize] {
        '#' => (x, y),
        '.' => {
            let tmp = grid[u as usize][v as usize];
            grid[u as usize][v as usize] = '.';
            grid[w as usize][z as usize] = tmp;
            (u as usize, v as usize)
        }
         _  => unreachable!(),
    }
}

fn advance_expanded(
    (x, y): (usize, usize),
    (dx, dy): (i32, i32),
    grid: &mut Vec<Vec<char>>) -> (usize, usize) {

    let (u, v) = (x as i32 + dx, y as i32 + dy);
    if !allowed((u, v), (dx, dy), (x as i32, y as i32), grid) { return (x, y) }

    move_boxes((u, v), (dx, dy), (x as i32, y as i32), grid);

    (u as usize, v as usize)
}

fn allowed((x, y): (i32, i32), (dx, dy): (i32, i32), prev: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    match grid[x as usize][y as usize] {
        '.' => true,
        '#' => false,
        '[' => {
            let current = allowed((x + dx, y + dy), (dx, dy), (x, y), grid);
            if dx == 0 || !current { return current } // moving left or right already
            let right = prev == (x, y + 1) || allowed((x, y + 1), (dx, dy), (x, y), grid);
            return right 
        }
        ']' => {
            let current = allowed((x + dx, y + dy), (dx, dy), (x, y), grid);
            if dx == 0 || !current { return current } // moving left or right already
            let left = prev == (x, y - 1) || allowed((x, y - 1), (dx, dy), (x, y), grid);
            return left
        }
         _  => unreachable!(),
    }
}

fn move_boxes((x, y): (i32, i32), (dx, dy): (i32, i32), prev: (i32, i32), grid: &mut Vec<Vec<char>>) {
    let (u, v) = (x + dx, y + dy);
    match grid[x as usize][y as usize] {
        '.' => (),
        '[' => {
            move_boxes((u, v), (dx, dy), (x, y), grid);
            grid[u as usize][v as usize] = grid[x as usize][y as usize];
            grid[x as usize][y as usize] = '.';
            if prev != (x, y + 1) && (dx, dy) != (0, 1) { move_boxes((x, y+1), (dx, dy), (x, y), grid); } 
        }
        ']' => {
            move_boxes((u, v), (dx, dy), (x, y), grid);
            grid[u as usize][v as usize] = grid[x as usize][y as usize];
            grid[x as usize][y as usize] = '.';
            if prev != (x, y - 1) && (dx, dy) != (0, -1) { move_boxes((x, y-1), (dx, dy), (x, y), grid); } 
        }
         _  => unreachable!(),
    }
}

fn get_gps(
    (mut x, mut y): (usize, usize), 
    robot_dir: &Vec<(i32, i32)>, 
    grid: &mut Vec<Vec<char>>,
    score_box: char,
    step: &dyn Fn((usize, usize), (i32, i32), &mut Vec<Vec<char>>) -> (usize, usize)) -> i64 {

    for dir in robot_dir {
        (x, y) = step((x, y), *dir, grid);
    }

    let mut gps: i64 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == score_box { gps += 100 * i as i64 + j as i64; }
        }
    }
    gps
}

