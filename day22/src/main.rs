use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};

fn main() {
    let secrets = read_input();

    const ITERATIONS: u32 = 2000;

    let mut prices = Vec::new(); 
    let sum_secrets = secrets
        .iter()
        .fold(0u64, |sum, &secret| {
            let (secret, ps) = simulate(secret, ITERATIONS);
            prices.push(ps);
            sum + secret
        });
    
    let bananas = maximize_bananas(&prices);

    println!("Sum of first 2000 generated prices: {}", sum_secrets);
    println!("Max number of bananas that can be sold: {}", bananas);
}

fn read_input() -> Vec<u32> {
    let mut s = String::new();
    let _ = File::open("input22.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    s
        .trim()
        .lines()
        .map(|line| line.parse::<u32>().expect("Could not parse line"))
        .collect()
}

fn simulate(mut secret: u32, iterations: u32) -> (u64, Vec<u8>) {
    let mut prices = Vec::from([(secret % 10) as u8]);
    const MOD: u32 = 16777216;
    for _ in 0..iterations {
        secret ^= secret << 6;
        secret %= MOD; 
        secret ^= secret >> 5;
        secret %= MOD;
        secret ^= secret << 11;
        secret %= MOD;
        prices.push((secret % 10) as u8);
    }
    (secret as u64, prices)
}

fn maximize_bananas(all_prices: &Vec<Vec<u8>>) -> u32 {
    let mut candidate: HashMap<(i32, i32, i32, i32), u32> = HashMap::new();
    let mut seen: HashSet<((i32, i32, i32, i32), usize)> = HashSet::new();
    for (i, prices) in all_prices.iter().enumerate() {
        for e in 4..prices.len() {
            let seq = diffs(&prices, e);
            if seen.contains(&(seq, i)) { continue; }
            candidate
                .entry(seq)
                .and_modify(|p| *p += prices[e] as u32)
                .or_insert(prices[e] as u32);
            seen.insert((seq, i));
        }
    }

    *candidate
        .values()
        .max()
        .unwrap()
}

fn diffs(prices: &Vec<u8>, end: usize) -> (i32, i32, i32, i32) {
    (prices[end] as i32 - prices[end - 1] as i32,
    prices[end - 1] as i32 - prices[end - 2] as i32,
    prices[end - 2] as i32 - prices[end - 3] as i32,
    prices[end - 3] as i32 - prices[end - 4] as i32)
}
