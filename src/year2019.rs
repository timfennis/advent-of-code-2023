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
use itertools::Itertools;
use std::process::exit;

pub fn run_program(state: &mut [i64], input: &[i64]) -> Vec<i64> {
    let mut pc = 0;
    let mut input = input.iter();
    let mut output: Vec<i64> = Default::default();

    let mut count = 0;
    // dbg!(state.iter().enumerate().collect_vec());

    loop {
        // println!("PC: {pc}");
        count += 1;

        let instruction = state[pc];

        let p_modes = [
            instruction / 100 % 10,
            instruction / 1000 % 10,
            instruction / 10000 % 10,
        ];

        match instruction % 100 {
            // 3 parameter instructions
            op @ 1 | op @ 2 | op @ 7 | op @ 8 => {
                assert!(p_modes[0] < 2 && p_modes[1] < 2 && p_modes[2] == 0);

                let p1 = if p_modes[0] == 0 {
                    state[state[pc + 1] as usize]
                } else {
                    state[pc + 1]
                };
                let p2 = if p_modes[1] == 0 {
                    state[state[pc + 2] as usize]
                } else {
                    state[pc + 2]
                };
                let p3 = state[pc + 3] as usize;

                match op {
                    // addition
                    1 => state[p3] = p1 + p2,
                    // multiplication
                    2 => state[p3] = p1 * p2,
                    // less than
                    7 => state[p3] = if p1 < p2 { 1 } else { 0 },
                    // equal
                    8 => state[p3] = if p1 == p2 { 1 } else { 0 },

                    _ => panic!("Invalid instruction '{op}' encountered"),
                }
                pc += 4;
            }
            // read from input
            3 => {
                // println!("READ");
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
                output.push(p1);

                // increase program counter
                pc += 2;
            }
            // 2 parameter jump instructions
            op @ 5 | op @ 6 => {
                let p1 = if p_modes[0] == 0 {
                    state[state[pc + 1] as usize]
                } else {
                    state[pc + 1]
                };

                let p2 = if p_modes[1] == 0 {
                    state[state[pc + 2] as usize]
                } else {
                    state[pc + 2]
                };

                if op == 5 && p1 != 0 || op == 6 && p1 == 0 {
                    pc = p2 as usize;
                } else {
                    pc += 3;
                }
            }

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

    macro_rules! assert_program_output {
        ($program:expr, $input:expr, $output:expr) => {
            let mut program = $program.clone();
            let output = run_program(&mut program, &$input);
            assert_eq!(output, $output);
        };
    }

    #[test]
    pub fn parameter_mode_multiplication() {
        let mut program = [1002i64, 4, 3, 4, 33];
        run_program(&mut program, Default::default());

        assert_eq!(program.to_vec(), vec![1002, 4, 3, 4, 99,])
    }

    #[test]
    pub fn day_5_part2_jump_test() {
        let pos_mode = [3i64, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_program_output!(pos_mode, [0], [0]);
        assert_program_output!(pos_mode, [1], [1]);
        assert_program_output!(pos_mode, [2], [1]);

        let im_mode = [3i64, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_program_output!(im_mode, [0], [0]);
        assert_program_output!(im_mode, [1], [1]);
        assert_program_output!(im_mode, [2], [1]);
    }
    #[test]
    pub fn day_5_part_2_test_program() {
        let program = [
            3i64, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98,
            0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20,
            4, 20, 1105, 1, 46, 98, 99,
        ];

        for i in 0..10 {
            assert_program_output!(
                program,
                [i],
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
    pub fn year_2019_day_2() {
        check(&mut year2019::Day2::default(), 4690667, 6255);
    }
}
