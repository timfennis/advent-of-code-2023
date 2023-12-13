#![allow(unused_imports)]

pub use day1::*;
pub use day10::*;
pub use day11::*;
pub use day12::*;
pub use day13::*;
pub use day2::*;
pub use day3::*;
pub use day4::*;
pub use day5::*;
pub use day6::*;
pub use day7::*;
pub use day8::*;
pub use day9::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[cfg(test)]
mod test {
    use crate::test::check;
    use crate::year2023;

    #[test]
    pub fn year_2023_day_1() {
        check(&mut year2023::Day1::default(), 55002, 55093);
    }

    #[test]
    pub fn year_2023_day_2() {
        check(&mut year2023::Day2::default(), 2879, 65122);
    }

    #[test]
    pub fn year_2023_day_3() {
        check(&mut year2023::Day3::default(), 537832, 81939900);
    }

    #[test]
    pub fn year_2023_day_4() {
        check(&mut year2023::Day4::default(), 20117, 13768818);
    }

    #[test]
    pub fn year_2023_day_5() {
        check(&mut year2023::Day5::default(), 510109797, 9622622);
    }

    #[test]
    pub fn year_2023_day_6() {
        check(&mut year2023::Day6::default(), 4403592, 38017587);
    }

    #[test]
    pub fn year_2023_day_7() {
        check(&mut year2023::Day7::default(), 250946742, 251824095);
    }
    #[test]
    pub fn year_2023_day_8() {
        check(&mut year2023::Day8::default(), 17141, 10818234074807u64);
    }
    #[test]
    pub fn year_2023_day_9() {
        check(&mut year2023::Day9::default(), 1861775706, 1082);
    }
    #[test]
    pub fn year_2023_day_10() {
        check(&mut year2023::Day10::default(), 6897, 367);
    }

    #[test]
    pub fn year_2023_day_11() {
        check(&mut year2023::Day11::default(), 9918828, 692506533832u64);
    }
    #[test]
    pub fn year_2023_day_12() {
        check(&mut year2023::Day12::default(), 7007, 3476169006222u64);
    }
    #[test]
    pub fn year_2023_day_13() {
        check(&mut year2023::Day13::default(), 28651, 25450);
    }
}
