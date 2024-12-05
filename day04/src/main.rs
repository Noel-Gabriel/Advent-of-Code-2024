use std::fs::File;
use std::io::Read;

fn main() {
    let (m, n, word_search) = read_input();

    let xmas = count_xmas(&word_search, m, n);
    let mas  = count_mas(&word_search, m, n);

    println!("Number of xmas': {xmas}");
    println!("Number of xmas': {mas}");
}

fn read_input() -> (usize,usize, String) {
    let mut file = File::open("input04.txt").expect("Could not open file");

    let mut s = String::new();
    let _ = file.read_to_string(&mut s); 
    assert!(s.is_ascii());
    
    // count second dimension
    let n = s.chars().position(|c| c == '\n').expect("Could not find line break");
    
    // remove line breaks
    let s: String = s.chars().filter(|&c| c != '\n').collect();

    // count first dimension
    let m = s.chars().count() / n;

    (m, n, s)
}

fn count_xmas(s: &str, m: usize, n: usize) -> i32 {
    let mut ans = 0;
    let bytes = s.as_bytes();

    for i in 0..m {
        for j in 0..n {
            if bytes[i * n + j] == b'X' {
                ans += left_right(&bytes, i, j, n);
                ans += up_down(&bytes, i, j, m, n);  
                ans += diags(&bytes, i, j, m, n);
            }
        }
    }
    ans
}

fn left_right(b: &[u8], i: usize, j: usize, n:usize) -> i32 {
    let mut count = 0;
    let word = [b'X', b'M', b'A', b'S'];
    let mut found = true;

    for c in 0..word.len() {
        if j < c { found = false; break; }
        if b[i * n + (j - c)] != word[c] { found = false; break; }
    }

    if found { count += 1; } else { found = true; }

    for c in 0..word.len() {
        if j + c >= n { found = false; break; }
        if b[i * n + (j + c)] != word[c] { found = false; break; }
    }

    count + (found as i32)
}

fn up_down(b: &[u8], i: usize, j: usize, m: usize, n: usize) -> i32 {
    let mut count = 0;
    let word = [b'X', b'M', b'A', b'S'];
    let mut found = true;

    for c in 0..word.len() {
        if i < c { found = false; break; }
        if b[(i - c) * n + j] != word[c] { found = false; break; }
    }

    if found { count += 1; } else { found = true; }

    for c in 0..word.len() {
        if i + c >= m { found = false; break; }
        if b[(i + c) * n + j] != word[c] { found = false; break; }
    }

    count + (found as i32)
}

fn diags(b: &[u8], i: usize, j: usize, m: usize, n: usize) -> i32 {
    let mut count = 0;
    let word = [b'X', b'M', b'A', b'S'];
    let mut found = true;

    for c in 0..word.len() {
        if j < c || i < c { found = false; break; }
        if b[(i - c) * n + (j - c)] != word[c] { found = false; break; }
    }

    if found { count += 1; } else { found = true; }

    for c in 0..word.len() {
        if j + c >= n || i + c >= m { found = false; break; }
        if b[(i + c) * n + (j + c)] != word[c] { found = false; break; }
    }
    
    if found { count += 1; } else { found = true; }

    for c in 0..word.len() {
        if j < c || i + c >= m { found = false; break; }
        if b[(i + c) * n + (j - c)] != word[c] { found = false; break; }
    }

    if found { count += 1; } else { found = true; }

    for c in 0..word.len() {
        if j + c >= n || i < c  { found = false; break; }
        if b[(i - c) * n + (j + c)] != word[c] { found = false; break; }
    }

    count + (found as i32)
}

fn count_mas(s: &str, m: usize, n: usize) -> i32 {
    let mut ans = 0;
    let bytes = s.as_bytes();

    for i in 0..m {
        for j in 0..n {
            if bytes[i * n + j] == b'A' {
                ans += diags_mas(&bytes, i, j, m, n);
            }
        }
    }
    ans
}

fn diags_mas(b: &[u8], i: usize, j: usize, m: usize, n: usize) -> i32 {
    let mut count = 0;

    if i >= 1 && j >= 1 && i + 1 < m && j + 1 < n {
        count += (b[(i - 1) * n + (j - 1 )] == b'M' && b[(i + 1) * n + (j + 1 )] == b'S' ||
                    b[(i - 1) * n + (j - 1 )] == b'S' && b[(i + 1) * n + (j + 1 )] == b'M') as i32;
    }

    if i >= 1 && j >= 1 && i + 1 < m && j + 1 < n {
        count += (b[(i - 1) * n + (j + 1 )] == b'M' && b[(i + 1) * n + (j - 1 )] == b'S' ||
                    b[(i - 1) * n + (j + 1 )] == b'S' && b[(i + 1) * n + (j - 1 )] == b'M') as i32;
    }

    if count == 2 { 1 } else { 0 }
}


