use std::fs::File;
use std::io::Read;
use std::collections::linked_list;

fn main() {
    let (trailheads, grid) = read_input();

    let trailheads_scores = bfs(&trailheads, &grid, false);
    let trailheads_ratings = bfs(&trailheads, &grid, true);

    println!("Trailheads scores: {}", trailheads_scores);
    println!("Trailheads ratings: {}", trailheads_ratings);
}

fn read_input() -> (Vec<(usize, usize)>, Vec<Vec<i32>>) {
    let mut s = String::new();

    let mut trailheads: Vec<(usize, usize)> = Vec::new();

    let _ = File::open("input10.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    let grid: Vec<Vec<i32>> = s
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    let d: i32 = c as i32 - '0' as i32;
                    if d == 0 { trailheads.push((i, j)); }
                    d
                }).collect()
        }).collect();

    (trailheads, grid)
}

fn bfs(trailheads: &Vec<(usize, usize)>, grid: &Vec<Vec<i32>>, rating: bool) -> i32 {
    let mut ans = 0;
    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for &trailhead in trailheads {
        let mut q = linked_list::LinkedList::from([trailhead]);
        let mut visited = vec![ vec![false; 20]; 20]; // path of length at most 9
        while let Some((x, y)) = q.pop_front() {
            if !rating {
                if visited[x + 9 - trailhead.0][y + 9 - trailhead.1] { continue; }
                visited[x + 9 - trailhead.0][y + 9 - trailhead.1] = true;
            }
            if grid[x][y] == 9 { ans += 1; continue; }
            for (dx, dy) in dir {
                let (xn, yn) = (x as i32 + dx, y as i32 + dy);
                if 0 <= xn && xn < grid.len() as i32 && 0 <= yn && yn < grid[0].len() as i32 {
                    if grid[xn as usize][yn as usize] == grid[x][y] + 1 { 
                        q.push_back((xn as usize, yn as usize)); 
                    }
                }
            }
        }
    }
    ans
}

