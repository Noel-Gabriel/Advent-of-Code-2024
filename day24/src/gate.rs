use std::collections::HashMap;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum Op {
    XOR,
    AND,
    OR,
    ID,
}

impl Op {
    pub fn evaluate(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            Op::XOR => lhs ^ rhs,
            Op::OR  => lhs | rhs,
            Op::AND => lhs & rhs,
            Op::ID  => lhs,
        }
    }

    pub fn from(op: &str) -> Self {
        match op {
            "XOR" => Op::XOR,
            "AND" => Op::AND,
            "OR"  => Op::OR,
            "ID"  => Op::ID,
            _     => unreachable!(),
        }
    }
}

#[derive(Hash, Debug, Clone)]
pub struct Gate {
    pub lhs: usize,
    pub rhs: usize,
    pub op : Op,
}

impl  Gate {
    pub fn new(lhs: usize, rhs: usize, op: &str) -> Self {
        Self {
            lhs,
            rhs,
            op: Op::from(op),
        }
    }

    pub fn evaluate(&self, bits: &HashMap<usize, i32>) -> i32{
        let &lhs_bit = bits.get(&self.lhs).unwrap();
        let &rhs_bit = bits.get(&self.rhs).unwrap();
        self.op.evaluate(lhs_bit, rhs_bit)
    }
}
