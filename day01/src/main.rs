use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (mut first_row, mut second_row) = read_rows();

    // sort
    first_row.sort();
    second_row.sort();

    let differences = find_diffs(&first_row, &second_row);
    let similarity  = find_sims(&first_row, &second_row);

    println!("Difference score: {}", differences);
    println!("Similarity score: {}", similarity);

    Ok(())
}

fn read_rows() -> (Vec<i32>, Vec<i32>) {
    
    // open and read file
    let file = File::open("input01.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut first_row:  Vec<i32> = vec![];
    let mut second_row: Vec<i32> = vec![];

    // save rows
    for line in lines.flatten() {
        let rows: Vec<&str> = line.split("   ").collect();
        assert!(rows.len() == 2);

        first_row.push(rows[0].parse::<i32>().unwrap());
        second_row.push(rows[1].parse::<i32>().unwrap());
    }

    assert!(first_row.len() == second_row.len());
    (first_row, second_row)
}

fn find_diffs(first_row: &Vec<i32>, second_row: &Vec<i32>) -> i32 {
    let mut ans: i32 = 0;

    // find differences and add to total
    for i in 0..first_row.len() {
        ans += (first_row[i] - second_row[i]).abs();
    }
    ans
}

fn find_sims(first_row: &Vec<i32>, second_row: &Vec<i32>) -> i64 {
    let mut ans: i64 = 0;
    let mut freqs: HashMap<i32, i32> = HashMap::new();

    // count occurences
    for r in second_row.iter() {
        freqs.entry(*r).and_modify(|val| *val += 1).or_insert(1);
    }

    for r in first_row.iter() {
        match freqs.get(&*r) {
            Some(f) => ans += i64::from(*r * f),
            None => (),
        }
    }
    ans
}
