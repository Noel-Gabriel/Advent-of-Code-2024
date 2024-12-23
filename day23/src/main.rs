use std::fs::File;
use std::io::Read;
use std::collections::{HashSet, HashMap};

fn main() {
    let (pairs, mapping) = read_input();
    let (ts, pw) = get_ts_and_pw(&pairs, &mapping);

    println!("Number of 3-Groups with ts: {}", ts);
    println!("Password: {}", pw);
}

fn read_input() -> (Vec<HashSet<usize>>, HashMap<usize, String>) {
    let mut s = String::new();
    let _ = File::open("input23.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    let mut stou: HashMap<String, usize> = HashMap::new();
    let mut utos: HashMap<usize, String> = HashMap::new();
    let mut graph: Vec<HashSet<usize>> = Vec::new();

    s
        .trim()
        .lines()
        .map(|line| {
            line
                .split_once("-")
                .unwrap()
        }).for_each(|(c1, c2)| {
            let c1 = c1.to_string();
            let c2 = c2.to_string();

            for str in [&c1, &c2] {
                if !stou.contains_key(str) {
                    stou.insert(str.clone(), graph.len());
                    utos.insert(graph.len(), str.clone());
                    graph.push(HashSet::new());
                }
            }

            let (&u, &v) = (stou.get(&c1).unwrap(), stou.get(&c2).unwrap());
            graph[u].insert(v);
            graph[v].insert(u);
        });

    (graph, utos)
}

// Idea: Cycle (a, b, c) and (a, b, d) -> cycle (a, b, c, d) if (c, d) are a cycle
fn get_ts_and_pw(pairs: &Vec<HashSet<usize>>, mapping:&HashMap<usize, String>) -> (u32, String) {
    let mut cycles = (0..pairs.len())
        .map(|v| Vec::from([v]))
        .collect::<Vec<Vec<usize>>>(); 

    let mut len3cycles = Vec::new();
    let longest = loop {
        let mut next: HashSet<Vec<usize>> = HashSet::new();
        for i in 0..cycles.len() {
            for j in i+1..cycles.len() {
                if cycles[i].len() > 1 && 
                    cycles[i][cycles[i].len()-2] != cycles[j][cycles[j].len()-2] { break; }

                if let Some(joined) = join(&cycles[i], &cycles[j], &pairs) {
                    next.insert(joined);
                }
            }
        }
        // save cycles of len 3 
        if cycles[0].len() == 3 { len3cycles = cycles.clone(); }
        cycles = next.into_iter().collect(); 
        if cycles.len() == 1 { break &cycles[0] }
        cycles.sort();
    };

    // part 1
    let ts = len3cycles
        .iter()
        .filter(|v| v.iter().any(|n| mapping.get(&n).unwrap().starts_with("t")))
        .count() as u32;

    // part 2
    let mut pw = longest
        .iter()
        .map(|n| mapping.get(&n).unwrap().to_string())
        .collect::<Vec<String>>();

    pw.sort();

    (ts, pw.join(","))
}

fn join(c1: &Vec<usize>, c2: &Vec<usize>, pairs: &Vec<HashSet<usize>>) -> Option<Vec<usize>> {
    let (&u, &v) = (c1.last().unwrap(), c2.last().unwrap());
    if !pairs[u].contains(&v) { return None }

    let mut joined = c1.clone();
    joined.pop();

    if u < v {
        joined.push(u);
        joined.push(v);
    } else {
        joined.push(v);
        joined.push(u);
    }
    return Some(joined)
}
