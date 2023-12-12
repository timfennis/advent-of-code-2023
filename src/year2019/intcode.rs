#[derive(Debug)]
pub struct IntCodeComputer {
    mem: Vec<i64>,
    rel: usize,
    pc: usize,
    halted: bool,
}

#[allow(dead_code)]
type Program = Vec<i64>;

impl IntCodeComputer {
    pub fn halted(&self) -> bool {
        self.halted
    }
    pub fn read(&self, addr: usize) -> i64 {
        if let Some(&val) = self.mem.get(addr) {
            val
        } else {
            0
        }
    }

    pub fn write(&mut self, addr: usize, val: i64) {
        // for now just resize our memory to fit the value
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = val;
    }

    fn write_bool(&mut self, addr: usize, val: bool) {
        self.write(addr, if val { 1 } else { 0 })
    }
    pub fn run_with_input_until_halt(&mut self, input: Vec<i64>) -> Vec<i64> {
        let mut input = input.into_iter();
        let mut out = Vec::new();

        while !self.halted {
            let output = if self.mem[self.pc] == 3 {
                if let Some(next) = input.next() {
                    self.step(Some(next))
                } else {
                    return out;
                }
            } else {
                self.step(None)
            };

            if let Some(output) = output {
                out.push(output)
            }
        }

        out
    }
    pub fn run_until_halt(&mut self, input: Vec<i64>) -> Vec<i64> {
        let mut input = input.into_iter();
        let mut out = Vec::new();

        while !self.halted {
            let output = if self.mem[self.pc] % 100 == 3 {
                self.step(input.next())
            } else {
                self.step(None)
            };

            if let Some(output) = output {
                out.push(output);
            }
        }
        out
    }

    /// Returns a tuple of the RAW value of the parameter and the mode it's in
    fn param(&self, position: usize) -> (i64, i64) {
        let instruction = self.mem[self.pc];
        let p_modes = [
            instruction / 100 % 10,
            instruction / 1000 % 10,
            instruction / 10000 % 10,
        ];

        (self.read(self.pc + position), p_modes[position - 1])
    }

    fn param_addr(&self, position: usize) -> usize {
        match self.param(position) {
            (addr, 0) => addr as usize,
            // in immediate mode to get the address of a parameter we just return the program counter + the offset
            (_, 1) => self.pc + position,
            // in relative mode we need to add the param value to the rel register
            (offset, 2) => ((self.rel as i64) + offset) as usize,
            _ => unreachable!("invalid parameter mode"),
        }
    }
    fn param_value(&self, position: usize) -> i64 {
        self.read(self.param_addr(position))
    }
    fn step(&mut self, input: Option<i64>) -> Option<i64> {
        let instruction = self.mem[self.pc];

        match instruction % 100 {
            // 3 parameter instructions
            op @ 1 | op @ 2 | op @ 7 | op @ 8 => {
                let p1 = self.param_value(1);
                let p2 = self.param_value(2);

                let p3 = self.param_addr(3);

                match op {
                    // addition
                    1 => self.write(p3, p1 + p2),
                    // multiplication
                    2 => self.write(p3, p1 * p2),
                    // less than
                    7 => self.write_bool(p3, p1 < p2),
                    // equal
                    8 => self.write_bool(p3, p1 == p2),

                    _ => panic!("Invalid instruction '{op}' encountered"),
                }
                self.pc += 4;
            }
            // read from input
            3 => {
                self.write(
                    self.param_addr(1),
                    input.expect("must have an input to run this instruction"),
                );
                self.pc += 2;
            }
            // write to output
            4 => {
                // figure out the value of p1 my checking the mode
                let p1 = self.param_value(1);

                self.pc += 2;
                return Some(p1);
            }
            // 2 parameter jump instructions
            op @ 5 | op @ 6 => {
                let p1 = self.param_value(1);

                let p2 = self.param_value(2) as usize;

                if op == 5 && p1 != 0 || op == 6 && p1 == 0 {
                    assert!(p2 < self.mem.len());
                    self.pc = p2;
                } else {
                    self.pc += 3;
                }
            }
            // adjust the relative base
            9 => {
                let p1 = self.param_value(1);
                self.rel = (self.rel as i64 + p1) as usize;
                self.pc += 2;
            }

            99 => self.halted = true,
            _ => panic!("Invalid instruction '{instruction}' encountered"),
        }

        None
    }
}

impl From<&[i64]> for IntCodeComputer {
    fn from(value: &[i64]) -> Self {
        Self {
            mem: value.to_vec(),
            pc: 0,
            rel: 0,
            halted: false,
        }
    }
}
impl From<&Vec<i64>> for IntCodeComputer {
    fn from(value: &Vec<i64>) -> Self {
        Self {
            mem: value.clone(),
            pc: 0,
            rel: 0,
            halted: false,
        }
    }
}
impl From<Vec<i64>> for IntCodeComputer {
    fn from(value: Vec<i64>) -> Self {
        Self {
            mem: value,
            pc: 0,
            rel: 0,
            halted: false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::year2019::intcode::{IntCodeComputer, Program};

    macro_rules! assert_program_output {
        ($program:expr, $input:expr, $output:expr) => {
            let mut computer = IntCodeComputer::from($program);
            let output = computer.run_until_halt($input);
            assert_eq!(output, $output);
        };
    }

    #[test]
    pub fn parameter_mode_multiplication() {
        let mut computer = IntCodeComputer::from(vec![1002i64, 4, 3, 4, 33]);
        computer.run_until_halt(Vec::new());

        assert_eq!(computer.mem, vec![1002, 4, 3, 4, 99,])
    }

    #[test]
    pub fn day_5_part2_jump_test() {
        let pos_mode = &vec![3i64, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_program_output!(pos_mode, vec![0], vec![0]);
        assert_program_output!(pos_mode, vec![1], vec![1]);
        assert_program_output!(pos_mode, vec![2], vec![1]);

        let im_mode = &vec![3i64, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_program_output!(im_mode, vec![0], vec![0]);
        assert_program_output!(im_mode, vec![1], vec![1]);
        assert_program_output!(im_mode, vec![2], vec![1]);
    }
    #[test]
    pub fn day_5_part_2_test_program() {
        let program = vec![
            3i64, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98,
            0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20,
            4, 20, 1105, 1, 46, 98, 99,
        ];

        for i in 0..10 {
            assert_program_output!(
                program.clone(),
                vec![i],
                match i {
                    i if i < 8 => [999],
                    i if i == 8 => [1000],
                    i if i > 8 => [1001],
                    _ => unreachable!("number must always match"),
                }
            );
        }
    }

    #[test]
    pub fn day9_part_1_test_program() {
        let program: Program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_program_output!(program.clone(), vec![], program);
    }
}
