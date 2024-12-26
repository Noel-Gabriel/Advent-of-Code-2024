use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

mod gate;

fn main() {
    let (deps, gates, mut bits, id_to_wire) = read_input();

    let z_output = get_output(&deps, &gates, &mut bits, &id_to_wire);
    // found manually c:
    let mut sus_wires = vec!["qjj", "gjc", "z17", "wmp", "gvm", "z26", "z39", "qsb"]; 
    sus_wires.sort();

    println!("Output: {}", z_output);
    println!("Sus wired: {}", sus_wires.join(",").to_string());
}

fn get_output(
    deps: &Vec<Vec<usize>>, 
    gates: &HashMap<usize, gate::Gate>,
    bits: &mut HashMap<usize, i32>,
    idw: &HashMap<usize, String>) -> i64 {

    let mut topo_sorted = topo_sort(&deps, idw.len());
    topo_sorted.reverse();

    topo_sorted
        .iter()
        .for_each(|res_wire| {
            let gate = gates.get(&res_wire).unwrap();
            let bit =  gate.evaluate(&bits); 

            bits
                .entry(*res_wire)
                .and_modify(|curr| *curr = bit)
                .or_insert(bit);
        });

    let mut outputs = idw
        .iter()
        .filter(|(_ , wire)| wire.starts_with("z"))
        .map(|(id, wire)| (*id, wire.to_string()))
        .collect::<Vec<(usize, String)>>();

    outputs.sort_by(|(_, wire1), (_, wire2)| wire1.cmp(wire2) ); 

    let mut z: i64 = 0;
    for i in 0..outputs.len() {
        let id = outputs[i].0;
        let bit = *bits.get(&id).unwrap();
        z |= (bit as i64) << i;
    }
    z
}

fn topo_sort(deps: &Vec<Vec<usize>>, num_wires: usize) -> Vec<usize> {
    let mut order = Vec::new();
    let mut visited = vec![false; num_wires];
    for wire in 0..deps.len() {
        if !visited[wire] {
            dfs(wire, &deps, &mut order, &mut visited);
        }
    }
    order
}

fn dfs(wire: usize, deps: &Vec<Vec<usize>>, order: &mut Vec<usize>, visited: &mut Vec<bool>) {
    visited[wire] = true;
    for &end in deps[wire].iter() {
        if !visited[end] {
            dfs(end, deps, order, visited);
        }
    }
    order.push(wire);
}

fn read_input() 
    -> (Vec<Vec<usize>>, 
        HashMap<usize, gate::Gate>,
        HashMap<usize, i32>,
        HashMap<usize, String>) {

    let mut s = String::new();
    let _ = File::open("input24.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    let (initial_bits, gates) = s
        .split_once("\n\n")
        .expect("Could not find empty line");

    let mut id: usize = 0;
    let mut wires_to_id: HashMap<String, usize> = HashMap::new();
    let mut wires_to_bit: HashMap<usize, i32> = HashMap::new();
    let mut res_to_gate: HashMap<usize, gate::Gate> = HashMap::new();

    parse_initial_bits(&mut id, &mut wires_to_id, &mut wires_to_bit, &mut res_to_gate, &initial_bits);
    let mut dependencies = parse_gates(&mut id, &mut wires_to_id, &mut res_to_gate, &gates);

    let reverse = wires_to_id
        .iter()
        .fold(HashMap::new(), |mut id_to_wire, (wire, id)| {
            id_to_wire.insert(*id, wire.clone());
            id_to_wire
        });

    if dependencies.len() <= reverse.len() { 
        dependencies.resize(reverse.len(), Vec::new());
    }

    (dependencies, res_to_gate, wires_to_bit, reverse)
}   

fn parse_initial_bits(
    id: &mut usize, 
    wid: &mut HashMap<String, usize>, 
    wbt: &mut HashMap<usize, i32>,
    res_to_gate: &mut HashMap<usize, gate::Gate>,
    bits: &str) {

    bits
        .trim()
        .lines()
        .for_each(|line| {
            let mut wire_it = line.split_whitespace();
            let wire    = wire_it.next().unwrap().to_string();
            let bit     = wire_it.next().unwrap().to_string();
            wid.insert(wire[..wire.len()-1].to_string(), *id);
            wbt.insert(*id, bit.parse::<i32>().unwrap());
            res_to_gate.insert(*id, gate::Gate::new(*id, *id, "ID"));
            *id += 1;
        });
}
    
fn parse_gates(
    id: &mut usize, 
    wid: &mut HashMap<String, usize>, 
    res_to_gate: &mut HashMap<usize, gate::Gate>,
    gates: &str) -> Vec<Vec<usize>> {

    let mut deps = Vec::new();

    gates
        .trim()
        .lines()
        .for_each(|line| {
            let mut gate_it = line.split_whitespace();
            let (lhs, op, rhs, _, res) = (
                gate_it.next().unwrap().to_string(), // lhs
                gate_it.next().unwrap().to_string(), // op
                gate_it.next().unwrap().to_string(), // rhs
                gate_it.next(),                      // ->
                gate_it.next().unwrap().to_string(), // res
            );

            for wire in [&lhs, &rhs, &res] {
                if !wid.contains_key(wire) {
                    wid.insert(wire.clone(), *id);
                    *id += 1;
                }
            }

            let lhs_id = *wid.get(&lhs).unwrap();
            let rhs_id = *wid.get(&rhs).unwrap();
            let res_id = *wid.get(&res).unwrap();

            for operand in [lhs_id, rhs_id] {
                if deps.len() <= operand {
                    deps.resize(operand + 1, Vec::new());
                }
                deps[operand].push(res_id);
            }

            res_to_gate.insert(res_id, gate::Gate::new(lhs_id, rhs_id, &op));
        });

    deps
}
