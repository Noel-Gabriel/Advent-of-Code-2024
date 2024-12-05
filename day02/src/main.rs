use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    let values = read_input();
    
    let safe_reports = count_safe(&values);
    let safe_reports_dampened = count_safe_dampened(&values); 

    println!("Safe reports: {safe_reports}");
    println!("Safe reports (using dampener): {safe_reports_dampened}");
}

fn read_input() -> Vec<Vec<i32>> {
    let file = File::open("input02.txt").expect("Could not open file");

    let lines = io::BufReader::new(file).lines().flatten();

    let mut values: Vec<Vec<i32>> = vec![];
    for line in lines {
        let row: Vec<i32> = line.split_whitespace()
                                .map(|s| s.parse::<i32>().expect("Could not map string to integer"))
                                .collect();
        values.push(row);
    };
    values
}

fn count_safe(values: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for row in values {
       if is_safe(&row) { ans += 1; } 
    }
    ans
}

fn count_safe_dampened(values: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for row in values {
        if is_safe(row) { ans += 1; continue; }
        for i in 0..row.len() {
            let cand: Vec<i32> = row.iter()
                                    .enumerate()
                                    .filter(|&(j, _)| j != i)
                                    .map(|(_, &v)| v)
                                    .collect();
            if is_safe(&cand) { ans += 1; break; }
        }
    }
    ans
}

fn is_safe(row: &Vec<i32>) -> bool {         
    if row.len() == 1 { return false; }
    let sig = (row[1] - row[0]).signum();
    if sig == 0 { return false; }
    for i in 1..row.len() {
        let diff = row[i] - row[i-1];
        if diff.signum() != sig || diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }
    true
}

