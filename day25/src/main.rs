use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let (keys, locks) = read_input();

    let pairs = find_pairs(&keys, &locks);

    println!("Unique key/lock pairs: {}", pairs);
}

fn read_input() -> (HashSet<Vec<u32>>, HashSet<Vec<u32>>) {
    let mut s = String::new();
    let _ = File::open("input25.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    let mut keys = HashSet::new();
    let mut locks = HashSet::new();

    s
        .trim()
        .split("\n\n")
        .for_each(|schematic| {
            let mut translated = vec![0u32; schematic.find("\n").unwrap()];
            schematic
                .lines()
                .fold(&mut translated, |translated, line| {
                    line
                        .trim()
                        .chars()
                        .enumerate()
                        .for_each(|(i, c)| { if c == '#' { translated[i] += 1; } });
                    translated
                });

            translated.iter_mut().for_each(|v| *v -= 1);

            if schematic.starts_with("#") {
                locks.insert(translated);
            } else {
                keys.insert(translated);
            }
        });

    (keys, locks)
}

fn find_pairs(keys: &HashSet<Vec<u32>>, locks: &HashSet<Vec<u32>>) -> u32 {
    const LOCK_WIDTH: u32 = 5;
    let mut pairs: u32 = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.iter().zip(lock.iter()).all(|(k, l)| k + l <= LOCK_WIDTH) {
                pairs += 1;
            }
        }
    }
    pairs
}

