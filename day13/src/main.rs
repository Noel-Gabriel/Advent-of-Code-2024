use std::io::Read;
use std::fs::File;
use regex::Regex;

fn main() {
    let values: Vec<Vec<i64>> = read_input();

    let cost = compute_cost(&values, false);
    let true_cost = compute_cost(&values, true);

    println!("Total cost: {}", cost);
    println!("Total true cost: {}", true_cost);
}

fn read_input() -> Vec<Vec<i64>> {
    let mut s = String::new();
    let _ = File::open("input13.txt")
        .expect("Could not open File")
        .read_to_string(&mut s);

    let button_regex = Regex::new(r".*X\+(\d+).*Y\+(\d+)").unwrap();
    let price_regex = Regex::new(r".*X=(\d+).*Y=(\d+)").unwrap();


    s
        .lines()
        .filter(|line| line.len() > 0)
        .enumerate()
        .fold(Vec::new(), |mut vec, (i, line)| {
            match i % 3 {
                2 => {
                    let (_, [x, y]) = price_regex.captures(line).unwrap().extract();
                    for dist in [x, y] {
                        vec.last_mut().unwrap().push(dist.parse::<i64>().expect("Could not parse to i32"));
                    }
                }
                n => { 
                    if n == 0 { vec.push(Vec::new()); }
                    let (_, [x, y]) = button_regex.captures(line).unwrap().extract();
                    for dist in [x, y] {
                        vec.last_mut().unwrap().push(dist.parse::<i64>().expect("Could not parse to i32"));
                    }
                }
            }
            vec
        })
}

fn determinant(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    x1 * y2 - x2 * y1
}

fn compute_cost(values: &Vec<Vec<i64>>,  true_cost: bool) -> i64 {
    let mut total_cost: i64 = 0;
    for v in values {
        let x1 = v[0];
        let y1 = v[1];
        let x2 = v[2];
        let y2 = v[3];
        let p1 = v[4] + if true_cost { 10000000000000 } else { 0 };
        let p2 = v[5] + if true_cost { 10000000000000 } else { 0 };

        let det = determinant(x1, y1, x2, y2);
        if det == 0 { panic!("Shit, no cramer's rule."); }
        let det_a = determinant(p1, x2, p2, y2);
        let det_b = determinant(x1, p1, y1, p2);

        if det_a % det != 0 || det_b % det != 0 { continue; }
        let a_presses = det_a / det;
        let b_presses = det_b / det ;

        if !true_cost && (a_presses > 100 || b_presses > 100) { continue; }

        total_cost += 3 * det_a / det + det_b / det;
    }
    total_cost
}
