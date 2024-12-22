use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let codes = read_input();
    let mapping = map();

    const KEYPAD_DEPTH: u32 = 25;
    let complexities = get_complexities(&codes, &mapping, KEYPAD_DEPTH);

    println!("Sum of complexities: {}", complexities);
}

fn read_input() -> Vec<Vec<char>> {
    let mut s = String::new();
    let _ = File::open("input21.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    s
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}


fn map() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        (' ', (3, 0)),
        ('0', (3, 1)),
        ('A', (3, 2)),
        ('^', (3, 1)),
        ('<', (4, 0)),
        ('v', (4, 1)),
        ('>', (4, 2)),
    ])
}


fn get_complexities(codes: &Vec<Vec<char>>, mapping: &HashMap<char, (i32, i32)>, keypad_depth: u32) -> u64 {
    let mut instructions = Vec::new();
    for code in codes { 
        let mut memo: HashMap<(i32, i32, i32, i32, u32), usize> = HashMap::new();
        instructions.push(traverse(&code, &mapping, keypad_depth, *mapping.get(&'A').unwrap(), &mut memo));
    }
    instructions
        .iter()
        .enumerate()
        .fold(0u64, |comp, (i, len)| {
            let num = codes[i]
                .iter()
                .rev()
                .filter(|&c| *c != 'A')
                .fold((0u64, 1), |(mut n, mut base), c| {
                    n += c.to_digit(10).unwrap() as u64 * base;
                    base *= 10;
                    (n, base)
                }).0 ;
            comp + (*len as u64 * num)
        })
}

fn traverse(code: &Vec<char>, mapping: &HashMap<char, (i32, i32)>, depth: u32, (mut sx, mut sy): (i32, i32),
    memo: &mut HashMap<(i32, i32, i32, i32, u32), usize>) -> usize {
    let gap = *mapping.get(&' ').unwrap();
    let mut current_instruction: Vec<char> = Vec::new();
    let mut len: usize = 0;
    for i in 0..code.len() {
        let (nx, ny) = *mapping.get(&code[i]).unwrap();

        if let Some(min_len) = memo.get(&(sx, sy, nx, ny, depth)) {
            len += min_len;
        } else { 
            let mut paths: Vec<Vec<char>> = Vec::new();
            get_all((sx, sy), (nx, ny), gap, &mut std::usize::MAX.clone(), &mut paths, &mut Vec::new());

            if depth == 0 {
                let min_path = &mut paths
                    .iter_mut()
                    .min_by_key(|path| path.len())
                    .unwrap();

                current_instruction.append(&mut min_path.clone()); 
                memo.insert((sx, sy, nx, ny, depth), min_path.len());

            } else {
                let min_path = &mut paths
                    .iter_mut()
                    .map(|path| traverse(path, mapping, depth - 1, *mapping.get(&'A').unwrap(), memo))
                    .min()
                    .unwrap();

                len += *min_path;
                memo.insert((sx, sy, nx, ny, depth), *min_path);
            }
        }
        (sx, sy) = (nx, ny);
    }
    current_instruction.len() + len
}

fn get_all(
    (x, y): (i32, i32),
    (ex, ey): (i32, i32),
    gap: (i32, i32), 
    curr_len: &mut usize,
    cand: &mut Vec<Vec<char>>, 
    curr: &mut Vec<char>) {

    if *curr_len < curr.len() { return }
    if (x, y) == (ex, ey) {
        curr.push('A');
        if *curr_len > curr.len() {
            cand.clear();
            *curr_len = curr.len();
        }
        cand.push(curr.clone()); 
        curr.pop();
        return 
    }
    if (x, y) == gap { return }

    match (x - ex).signum() {
        1 => {
            for _ in 0..(x-ex).abs() { curr.push('^'); }
            get_all((x - (x-ex), y), (ex, ey), gap, curr_len, cand, curr);
            for _ in 0..(x-ex).abs() { curr.pop(); }
        }
        -1 => {
            for _ in 0..(x-ex).abs() { curr.push('v'); }
            get_all((x + (ex-x), y), (ex, ey), gap, curr_len, cand, curr);
            for _ in 0..(x-ex).abs() { curr.pop(); }
        }
        _ => (),
    }

    match (y - ey).signum() {
        1 => {
            for _ in 0..(y-ey).abs() { curr.push('<'); }
            get_all((x, y - (y-ey)), (ex, ey), gap, curr_len, cand, curr);
            for _ in 0..(y-ey).abs() { curr.pop(); }
        }
        -1 => {
            for _ in 0..(y-ey).abs() { curr.push('>'); }
            get_all((x, y + (ey-y)), (ex, ey), gap, curr_len, cand, curr);
            for _ in 0..(y-ey).abs() { curr.pop(); }
        }
        _ => (),
    }
}


/* NOT NECESSARY FOR PROBLEM, MAPS ROBOT MOVEMENT TO KEYPAD CODE
 
fn reverse(instructions: &Vec<char>, mapping: &HashMap<char, (i32, i32)>, depth: u32) -> Vec<char> {
    let mut pos = vec![mapping.get(&'A').unwrap().clone(); depth as usize + 1];
    let mut code = Vec::new();
    for &i in instructions {
        if let Some(c) = mv(i, mapping, depth, &mut pos) {
            code.push(c);
        }
    }
    code
}

fn map_dir(i: char) -> (i32, i32) {
    match i {
        '^' => (-1, 0),
        '<' => (0, -1),
        'v' => (1,  0),
        '>' => (0,  1),
        _   => panic!("Unreachable at char {}", i),
    }
}

fn get_key(pos: (i32, i32), mapping: &HashMap<char, (i32, i32)>, human: bool) -> char {
    for (&c, &map) in mapping.iter() {
        if map == pos {
            if !human && !c.is_digit(10) { return c }
            else if human && (c == 'A' || c.is_digit(10)) { return c }
        }
    }
    panic!("Char not found at pos {:?}", pos);
}

fn mv(i: char, mapping: &HashMap<char, (i32, i32)>, depth: u32, pos: &mut Vec<(i32, i32)>) -> Option<char> {
    if i == 'A' {
        if depth == 0 {
            return Some(get_key(pos[depth as usize], mapping, true))
        } 
        return mv(get_key(pos[depth as usize], mapping, false), mapping, depth - 1, pos);
    }
    let (x, y) = &mut pos[depth as usize];
    let (dx, dy) = map_dir(i);
    *x += dx;
    *y += dy;
    return None
}*/
