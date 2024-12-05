use std::fs::File;
use regex::Regex;
use std::io::Read;

fn main() {
    let memory = read_input();

    let mults = compute_non_corrupt(&memory);
    let mults_instructions = compute_non_corrupt_with_ins(&memory);

    println!("Multiplication result: {mults}");
    println!("Multiplication result (instructions): {mults_instructions}");
}

fn read_input() -> String {
    let mut file = File::open("input03.txt").expect("Could not open file.");

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn compute_non_corrupt(memory: &String) -> i64 {
    let mut ans: i64 = 0;

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Error in defining Regex.");
    
    for (_, [p1, p2]) in re.captures_iter(&memory).map(|c| c.extract()) {
        ans += p1.parse::<i64>().unwrap() * p2.parse::<i64>().unwrap();
    }
    ans
}

fn compute_non_corrupt_with_ins(memory: &String) -> i64 {
    let mut ans: i64 = 0;

    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(don\'t\(\))|(do\(\))").expect("Error in defining Regex.");
    
    let mut track: bool = true;
    for capture in re.captures_iter(&memory) {
        if capture.get(1).is_some() {
            if !track { continue; }
            ans += &capture[2].parse::<i64>().unwrap() * &capture[3].parse::<i64>().unwrap();
        } else if capture.get(4).is_some() {
            track = false;
        } else {
            track = true;
        }
    }
    ans
}



