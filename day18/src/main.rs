use std::fs::File;
use std::io::Read;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

mod union_find;

fn main() {
    let bytes = read_input();

    let grid_width = 71;
    let grid_height = 71;
    let to_simulate = 1024;

    let shortest_path = dijkstra(&bytes, grid_width, grid_height, to_simulate);
    let (x, y) = first_byte_to_block(&bytes, grid_width as usize, grid_height as usize);

    println!("Shortest path: {}", shortest_path);
    println!("First blocking byte: {},{}", y, x); // loaded the bytes in reverse
}

fn read_input() -> Vec<(usize, usize)> {
    let mut s = String::new();
    let _ = File::open("input18.txt")
        .expect("Could not open File.")
        .read_to_string(&mut s);

    s
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(",")
                .expect("Could not split.");
            (y.parse::<usize>().expect("Could not parse as usize."),
             x.parse::<usize>().expect("Could not parse as usize."))
        }).collect()
}

fn dijkstra(bytes: &Vec<(usize, usize)>, width: i32, height: i32, to_simulate: usize) -> i32 {
    let mut grid = vec![ vec![1; width as usize]; height as usize];
    for i in 0..to_simulate { 
        let (bx, by) = bytes[i];
        grid[bx][by] = 0;
    }
    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let start: (usize, usize) = (0, 0);
    grid[0][0] = 2;
    let end = (width as usize - 1, height as usize - 1);
    let mut shortest = BinaryHeap::from([Reverse((0, start))]);
    while let Some(Reverse((dist, (x, y)))) = shortest.pop() {
        if (x, y) == end { return dist }
        for (dx, dy) in dir {
            let (u, v) = (x as i32 + dx, y as i32 + dy);
            if 0 <= u && u < height && 0 <= v && v < width && grid[u as usize][v as usize] == 1 {
                shortest.push(Reverse((dist + 1, (u as usize, v as usize))));
                grid[u as usize][v as usize] = 2;
            }
        }
    }
    -1
}

// assumes that (0, 0) and (width - 1, height - 1) are disconnected if all bytes have fallen
fn first_byte_to_block(bytes: &Vec<(usize, usize)>, width:usize, height: usize) -> (usize, usize) {
    let mut grid = vec![ vec![1; width]; height];
    for &(bx, by) in bytes { grid[bx][by] = 0; }
    let mut uf = build_union_find(&grid);
    let s: usize  = 0;
    let e: usize = height * width - 1;
    for &(bx, by) in bytes.iter().rev() {
        grid[bx][by] = 1;
        connect_neighbours((bx, by), &grid, &mut uf);
        if uf.find(s) == uf.find(e) {
            return (bx, by)
        }
    }
    panic!("Blocking Byte not found!");
}

fn build_union_find(grid: &Vec<Vec<i32>>) -> union_find::UnionFind {
    let mut uf = union_find::UnionFind::new(grid.len() * grid[0].len()); 
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 0 { continue; }
            connect_neighbours((x, y), grid, &mut uf); 
        }
    }
    uf
}

fn connect_neighbours((x, y): (usize, usize), grid: &Vec<Vec<i32>>, uf: &mut union_find::UnionFind) {
    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for (dx, dy) in dir {
        let (u, v) = (x as i32 + dx, y as i32 + dy);
        if 0 <= u && u < grid.len() as i32 && 0 <= v && v < grid[u as usize].len() as i32 
                && grid[u as usize][v as usize] == 1 {
            uf.union(x * grid[x].len() + y, u as usize * grid[u as usize].len() + v as usize);
        }
    }
}
