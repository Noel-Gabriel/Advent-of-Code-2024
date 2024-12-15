use std::fs::File;
use std::io::Read;

fn main() {
    let inputs = read_input();

    let (sum, sum_concat) = inputs
        .into_iter()
        .fold((0, 0), |(a1, a2), v| {
            if search(&v, v[1], 2, false) { return (a1 + v[0], a2 + v[0]) } 
            else if search(&v, v[1], 2, true) { return (a1, a2 + v[0]) }
            (a1, a2)
        });

    println!("Sum of possible equations: {}", sum);
    println!("Sum of possible equations with concat: {}", sum_concat);
}

fn read_input() -> Vec<Vec<u64>> {
    let mut file = File::open("input07.txt").expect("Could not open file");

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    s.split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.trim_end_matches(':'))
                .map(|w| w.parse::<u64>()
                          .expect("Could not parse word.")) 
                .collect()
        }).filter(|line : &Vec<u64>| line.len() > 0)
          .collect()
}

fn search(input: &Vec<u64>, result: u64,  index: usize, using_concat: bool) -> bool {
    if index == input.len() { return result == input[0] }

    search(input, result + input[index], index + 1, using_concat) ||
        search(input, result * input[index], index + 1, using_concat) ||
            using_concat && search(input, concat(result, input[index]), index + 1, using_concat)
}

fn concat(a: u64, b: u64) -> u64 {
    let (shift, of) = a.overflowing_mul(10i32.pow(b.to_string().len() as u32) as u64);
    if of { panic!("Overflowed") }
    shift + b
}
