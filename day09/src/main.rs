use std::fs::File;
use std::io::Read;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let (mut files, mut spaces) = read_input();

    let checksum = compute_checksum(files.clone(), spaces.clone());
    let checksum_blocks = compute_checksum_blocks(&mut files, &mut spaces);

    println!("Checksum: {}", checksum);
    println!("Checksum moving blocks: {}", checksum_blocks);
}

fn read_input() -> (Vec<i32>, Vec<i32>) {

    let mut diskmap = String::new();

    let _ = File::open("input.txt")
        .expect("Could not open file")
        .read_to_string(&mut diskmap);

    diskmap
        .trim()
        .chars()
        .enumerate()
        .fold((Vec::new(), Vec::new()), |(mut files, mut spaces), (i, c)| {
            let d: i32 = (c as u32 - '0' as u32) as i32;
            if i % 2 == 0 { files.push(d); } else { spaces.push(d); }
            (files, spaces)
        })
}

// pos * a + (pos + 1) * a + (pos + 2) * a + ... + (pos + n) * a
// pos * a + pos * a + a + pos * a + 2a + ... + pos * a + na
// n * pos * a + (1 + 2 + ... + n) * a
// (n + 1) * pos * a + n * (n + 1) / 2 * a
fn compute_checksum(mut files: Vec<i32>, mut spaces: Vec<i32>) -> i64 {
    let mut checksum: i64 = 0;
    let mut s: usize = 0;
    let mut e: usize = files.len() - 1;
    let mut space: usize = 0;
    let mut pos: usize = 0;
    while s <= e {
        checksum += (files[s] as usize * pos * s + ((files[s] * (files[s] - 1)) / 2) as usize * s) as i64;
        pos += files[s] as usize;
        s += 1;
        while s <= e && spaces[space] != 0 {
            if files[e] == 0 { e -= 1; if e < s { break; } }
            let diff = cmp::min(files[e], spaces[space]);
            checksum += (diff as usize * pos * e + ((diff * (diff - 1)) / 2) as usize * e) as i64;
            files[e] -= diff;
            spaces[space] -= diff;
            pos += diff as usize;
        } 
        space += 1;
    }
    checksum
}


fn fill_spaces(files: &mut Vec<i32>, spaces: &mut Vec<i32>) -> HashMap<usize, Vec<(i32, usize)>> {
    let mut filled: HashMap<usize, Vec<(i32, usize)>> = HashMap::new();
    for e in (0..files.len()).rev() {
        for (i, space) in spaces.iter_mut().enumerate() {
            if i >= e { break; } 
            if *space == 0 { continue; }
            if files[e] <= *space {
                *space -= files[e];
                filled
                    .entry(i)
                    .and_modify(|v| v.push((files[e], e)))
                    .or_insert(vec![(files[e], e)]);
                files[e] *= -1;
                break;
            }
        }
    }
    filled
}
    

fn compute_checksum_blocks(files: &mut Vec<i32>, spaces: &mut Vec<i32>) -> i64 {
    let mut checksum: i64 = 0;
    let mut pos: usize = 0;
    let filled = fill_spaces(files, spaces);
    for s in 0..files.len() {
        if files[s] < 0 {
            pos += (files[s] * (-1)) as usize;
        } else {
            checksum += (files[s] as usize * pos * s + ((files[s] * (files[s] - 1)) / 2) as usize * s) as i64;
            pos += files[s] as usize;
        }
        if s < spaces.len() {
            if filled.contains_key(&s) {
                for &(len, val) in filled.get(&s).unwrap() {
                    checksum += (len as usize * pos * val + ((len * (len - 1)) / 2) as usize * val) as i64;
                    pos += len as usize;
                }
            }
            pos += spaces[s] as usize;
        }
    }
    checksum
}

