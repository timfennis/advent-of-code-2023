#![allow(unused_imports)]

pub mod day1;
pub mod day2;

pub mod day3;
mod day4;
mod day5;

pub use day1::*;
pub use day2::*;
pub use day3::*;
pub use day4::*;
pub use day5::*;

pub fn run_program(state: &mut [i64], input: &[i64]) -> Vec<i64> {
    let mut pc = 0;
    let mut input = input.iter();
    let mut output: Vec<i64> = Default::default();

    loop {
        let instruction = state[pc];

        match instruction % 100 {
            op @ 1 | op @ 2 => {
                let p1_mode = instruction / 100 % 10;
                let p2_mode = instruction / 1000 % 10;
                let p3_mode = instruction / 10000 % 10;
                assert!(p1_mode < 2);
                assert!(p2_mode < 2);
                assert_eq!(p3_mode, 0);

                let p1 = if p1_mode == 0 {
                    state[state[pc + 1] as usize]
                } else {
                    state[pc + 1]
                };
                let p2 = if p2_mode == 0 {
                    state[state[pc + 2] as usize]
                } else {
                    state[pc + 2]
                };
                let p3 = state[pc + 3] as usize;

                match op {
                    // addition
                    1 => {
                        // println!("add[{}] {} + {} -> {}", state[pc], state[pc + 1], state[pc + 2], state[pc + 3]);
                        state[p3] = p1 + p2;
                    }
                    // multiplication
                    2 => {
                        // println!("mul[{}] {} * {} -> {}", state[pc], state[pc + 1], state[pc + 2], state[pc + 3]);
                        state[p3] = p1 * p2;
                    }

                    _ => panic!("Invalid instruction '{op}' encountered"),
                }
                pc += 4;
            }
            // read from input
            3 => {
                // println!("read from input");
                let p1 = state[pc + 1] as usize;
                pc += 2;
                state[p1] = *input.next().expect("must have another input value");
            }
            // write to output
            4 => {
                let p1_mode = instruction / 100 % 10;
                assert!(p1_mode < 2);

                // figure out the value of p1 my checking the mode
                let p1 = if p1_mode == 0 {
                    state[state[pc + 1] as usize]
                } else {
                    state[pc + 1]
                };
                // println!("{instruction},{} -> {:#?}", state[pc + 1], output);
                // println!("write {}", p1);
                output.push(p1);

                // increase program counter
                pc += 2;
            }

            // Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
            // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
            // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            99 => {
                return output;
            }
            _ => panic!("Invalid instruction '{instruction}' encountered"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test::check;
    use crate::year2019::run_program;
    use crate::{year2019, year2023};

    #[test]
    pub fn parameter_mode_multiplication() {
        let mut program: [i64; 5] = [1002, 4, 3, 4, 33];
        run_program(&mut program, Default::default());

        assert_eq!(program.to_vec(), vec![1002, 4, 3, 4, 99,])
    }

    #[test]
    pub fn year_2019_day_2() {
        check(&mut year2019::Day2::default(), 4690667, 6255);
    }
}
