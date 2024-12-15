use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use itertools::Itertools; 

fn main() {
    let (antennas, m, n) = read_input();

    let antinodes = find_antinodes(&antennas, m, n, false); 
    let antinodes_resonant = find_antinodes(&antennas, m, n, true); 

    println!("Antinodes: {}", antinodes);
    println!("Antinodes (Resonant): {}", antinodes_resonant);
}

fn read_input() -> (HashMap<char, Vec<(i32, i32)>>, usize, usize) {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut s = String::new();
    let _ = File::open("input08.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    let m: usize = s.lines().count();
    let n: usize = s.lines().into_iter().count();
    
    s.lines()
        .enumerate()
        .for_each(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .for_each(|(j, c)| {
                    antennas
                        .entry(c)
                        .or_default()
                        .push( (i as i32, j as i32) );
                })
        });

    (antennas, m, n) 
}

fn is_valid((x, y): (i32, i32), m: usize, n: usize) -> bool {
    0 <= x && x < m as i32 && 0 <= y && y < n as i32
}

fn find_antinodes(antennas: &HashMap<char, Vec<(i32, i32)>>, m: usize, n: usize, resonant: bool) -> i32 {
    let mut included = vec![ vec![false; n]; m];
    antennas.iter()
            .fold(0, |mut acc, (_, vec)| {
                (0..vec.len())
                    .cartesian_product(0..vec.len())
                    .filter(|(i, j)| i < j)
                    .for_each(|(i, j)| {
                        let (x1, y1) = vec[i];
                        let (x2, y2) = vec[j];
                        let (dx, dy) = (x1 - x2, y1 - y2);
                        let mut sign = 1;
                        for (x, y) in [(x1, y1), (x2, y2)] {
                            if !included[x as usize][y as usize] && resonant { 
                                included[x as usize][y as usize] = true;
                                acc += 1; 
                            }
                            let (mut u, mut v) = (x + sign * dx, y + sign * dy); 
                            while is_valid((u, v), m, n) {
                                if !included[u as usize][v as usize] {
                                    acc += 1;
                                    included[u as usize][v as usize] = true;
                                }  
                                if !resonant { break; }
                                (u, v) = (u + sign * dx, v + sign * dy); 
                            }
                            sign *= -1;
                        }
                    });
                acc
            })
}

