use std::fs::File;
use std::io::Read;

mod computer;

fn main() {
    let ((a, b, c), memory) = read_input();

    let mut comp = computer::load_computer(a, b, c, &memory);
    println!("Output: {}", comp.execute());

    let mut init_a: i64 = 0;
    let _ = find(&mut init_a, &memory);

    println!("Initial Register A configuration: {}", init_a);

    let mut comp = computer::load_computer(init_a, 0, 0, &memory);
    println!("Output with initial Register A configuration: {}", comp.execute());
}

fn read_input() -> ((i64, i64, i64), Vec<i32>) {
    let mut s = String::new();
    let _ = File::open("input17.txt")
        .expect("Could not open File.")
        .read_to_string(&mut s);

    let (register, program) = s
        .split_once("\n\n")
        .expect("Could not split into reg and prog");

    let (a, b, c) = register
        .trim()
        .lines()
        .enumerate()
        .fold((0i64, 0i64, 0i64), |(a, b, c), (i, line)| {
            let val: i64 = (&line[line.find(|c: char| c.is_ascii_digit())
                .expect("Could not find integer value in register")..])
                .parse::<i64>()
                .expect("Could not parse integer value");
            match i {
                0 => (val, b, c),
                1 => (a, val, c),
                2 => (a, b, val),
                _ => unreachable!(),
            }
        });

    let prog_as_vec = (&program[program.find(|c: char| c.is_ascii_digit())
        .expect("Could not find program begin")..])
        .trim()
        .split(",")
        .map(|st| {
            st.parse::<i32>().expect("Could not parse program")
        })
        .collect();

    ((a, b, c), prog_as_vec)
}

fn find(a: &mut i64, memory: &Vec<i32>) -> bool {
    for a_cand in 0..8 {
        let mut comp = computer::load_computer((*a << 3) | a_cand, 0, 0, &memory);
        let _ = comp.execute();

        if comp.output.len() <= memory.len() {
            if comp.output.iter().rev().zip(memory.iter().rev()).all(|(&c, &m)| c as i32 == m) {
                *a = (*a << 3) | a_cand;
                if comp.output.len() == memory.len() { return true }
                let found = find(a, memory);
                if found { return true }
                *a >>= 3;
            }
        } else {
            return false 
        }
    }
    false
}
