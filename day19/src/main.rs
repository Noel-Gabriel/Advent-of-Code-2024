use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

mod trie;

fn main() {
    let (trie, patterns) = read_input();

    let (valid_patterns, total_valid) = find_patterns(&trie, &patterns);

    println!("Number of valid patterns: {}", valid_patterns);
    println!("Total Number of valid patterns: {}", total_valid);

}


fn read_input()  -> (trie::Trie, Vec<String>) {
    let mut s = String::new();
    let _ = File::open("input19.txt")
        .expect("Could not open File.")
        .read_to_string(&mut s);
    
    let (words, patterns) = s
        .split_once("\n\n")
        .expect("Could not find empty line.");

    let trie = trie::Trie::new(&words
        .split(",")
        .map(|s| {
            s
                .trim()
                .to_string()
        }).collect::<Vec<String>>());

    let ps: Vec<String> = patterns
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect();

    (trie, ps)
}

fn find_patterns(trie: &trie::Trie, patterns: &Vec<String>) -> (i64, i64) {
    let mut valids: i64 = 0;
    let mut total_valids: i64 = 0;
    let mut memo: HashMap<String, i64> = HashMap::new();
    for pattern in patterns {
        let mut count: i64 = 0;
        get_count(trie, &pattern, &mut count, &mut memo); 
        if count > 0 { 
            valids += 1;
            total_valids += count;
        }
    }
    (valids, total_valids)
}

fn get_count(root: &trie::Trie, pattern: &str, count: &mut i64, memo: &mut HashMap<String, i64>) { 
    if pattern.len() == 0 { *count += 1; return }
    if memo.contains_key(pattern) { *count += memo.get(pattern).unwrap(); return }
    let mut subtrie = root;
    for (i, c) in pattern.chars().enumerate() {
        if let Some(next) = subtrie.next(c) { 
            if next.found { 
                let current = *count;
                get_count(root, &pattern[i+1..], count, memo); 
                memo.insert(pattern[i+1..].to_string(),  *count - current);
            }
            subtrie = next;
        } else { return }
    }
}
