use std::io::Read;
use std::fs::File;
use std::collections::{HashSet, HashMap};

fn main() {
    let (ordering, reports) = read_input(); 

    let order_hm = build_hs(&ordering);

    let correct_order: i32 = reports
        .iter()
        .filter(|report| is_ordered(&report, &order_hm))
        .fold(0, |acc, report| {
            acc + report[report.len() / 2]
        });

    let after_correcting_order: i32 = reports
        .iter()
        .filter(|report| !is_ordered(&report, &order_hm))
        .fold(0, |acc, report| {
            acc + middle_page_after_correcting_order(&report, &order_hm)
        });

    println!("Sum of middle pages of correct ordering: {correct_order}");
    println!("Sum of middle pages after correcting ordering: {after_correcting_order}");
}

fn read_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut file = File::open("input05.txt").expect("Could not open file");

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);
    
    let lines: Vec<&str> = s.split('\n').collect();

    let split_i = lines.iter().position(|l| l.len() == 0).expect("Could not find empty line");

    // map "x|y" to (x, y) as i32
    let ordering: Vec<(i32, i32)> = lines[0..split_i]
        .iter()
        .map(|l| {
            let mut pair = l.trim().split('|');
            let x = pair.next().expect("No next in iter").parse::<i32>().expect("Could not parse i32");
            let y = pair.next().expect("No next in iter 2").parse::<i32>().expect("Could not parse i32 2");
            (x, y)
        })
        .collect();

    // map "x, y, z, .." to Vec<i32> = [x, y, z, ... ]
    let reports: Vec<Vec<i32>> = lines[split_i+1..lines.len()-1]
        .iter()
        .map(|l| {
            l.trim().split(',').map(|v| v.parse::<i32>().expect("Could not parse report")).collect()
        }).collect();

    (ordering, reports)
}

fn build_hs(ordering: &Vec<(i32, i32)>) -> HashMap<i32, HashSet<i32>> {
    let mut hm: HashMap<i32, HashSet<i32>> = HashMap::new();

    for pair in ordering {
        hm.entry(pair.0).and_modify(|hs| { hs.insert(pair.1); } ).or_insert( HashSet::from([pair.1]) );
    }

    hm
}

fn is_ordered(report: &Vec<i32>, order_hm: &HashMap<i32, HashSet<i32>>) -> bool {
    for (i, page) in report.iter().enumerate() {
        for j in &report[0..i] {
            if order_hm[page].contains(&j) { return false };
        }
    }
    true
}

// The middle page is the page where report.len() / 2 number of pages are expected to come after
// it, because the problem assumes that there exists only one unique ordering.
fn middle_page_after_correcting_order(report: &Vec<i32>, order_hm: &HashMap<i32, HashSet<i32>>) -> i32 {
    let target: i32 = report.len() as i32 / 2;

    for (i, page1) in report.iter().enumerate() {
        let mut count: i32 = 0;
        for (j, page2) in report.iter().enumerate() {
            if i == j { continue; }
            if order_hm[page1].contains(&page2) { count += 1; }
        }
        if count == target { return *page1 }
    }
    panic!("No middle page found.");
}
