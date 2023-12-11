#![allow(unused_imports)]

mod day1;
mod day2;

mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod intcode;

pub use day1::*;
pub use day2::*;
pub use day3::*;
pub use day4::*;
pub use day5::*;
pub use day6::*;
pub use day7::*;
pub use day8::*;
pub use day9::*;

use crate::year2019::intcode::IntCodeComputer;
use itertools::Itertools;
use std::process::exit;

fn run_program(program: &[i64], input: Vec<i64>) -> Vec<i64> {
    let mut computer = IntCodeComputer::from(program);
    computer.run_until_halt(input)
}

#[cfg(test)]
mod test {
    use crate::test::check;
    use crate::year2019::{run_program, IntCodeComputer};
    use crate::{year2019, year2023};
    #[test]
    pub fn year_2019_day_1() {
        check(&mut year2019::Day1::default(), 3295424, 4940279);
    }

    #[test]
    pub fn year_2019_day_2() {
        check(&mut year2019::Day2::default(), 4690667, 6255);
    }
    #[test]
    pub fn year_2019_day_3() {
        check(&mut year2019::Day3::default(), 1674, 14012);
    }
    #[test]
    pub fn year_2019_day_4() {
        check(&mut year2019::Day4::default(), 2220, 1515);
    }

    #[test]
    pub fn year_2019_day_5() {
        check(&mut year2019::Day5::default(), 6761139, 9217546);
    }
    #[test]
    pub fn year_2019_day_6() {
        check(&mut year2019::Day6::default(), 204521, 307);
    }
    #[test]
    pub fn year_2019_day_7() {
        check(&mut year2019::Day7::default(), 199988, 17519904);
    }

    #[test]
    pub fn year_2019_day_8() {
        check(&mut year2019::Day8::default(), 1690, "ZPZUB");
    }
}
