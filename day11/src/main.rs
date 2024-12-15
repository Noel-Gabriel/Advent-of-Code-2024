use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let stones = read_input(); 

    let blinks = 25;
    let blinks2 = 75;

    let num_stones = blink(&mut stones.clone(), blinks);
    let num_stones2 = blink(&mut stones.clone(), blinks2);

    println!("Number of stones after {} blinks: {}", blinks, num_stones);
    println!("Number of stones after {} blinks: {}", blinks2, num_stones2);
}

fn read_input() -> HashMap<u64, u64> {
    let mut s = String::new();
    let _ = File::open("input11.txt")
        .expect("Could not open file")
        .read_to_string(&mut s);

    let mut hs: HashMap<u64, u64> = HashMap::new();

    s
        .trim()
        .split_whitespace()
        .for_each(|n| {
            hs.entry(n
                .parse::<u64>()
                .expect("Could not parse digit"))
              .and_modify(|occ| *occ += 1)
              .or_insert(1);
            
        });
    hs
}

fn num_len(mut stone: u64) -> usize {
    if stone == 0 { return 1 }
    let mut ans = 0;
    while stone > 0 {
        stone /= 10;
        ans += 1;  
    }
    ans
}

fn handle_even(mut stone: u64, occ: u64, len: usize, next_stones: &mut HashMap<u64, u64>) {
    let target_split = len as u64 / 2;
    let mut right = 0;
    let mut base = 1;
    for _ in 0..target_split {
        right += (stone % 10) * base;
        stone /= 10;
        base *= 10;
    }

    for s in [stone, right] {
        next_stones
            .entry(s)
            .and_modify(|occ_| *occ_ += occ)
            .or_insert(occ);
    }
}

fn handle_odd(stone: u64, occ:u64, next_stones: &mut HashMap<u64, u64>) {
    match stone {
        0 => {
            next_stones
                .entry(1)
                .and_modify(|occ_ones| *occ_ones += occ)
                .or_insert(occ);
        }
        s => {
            next_stones
                .entry(s * 2024)
                .and_modify(|occ_odd| *occ_odd += occ)
                .or_insert(occ);
        }
    }
}

fn blink(stones: &mut HashMap<u64, u64>, blinks: i32) -> u64 {
    for _ in 0..blinks {
        let mut next_stones: HashMap<u64, u64> = HashMap::new();
        for (&stone, occ) in stones.iter() {
            let len = num_len(stone);
            match len % 2 {
                0 => handle_even(stone, *occ, len, &mut next_stones),
                1 => handle_odd(stone, *occ, &mut next_stones),
                _ => unreachable!()
            }
        }
        *stones = next_stones;
    }
    let mut ans = 0;
    for (_, occ) in stones {
        ans += *occ;
    }
    ans
}
