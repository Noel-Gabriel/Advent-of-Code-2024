use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::io::prelude::*;

fn main() {
    let mut robots = read_input();

    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;
    const TIME: i64 = 100;

    let safety = compute_safety_factor(&mut robots.clone(), WIDTH, HEIGHT, TIME, false);
    println!("Safety factor: {}", safety);

    println!("Starting part 2...");
    pause();
    compute_safety_factor(&mut robots, WIDTH, HEIGHT, 10000, true);
}

fn read_input() -> Vec<(i64, i64, i64, i64)> {
    let mut s = String::new();
    let _ = File::open("input14.txt")
        .expect("Could not open File.")
        .read_to_string(&mut s);

    let reg = Regex::new(r".*=(-?\d+),(-?\d+).*=(-?\d+),(-?\d+)").unwrap();

    s
        .trim()
        .lines()
        .map(|line| {
            let (_, [x, y, v1, v2]) = reg
                .captures(line)
                .expect("Could not capture")
                .extract();
            (x.parse::<i64>().expect("Could not parse x pos"),
             y.parse::<i64>().expect("Could not parse y pos"),
            v1.parse::<i64>().expect("Could not parse v1"),
            v2.parse::<i64>().expect("Coudl not parse v2"))
        }).collect()
}

fn compute_safety_factor(
    robots: &mut Vec<(i64, i64, i64, i64)>,
    width: i64, 
    height: i64,
    time: i64,
    stepwise: bool) -> i64 {


    let mut floor = vec![ vec![0; width as usize]; height as usize];

    for &(x, y, _, _) in robots.iter() { floor[y as usize][x as usize] += 1; }
    if stepwise { printfloor(&floor, 0); }

    let mut printat = 69; // saw pattern here resembling a tree

    for t in 0..time {
        for (x, y, v1, v2) in robots.iter_mut() {
            floor[*y as usize][*x as usize] -= 1;

            *x += *v1;
            *y += *v2;

            *x %= width as i64; 
            *y %= height as i64; 
            if *x < 0 { *x += width as i64; } 
            if *y < 0 { *y += height as i64; } 

            floor[*y as usize][*x as usize] += 1;
        }
        if stepwise && t == printat { printfloor(&floor, t+1); printat += 101; }
    }
    find_final_safety(&robots, width, height)
}

fn find_final_safety(
    robots: &Vec<(i64, i64, i64, i64)>,
    width: i64,
    height: i64) -> i64 {
    let mut q1: i64 = 0;
    let mut q2: i64 = 0;
    let mut q3: i64 = 0;
    let mut q4: i64 = 0;

    for &(x, y, _, _) in robots {
        if x < width/2 && y < height/2 { q1 += 1; }
        if x < width/2 && y > height/2 { q2 += 1; }
        if x > width/2 && y < height/2 { q3 += 1; }
        if x > width/2 && y > height/2 { q4 += 1; }
    }

    q1 * q2 * q3 * q4
}

fn printfloor(floor: &Vec<Vec<i32>>, time: i64) {
    println!("At t = {}:", time);
    for i in 0..floor.len() {
        for j in 0..floor[i].len() {
            if floor[i][j] == 0 { print!("   "); }
            else { print!("{}  ", std::cmp::min(floor[i][j],9)); }
        }
        println!();
    }
    pause();
}

// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/4
fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
