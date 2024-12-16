use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let (s, e, maze) = read_input();

    let (min_score, tiles) = traverse(s, e, &maze);

    println!("Min score: {}", min_score);
    println!("Tiles part of at least one min score path: {}", tiles);
}

fn read_input() -> ((i32, i32), (i32, i32), Vec<Vec<char>>) {
    let mut s = String::new();
    let _ = File::open("input16.txt")
        .expect("Could not open File.")
        .read_to_string(&mut s);

    let (mut sx, mut sy): (i32, i32) = (0, 0);
    let (mut ex, mut ey): (i32, i32) = (0, 0);

    let maze = s
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        'S' => { (sx, sy) = (i as i32, j as i32); '.' },
                        'E' => { (ex, ey) = (i as i32, j as i32); '.' },
                         _  => c,
                    }
                }).collect()
        }).collect();

    ((sx, sy), (ex, ey), maze)
}


fn traverse(s: (i32, i32), e: (i32, i32), maze: &Vec<Vec<char>>) -> (u32, usize) {
    let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![ vec![std::u32::MAX; maze[0].len() ]; maze.len() ];
    let mut min_score: u32 = std::u32::MAX;
    let mut best_tiles: HashSet<(i32, i32)> = HashSet::from([e]);
    let mut current_tiles: HashSet<(i32, i32)> = HashSet::new();
    dfs(s, e, &maze, 0u32, &mut min_score, &dir, (0, 1), &mut visited, &mut current_tiles, &mut best_tiles);
    (min_score, best_tiles.len())
}

fn dfs(
    (x, y): (i32, i32),
    (ex, ey): (i32, i32),
    maze: &Vec<Vec<char>>,
    current_score: u32,
    min_score: &mut u32,
    dir: &[(i32, i32)],
    current_dir: (i32, i32),
    visited: &mut Vec<Vec<u32>>,
    current_tiles: &mut HashSet<(i32, i32)>,
    best_tiles: &mut HashSet<(i32, i32)>) {

    let future_dist = visited[(x + current_dir.0) as usize][(y + current_dir.1) as usize];
    let dist = &mut visited[x as usize][y as usize];
    // might still be a good path if we previously had to turn after this tile
    // check the next tile if we can still keep up by going straight if we previously turned
    if *dist < current_score && (future_dist == std::u32::MAX || future_dist != current_score + 1) { return () }
    *dist = current_score;
    if (x, y) == (ex, ey) { 
        if *min_score > current_score { 
            best_tiles.clear();
            best_tiles.insert((ex, ey));
        }
        for tile in current_tiles.iter() { best_tiles.insert(*tile); }
        *min_score = current_score; 
        return ()
    }
    current_tiles.insert((x,y));
    for &(dx, dy) in dir {
        let (u, v) = (x + dx, y + dy);
        if maze[u as usize][v as usize] == '#' { continue; }
        if current_dir == (dx, dy) {
            dfs((u,v), (ex, ey), maze, current_score + 1, min_score, dir, current_dir, visited, current_tiles, best_tiles);
        } else {
            dfs((u,v), (ex, ey), maze, current_score + 1001, min_score, dir, (dx, dy), visited, current_tiles, best_tiles);
        }
    }
    current_tiles.remove(&(x,y));
}
