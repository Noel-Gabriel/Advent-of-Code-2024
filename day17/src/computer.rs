use std::fmt;

pub struct Computer {
    pub reg_a: i64,
    pub reg_b: i64,
    pub reg_c: i64,

    pub memory: Vec<i32>,
    pub pc: usize,
    pub output: Vec<i64>,
}

pub fn load_computer(a: i64, b: i64, c: i64, memory: &Vec<i32>) -> Computer {
    Computer {
        reg_a: a,
        reg_b: b,
        reg_c: c,

        memory: memory.clone(),
        pc:0,
        output: Vec::new(),
    }
}

impl Computer {

    pub fn execute(&mut self) -> String {
        while self.is_executing() {
            self.execute_next();
        }
        self.output()
    }

    pub fn output(&self) -> String { 
        self.output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn is_executing(&self) -> bool { self.pc < self.memory.len() }

    fn execute_next(&mut self) {
        if !self.is_executing() { return () }
        match self.memory[self.pc] {
            0 => { 
                self.reg_a >>= self.operand(false);
                self.pc += 2;
            }
            1 => {
                self.reg_b ^= self.operand(true);
                self.pc += 2;
            }
            2 => {
                self.reg_b = self.operand(false) % 8;
                self.pc += 2;
            }
            3 => {
                if self.reg_a != 0 { self.pc = self.operand(true) as usize; } else { self.pc += 2; } 
            }
            4 => {
                self.reg_b ^= self.reg_c;
                self.pc += 2;
            }
            5 => {
                self.output.push(self.operand(false) % 8);
                self.pc += 2;
            }
            6 => {
                self.reg_b = self.reg_a >> self.operand(false);
                self.pc += 2;
            }
            7 => {
                self.reg_c = self.reg_a >> self.operand(false);
                self.pc += 2;
            }
            _ => unreachable!(),
        }
    }

    fn operand(&self, literal: bool) -> i64 {
        match self.memory[self.pc + 1] {
            i if literal => i as i64,
            4            => self.reg_a,
            5            => self.reg_b,
            6            => self.reg_c,
            i if 0 <= i && i <= 3 => i as i64,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Computer {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Register A: {}, Register B: {}, Register C: {} \nPC: {} \
            \nMemory: {:?}\nOutput: {:?}", 
            self.reg_a, self.reg_b, self.reg_c, self.pc, self.memory, self.output)
    }
} 
